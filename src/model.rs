#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlaybackState {
    Empty,
    Loading,
    Paused,
    Playing,
    Stopped,
    Ended,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Track {
    pub id: String,
    pub title: String,
    pub artists: Vec<String>,
    #[serde(default)]
    pub duration_ms: Option<u64>,
    #[serde(default)]
    pub thumbnail: Option<String>,
}

impl Track {
    pub fn valid(&self) -> bool {
        crate::service::parse::valid_id(&self.id)
            && !self.title.is_empty()
            && self.title.chars().count() <= 512
            && self.artists.len() <= 8
            && self
                .artists
                .iter()
                .map(|name| name.chars().count())
                .sum::<usize>()
                <= 4096
            && self
                .duration_ms
                .is_none_or(|duration| duration <= 7 * 24 * 3600 * 1000)
            && self
                .thumbnail
                .as_ref()
                .is_none_or(|url| url.len() <= 4096 && crate::service::valid_artwork_url(url))
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct HomePage {
    pub trending: Vec<Track>,
    pub discovery: Vec<Track>,
    pub source: String,
    pub country: String,
    pub fetched_at_unix_ms: u64,
    pub warning: Option<String>,
}

pub fn now_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|value| value.as_millis() as u64)
        .unwrap_or(0)
}

impl PlaybackState {
    pub fn label(self) -> &'static str {
        match self {
            Self::Empty => "Recherchez un titre pour commencer.",
            Self::Loading => "Préparation du morceau…",
            Self::Paused => "En pause",
            Self::Playing => "Lecture en cours",
            Self::Stopped => "Lecture arrêtée",
            Self::Ended => "Lecture terminée",
        }
    }
}
