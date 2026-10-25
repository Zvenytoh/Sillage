#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlaybackState {
    Empty,
    Loading,
    Paused,
    Playing,
    Stopped,
    Ended,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Track {
    pub id: String,
    pub title: String,
    pub artists: Vec<String>,
    pub duration_ms: Option<u64>,
    pub thumbnail: Option<String>,
}

impl PlaybackState {
    pub fn label(self) -> &'static str {
        match self {
            Self::Empty => "Search for music to begin.",
            Self::Loading => "Preparing audio…",
            Self::Paused => "Ready when you are.",
            Self::Playing => "Playing",
            Self::Stopped => "Stopped",
            Self::Ended => "Finished",
        }
    }
}
