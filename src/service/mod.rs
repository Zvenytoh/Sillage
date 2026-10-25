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
            Self::Cancelled => "Request cancelled.",
            Self::Network => {
                "YouTube Music could not be reached. Check your connection and try again."
            }
            Self::RateLimited => "YouTube Music is limiting requests. Wait before trying again.",
            Self::Unavailable => {
                "This track has no supported public audio stream. Try another result."
            }
            Self::InvalidResponse => {
                "YouTube Music returned an unsupported response. Try again later."
            }
            Self::DestinationRejected => "The service returned an unsupported network destination.",
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
}

impl ResolvedStream {
    pub fn duration(&self) -> Option<Duration> {
        self.duration
    }
}

impl Client {
    pub fn search(&self, query: &str, cancel: &Cancellation) -> Result<Vec<Track>, ServiceError> {
        if query.trim().is_empty() || query.chars().count() > 120 {
            return Err(ServiceError::InvalidResponse);
        }
        let response = self.transport.json("https://music.youtube.com/youtubei/v1/search?prettyPrint=false", &json!({
            "context": {"client": {"clientName": "WEB_REMIX", "clientVersion": "1.20260707.12.00", "hl": "en"}},
            "query": query.trim()
        }), "67", "1.20260707.12.00", None, cancel)?;
        parse::search(&response)
    }

    pub fn resolve(&self, id: &str, cancel: &Cancellation) -> Result<ResolvedStream, ServiceError> {
        if !parse::valid_id(id) {
            return Err(ServiceError::InvalidResponse);
        }
        let visitor = self.transport.json("https://www.youtube.com/youtubei/v1/visitor_id?prettyPrint=false", &json!({
            "context": {"client": {"clientName": "WEB", "clientVersion": "2.20260707.00.00", "hl": "en"}}
        }), "1", "2.20260707.00.00", None, cancel)?;
        let visitor = visitor
            .pointer("/responseContext/visitorData")
            .and_then(Value::as_str)
            .filter(|value| value.len() <= 4096)
            .ok_or(ServiceError::InvalidResponse)?;
        let response = self.transport.json("https://www.youtube.com/youtubei/v1/player?prettyPrint=false", &json!({
            "context": {"client": {"clientName": "VISIONOS", "clientVersion": "1.02", "deviceMake": "Apple", "deviceModel": "RealityDevice17,1", "osName": "visionOS", "osVersion": "26.5.23O471", "visitorData": visitor, "hl": "en"}},
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
        })
    }

    pub fn input(&self, stream: ResolvedStream, cancel: Cancellation) -> StreamInput {
        StreamInput::new(self.transport.clone(), stream, cancel)
    }
}
