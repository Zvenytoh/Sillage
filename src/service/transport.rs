use super::{Cancellation, ServiceError};
use serde_json::Value;
use std::io::Read;
use std::time::Duration;
use url::Url;

const WEB_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/140.0.0.0 Safari/537.36";
const PLAYER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 15_7_3) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/26.0 Safari/605.1.15";

pub fn audio_url(raw: &str) -> Result<Url, ServiceError> {
    let url = Url::parse(raw).map_err(|_| ServiceError::DestinationRejected)?;
    if url.scheme() != "https"
        || url.port_or_known_default() != Some(443)
        || !url.username().is_empty()
        || url.password().is_some()
        || url.fragment().is_some()
        || !url.domain().is_some_and(|host| {
            host.strip_suffix(".googlevideo.com").is_some_and(|prefix| {
                !prefix.is_empty()
                    && prefix.split('.').all(|label| {
                        !label.is_empty()
                            && label.len() <= 63
                            && !label.starts_with('-')
                            && !label.ends_with('-')
                            && label
                                .bytes()
                                .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
                    })
            })
        })
    {
        return Err(ServiceError::DestinationRejected);
    }
    Ok(url)
}

#[cfg(test)]
mod tests {
    use super::audio_url;
    #[test]
    fn audio_destinations_use_dns_boundaries_and_https() {
        assert!(
            audio_url("https://r1---sn-example.googlevideo.com/videoplayback?opaque=value").is_ok()
        );
        for url in [
            "http://r1.googlevideo.com/a",
            "https://evilgooglevideo.com/a",
            "https://r1.googlevideo.com.evil.test/a",
            "https://127.0.0.1/a",
            "https://user:password@r1.googlevideo.com/a",
            "https://r1.googlevideo.com:444/a",
            "https://.googlevideo.com/a",
        ] {
            assert!(audio_url(url).is_err());
        }
    }
}

#[derive(Clone)]
pub(super) struct Transport {
    agent: ureq::Agent,
}

impl Default for Transport {
    fn default() -> Self {
        Self {
            agent: ureq::Agent::new_with_config(
                ureq::Agent::config_builder()
                    .https_only(true)
                    .max_redirects(0)
                    .http_status_as_error(false)
                    .timeout_global(Some(Duration::from_secs(15)))
                    .max_idle_connections(2)
                    .build(),
            ),
        }
    }
}

fn status(code: u16) -> Result<(), ServiceError> {
    match code {
        200 | 206 => Ok(()),
        429 => Err(ServiceError::RateLimited),
        _ => Err(ServiceError::Network),
    }
}

fn bytes(
    mut reader: impl Read,
    limit: usize,
    cancel: &Cancellation,
) -> Result<Vec<u8>, ServiceError> {
    let mut data = Vec::new();
    let mut buffer = [0_u8; 16_384];
    loop {
        cancel.check()?;
        let count = reader
            .read(&mut buffer)
            .map_err(|_| ServiceError::Network)?;
        if count == 0 {
            break;
        }
        if data.len() + count > limit {
            return Err(ServiceError::InvalidResponse);
        }
        data.extend_from_slice(&buffer[..count]);
    }
    cancel.check()?;
    Ok(data)
}

impl Transport {
    pub fn json(
        &self,
        endpoint: &'static str,
        body: &Value,
        client: &str,
        version: &str,
        visitor: Option<&str>,
        cancel: &Cancellation,
    ) -> Result<Value, ServiceError> {
        cancel.check()?;
        let url = Url::parse(endpoint).map_err(|_| ServiceError::DestinationRejected)?;
        if url.scheme() != "https"
            || !matches!(url.domain(), Some("music.youtube.com" | "www.youtube.com"))
        {
            return Err(ServiceError::DestinationRejected);
        }
        let mut request = self
            .agent
            .post(endpoint)
            .header("Origin", "https://music.youtube.com")
            .header(
                "User-Agent",
                if client == "101" {
                    PLAYER_AGENT
                } else {
                    WEB_AGENT
                },
            )
            .header("X-YouTube-Client-Name", client)
            .header("X-YouTube-Client-Version", version)
            .header("Accept-Encoding", "identity");
        if let Some(visitor) = visitor {
            request = request.header("X-Goog-Visitor-Id", visitor);
        }
        let response = request.send_json(body).map_err(|_| ServiceError::Network)?;
        if response.status().as_u16() != 200 {
            status(response.status().as_u16())?;
            return Err(ServiceError::InvalidResponse);
        }
        let data = bytes(response.into_body().into_reader(), 4 * 1024 * 1024, cancel)?;
        serde_json::from_slice(&data).map_err(|_| ServiceError::InvalidResponse)
    }

    pub fn range(
        &self,
        initial: &Url,
        from: u64,
        to: u64,
        length: u64,
        cancel: &Cancellation,
    ) -> Result<Vec<u8>, ServiceError> {
        let mut url = audio_url(initial.as_str())?;
        for redirect in 0..=3 {
            cancel.check()?;
            let response = self
                .agent
                .get(url.as_str())
                .header("User-Agent", PLAYER_AGENT)
                .header("Accept-Encoding", "identity")
                .header("Range", format!("bytes={from}-{to}"))
                .call()
                .map_err(|_| ServiceError::Network)?;
            if response.status().is_redirection() {
                if redirect == 3 {
                    return Err(ServiceError::DestinationRejected);
                }
                let location = response
                    .headers()
                    .get("Location")
                    .and_then(|value| value.to_str().ok())
                    .ok_or(ServiceError::DestinationRejected)?;
                url = audio_url(
                    url.join(location)
                        .map_err(|_| ServiceError::DestinationRejected)?
                        .as_str(),
                )?;
                continue;
            }
            status(response.status().as_u16())?;
            if response.status().as_u16() != 206 {
                return Err(ServiceError::InvalidResponse);
            }
            let range = response
                .headers()
                .get("Content-Range")
                .and_then(|value| value.to_str().ok())
                .ok_or(ServiceError::InvalidResponse)?;
            let (bounds, total) = range
                .strip_prefix("bytes ")
                .and_then(|value| value.split_once('/'))
                .ok_or(ServiceError::InvalidResponse)?;
            let (start, end) = bounds
                .split_once('-')
                .ok_or(ServiceError::InvalidResponse)?;
            let parsed = |value: &str| {
                value
                    .parse::<u64>()
                    .map_err(|_| ServiceError::InvalidResponse)
            };
            let end = parsed(end)?;
            if parsed(start)? != from || parsed(total)? != length || end < from || end > to {
                return Err(ServiceError::InvalidResponse);
            }
            let data = bytes(response.into_body().into_reader(), 1024 * 1024, cancel)?;
            if data.len() as u64 != end - from + 1 {
                return Err(ServiceError::InvalidResponse);
            }
            return Ok(data);
        }
        Err(ServiceError::DestinationRejected)
    }
}
