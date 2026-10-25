use crate::model::Track;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RecentListen {
    pub track: Track,
    pub played_at_unix_ms: u64,
    pub position_ms: u64,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct LibrarySnapshot {
    #[serde(default)]
    pub favorites: Vec<Track>,
    #[serde(default)]
    pub recent: Vec<RecentListen>,
}

#[derive(Default)]
pub struct History {
    pub entries: Vec<RecentListen>,
    qualified: bool,
}

impl History {
    pub fn begin(&mut self) {
        self.qualified = false;
    }
    pub fn observe(&mut self, track: &Track, position_ms: u64, now_unix_ms: u64) -> bool {
        let threshold = track
            .duration_ms
            .filter(|duration| *duration > 0 && *duration < 60_000)
            .map(|duration| duration / 2)
            .unwrap_or(30_000);
        if self.qualified || position_ms < threshold {
            return false;
        }
        self.qualified = true;
        self.entries.retain(|entry| entry.track.id != track.id);
        self.entries.insert(
            0,
            RecentListen {
                track: track.clone(),
                played_at_unix_ms: now_unix_ms,
                position_ms,
            },
        );
        self.entries.truncate(200);
        true
    }
    pub fn position(&mut self, id: &str, position_ms: u64) -> bool {
        if self.qualified
            && let Some(entry) = self.entries.iter_mut().find(|entry| entry.track.id == id)
        {
            let changed = entry.position_ms != position_ms;
            entry.position_ms = position_ms;
            return changed;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn track(duration_ms: Option<u64>) -> Track {
        Track {
            id: "abcdefghijk".into(),
            title: "Exemple".into(),
            artists: vec![],
            duration_ms,
            thumbnail: None,
        }
    }
    #[test]
    fn preparation_and_buffering_are_not_history() {
        let mut history = History::default();
        assert!(!history.observe(&track(None), 0, 1));
        assert!(!history.observe(&track(None), 29_999, 2));
        assert!(history.entries.is_empty());
        assert!(history.observe(&track(None), 30_000, 3));
    }
    #[test]
    fn short_track_and_repeat_remain_unique() {
        let mut history = History::default();
        assert!(history.observe(&track(Some(20_000)), 10_000, 1));
        history.begin();
        assert!(history.observe(&track(Some(20_000)), 10_000, 2));
        assert_eq!(history.entries.len(), 1);
        assert_eq!(history.entries[0].played_at_unix_ms, 2);
    }
}
