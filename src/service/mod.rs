pub mod parse;
mod stream;
mod transport;

use crate::model::Track;
use serde_json::{Value, json};
use std::fmt;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
pub use stream::StreamInput;
pub fn valid_artwork_url(raw: &str) -> bool {
    transport::artwork_url(raw).is_ok()
}

#[derive(Clone)]
pub struct Cancellation(Arc<AtomicBool>);

impl Default for Cancellation {
    fn default() -> Self {
        Self(Arc::new(AtomicBool::new(false)))
    }
}

impl Cancellation {
    pub fn cancel(&self) {
        self.0.store(true, Ordering::Release);
    }
    pub fn check(&self) -> Result<(), ServiceError> {
        if self.0.load(Ordering::Acquire) {
            Err(ServiceError::Cancelled)
        } else {
            Ok(())
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ServiceError {
    Cancelled,
    Network,
    RateLimited,
    Unavailable,
    InvalidResponse,
    DestinationRejected,
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Cancelled => "Requête annulée.",
            Self::Network => {
                "YouTube Music est inaccessible. Vérifiez votre connexion et réessayez."
            }
            Self::RateLimited => "YouTube Music limite les requêtes. Patientez avant de réessayer.",
            Self::Unavailable => {
                "Aucun flux public compatible pour ce titre. Essayez un autre résultat."
            }
            Self::InvalidResponse => {
                "La réponse de YouTube Music est incompatible. Réessayez plus tard."
            }
            Self::DestinationRejected => "La destination réseau reçue ne peut pas être utilisée.",
        })
    }
}
impl std::error::Error for ServiceError {}

#[derive(Clone, Default)]
pub struct Client {
    transport: transport::Transport,
}

pub struct ResolvedStream {
    url: url::Url,
    length: u64,
    duration: Option<Duration>,
    thumbnail: Option<String>,
}

impl ResolvedStream {
    pub fn duration(&self) -> Option<Duration> {
        self.duration
    }
    pub fn thumbnail(&self) -> Option<&str> {
        self.thumbnail.as_deref()
    }
}

impl Client {
    fn browse(&self, id: &str, cancel: &Cancellation) -> Result<Value, ServiceError> {
        self.transport.json("https://music.youtube.com/youtubei/v1/browse?prettyPrint=false", &json!({
            "context":{"client":{"clientName":"WEB_REMIX","clientVersion":"1.20260707.12.00","hl":"fr","gl":"FR"}},
            "browseId":id
        }),"67","1.20260707.12.00",None,cancel)
    }

    pub fn home(
        &self,
        country: &str,
        cancel: &Cancellation,
    ) -> Result<crate::model::HomePage, ServiceError> {
        if country != "FR" {
            return Err(ServiceError::InvalidResponse);
        }
        let mut page = crate::model::HomePage {
            country: "FR".into(),
            fetched_at_unix_ms: crate::model::now_ms(),
            ..Default::default()
        };
        let charts = self
            .browse("FEmusic_charts", cancel)
            .and_then(|value| parse::chart_playlist(&value).ok_or(ServiceError::InvalidResponse))
            .and_then(|id| self.browse(&id, cancel))
            .and_then(|value| parse::search(&value));
        match charts {
            Err(ServiceError::RateLimited) => return Err(ServiceError::RateLimited),
            Ok(tracks) if !tracks.is_empty() => {
                page.trending = tracks.into_iter().take(20).collect();
                page.source = "YouTube Music · Les 20 clips tendance France".into();
            }
            _ => {
                page.warning = Some(
                    "Les tendances sont indisponibles. Réessayez dans quelques instants.".into(),
                )
            }
        }
        cancel.check()?;
        match self.browse("FEmusic_explore", cancel) {
            Ok(explore) => {
                page.discovery = parse::section(&explore, "Nouveaux clips musicaux")
                    .into_iter()
                    .take(24)
                    .collect()
            }
            Err(ServiceError::RateLimited) => return Err(ServiceError::RateLimited),
            Err(_) => {}
        }
        cancel.check()?;
        if page.trending.is_empty() && page.discovery.is_empty() {
            return Err(ServiceError::Network);
        }
        Ok(page)
    }
    pub fn artwork(&self, url: &str, cancel: &Cancellation) -> Result<Vec<u8>, ServiceError> {
        self.transport.artwork(url, cancel)
    }

