use sillage::model::PlaybackState;
use sillage::playback::{
    Playback, PlaybackCommand, PlaybackError, PlaybackEvent, PlaybackEventKind,
};
use std::sync::mpsc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

fn until(receiver: &mpsc::Receiver<PlaybackEvent>, matches: impl Fn(&PlaybackEventKind) -> bool) {
    for _ in 0..8 {
        let event = receiver
            .recv_timeout(Duration::from_secs(2))
            .expect("worker event");
        if matches(&event.kind) {
            return;
        }
        if let PlaybackEventKind::Failed(message) = event.kind {
            panic!("unexpected failure: {message}");
        }
    }
    panic!("expected event not received");
}

#[test]
fn volume_rejects_non_finite_and_out_of_range() {
    let (events, _receiver) = mpsc::sync_channel(32);
    let playback = Playback::spawn(events).expect("worker");
    for volume in [f32::NAN, f32::INFINITY, -0.1, 1.1] {
        assert_eq!(
            playback.send(PlaybackCommand::SetVolume(volume)),
            Err(PlaybackError::InvalidVolume)
        );
    }
}

#[test]
fn pause_before_open_does_not_start_an_output() {
    let (events, receiver) = mpsc::sync_channel(32);
    let playback = Playback::spawn(events).expect("worker");
    until(&receiver, |event| matches!(event, PlaybackEventKind::Ready));
    playback.send(PlaybackCommand::Pause).expect("pause");
    playback.send(PlaybackCommand::Stop).expect("stop");
    until(&receiver, |event| {
        matches!(event, PlaybackEventKind::StateChanged(PlaybackState::Empty))
    });
}

#[test]
fn stop_releases_prepared_file_without_opening_audio() {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    let path = std::env::temp_dir().join(format!(
        "sillage-fixture-{}-{stamp}.wav",
        std::process::id()
    ));
    let mut wav = Vec::new();
    wav.extend_from_slice(b"RIFF");
    wav.extend_from_slice(&38_u32.to_le_bytes());
    wav.extend_from_slice(b"WAVEfmt ");
    wav.extend_from_slice(&16_u32.to_le_bytes());
    wav.extend_from_slice(&1_u16.to_le_bytes());
    wav.extend_from_slice(&1_u16.to_le_bytes());
    wav.extend_from_slice(&44_100_u32.to_le_bytes());
    wav.extend_from_slice(&88_200_u32.to_le_bytes());
    wav.extend_from_slice(&2_u16.to_le_bytes());
    wav.extend_from_slice(&16_u16.to_le_bytes());
    wav.extend_from_slice(b"data");
    wav.extend_from_slice(&2_u32.to_le_bytes());
    wav.extend_from_slice(&0_i16.to_le_bytes());
    std::fs::write(&path, wav).expect("fixture");
    let (events, receiver) = mpsc::sync_channel(32);
    let playback = Playback::spawn(events).expect("worker");
    playback
        .send(PlaybackCommand::OpenLocal(path.clone()))
        .expect("open");
    until(&receiver, |event| {
        matches!(event, PlaybackEventKind::Loaded { .. })
    });
    playback.send(PlaybackCommand::Stop).expect("stop");
    until(&receiver, |event| {
        matches!(
            event,
            PlaybackEventKind::StateChanged(PlaybackState::Stopped)
        )
    });
    #[cfg(windows)]
    {
        use std::os::windows::fs::OpenOptionsExt;
        let exclusive = std::fs::OpenOptions::new()
            .read(true)
            .share_mode(0)
            .open(&path)
            .expect("file released");
        drop(exclusive);
    }
    drop(playback);
    std::fs::remove_file(path).expect("fixture cleanup");
}
