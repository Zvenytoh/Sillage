use rodio::{DeviceSinkBuilder, Player, Source};
use std::process::ExitCode;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

fn check() -> Result<(), &'static str> {
    let mut args = std::env::args_os().skip(1);
    if args.next().as_deref() != Some(std::ffi::OsStr::new("--play-tone")) || args.next().is_some()
    {
        return Err("Explicit --play-tone argument required; no device opened.");
    }
    let failed = Arc::new(AtomicBool::new(false));
    let callback_failed = failed.clone();
    let mut output = DeviceSinkBuilder::from_default_device()
        .map_err(|_| "No default audio output is available.")?
        .with_error_callback(move |_| callback_failed.store(true, Ordering::Release))
        .open_sink_or_fallback()
        .map_err(|_| "The default audio device could not be opened.")?;
    output.log_on_drop(false);
    let player = Player::connect_new(output.mixer());
    let tone = rodio::source::SineWave::new(440.0)
        .take_duration(Duration::from_millis(750))
        .amplify(0.05);
    player.append(tone);
    let started = Instant::now();
    while !player.empty() {
        if failed.load(Ordering::Acquire) {
            return Err("The audio device reported an error.");
        }
        if started.elapsed() > Duration::from_secs(5) {
            return Err("Audio completion deadline exceeded.");
        }
        std::thread::sleep(Duration::from_millis(20));
    }
    if failed.load(Ordering::Acquire) {
        return Err("The audio device reported an error.");
    }
    println!(
        "{{\"tone_hz\":440,\"duration_ms\":750,\"amplitude\":0.05,\"output\":\"submitted_and_drained\",\"audibility\":\"requires_human_confirmation\",\"network\":\"not_used\"}}"
    );
    Ok(())
}

fn main() -> ExitCode {
    match check() {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("Audio output check: {message}");
            ExitCode::FAILURE
        }
    }
}