    pub fn search(&self, query: &str, cancel: &Cancellation) -> Result<Vec<Track>, ServiceError> {
        if query.trim().is_empty() || query.chars().count() > 120 {
            return Err(ServiceError::InvalidResponse);
        }
        let response = self.transport.json("https://music.youtube.com/youtubei/v1/search?prettyPrint=false", &json!({
            "context": {"client": {"clientName": "WEB_REMIX", "clientVersion": "1.20260707.12.00", "hl": "fr", "gl": "FR"}},
            "query": query.trim()
        }), "67", "1.20260707.12.00", None, cancel)?;
        parse::search(&response)
    }

    pub fn resolve(&self, id: &str, cancel: &Cancellation) -> Result<ResolvedStream, ServiceError> {
        if !parse::valid_id(id) {
            return Err(ServiceError::InvalidResponse);
        }
        let visitor = self.transport.json("https://www.youtube.com/youtubei/v1/visitor_id?prettyPrint=false", &json!({
            "context": {"client": {"clientName": "WEB", "clientVersion": "2.20260707.00.00", "hl": "fr", "gl": "FR"}}
        }), "1", "2.20260707.00.00", None, cancel)?;
        let visitor = visitor
            .pointer("/responseContext/visitorData")
            .and_then(Value::as_str)
            .filter(|value| value.len() <= 4096)
            .ok_or(ServiceError::InvalidResponse)?;
        let response = self.transport.json("https://www.youtube.com/youtubei/v1/player?prettyPrint=false", &json!({
            "context": {"client": {"clientName": "VISIONOS", "clientVersion": "1.02", "deviceMake": "Apple", "deviceModel": "RealityDevice17,1", "osName": "visionOS", "osVersion": "26.5.23O471", "visitorData": visitor, "hl": "fr", "gl": "FR"}},
            "videoId": id, "contentCheckOk": true, "racyCheckOk": true
        }), "101", "1.02", Some(visitor), cancel)?;
        if response
            .pointer("/playabilityStatus/status")
            .and_then(Value::as_str)
            != Some("OK")
        {
            return Err(ServiceError::Unavailable);
        }
        let format = response
            .pointer("/streamingData/adaptiveFormats")
            .and_then(Value::as_array)
            .and_then(|formats| {
                formats
                    .iter()
                    .find(|format| format["itag"].as_u64() == Some(140))
            })
            .ok_or(ServiceError::Unavailable)?;
        if !format["mimeType"]
            .as_str()
            .is_some_and(|mime| mime.starts_with("audio/mp4"))
        {
            return Err(ServiceError::Unavailable);
        }
        let url = transport::audio_url(format["url"].as_str().ok_or(ServiceError::Unavailable)?)?;
        let length = format["contentLength"]
            .as_str()
            .and_then(|value| value.parse::<u64>().ok())
            .filter(|value| *value > 0 && *value <= 1 << 40)
            .ok_or(ServiceError::InvalidResponse)?;
        let duration = format["approxDurationMs"]
            .as_str()
            .and_then(|value| value.parse().ok())
            .map(Duration::from_millis);
        Ok(ResolvedStream {
            url,
            length,
            duration,
            thumbnail: response
                .pointer("/videoDetails/thumbnail/thumbnails")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
                .filter(|item| {
                    item["url"]
                        .as_str()
                        .is_some_and(|url| url.len() <= 4096 && valid_artwork_url(url))
                })
                .min_by_key(|item| item["width"].as_u64().unwrap_or(0).abs_diff(512))
                .and_then(|item| item["url"].as_str())
                .map(str::to_owned),
        })
    }

    pub fn input(&self, stream: ResolvedStream, cancel: Cancellation) -> StreamInput {
        StreamInput::new(self.transport.clone(), stream, cancel)
    }
}
