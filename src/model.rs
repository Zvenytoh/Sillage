#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlaybackState {
    Empty,
    Loading,
    Paused,
    Playing,
    Stopped,
    Ended,
}

impl PlaybackState {
    pub fn label(self) -> &'static str {
        match self {
            Self::Empty => "Choose an audio file to begin.",
            Self::Loading => "Opening your file…",
            Self::Paused => "Ready when you are.",
            Self::Playing => "Playing",
            Self::Stopped => "Stopped",
            Self::Ended => "Finished",
        }
    }
}
