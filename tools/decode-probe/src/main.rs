use std::fs::File;
use std::io::{Cursor, Read};
use std::path::Path;
use std::process::ExitCode;
use std::time::{Duration, Instant};
use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::{CODEC_TYPE_AAC, DecoderOptions};
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::{MediaSourceStream, ReadOnlySource};
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

const MAX_INPUT_BYTES: usize = 65_536;
const MAX_PACKETS: usize = 256;

fn validate_sample(bytes: &[u8]) -> Result<(), &'static str> {
    if bytes.len() > MAX_INPUT_BYTES {
        return Err("input_exceeds_64_kib");
    }
    if bytes.len() < 12 || &bytes[4..8] != b"ftyp" {
        return Err("invalid_mp4_prefix");
    }
    Ok(())
}

fn decode(path: &Path) -> Result<(), &'static str> {
    let file = File::open(path).map_err(|_| "input_open_failed")?;
    if !file
        .metadata()
        .map_err(|_| "input_metadata_failed")?
        .is_file()
    {
        return Err("input_must_be_regular_file");
    }
    let mut bytes = Vec::new();
    file.take((MAX_INPUT_BYTES + 1) as u64)
        .read_to_end(&mut bytes)
        .map_err(|_| "input_read_failed")?;
    validate_sample(&bytes)?;
    // A captured prefix has no complete-file length or seek semantics.
    let prefix = ReadOnlySource::new(Cursor::new(bytes));
    let source = MediaSourceStream::new(Box::new(prefix), Default::default());
    let mut hint = Hint::new();
    hint.with_extension("m4a");
    let mut format = symphonia::default::get_probe()
        .format(
            &hint,
            source,
            &FormatOptions::default(),
            &MetadataOptions::default(),
        )
        .map_err(|_| "mp4_probe_failed_or_incomplete")?
        .format;
    let track = format.default_track().ok_or("audio_track_missing")?;
    if track.codec_params.codec != CODEC_TYPE_AAC {
        return Err("expected_aac_codec");
    }
    let track_id = track.id;
    let mut decoder = symphonia::default::get_codecs()
        .make(&track.codec_params, &DecoderOptions::default())
        .map_err(|_| "aac_decoder_creation_failed")?;
    let started = Instant::now();
    let mut frames = 0_u64;
    let mut previous_spec = None;
    for _ in 0..MAX_PACKETS {
        if started.elapsed() > Duration::from_secs(5) {
            return Err("decode_budget_exceeded");
        }
        let packet = format
            .next_packet()
            .map_err(|_| "insufficient_or_invalid_packets")?;
        if packet.track_id() != track_id {
            continue;
        }
        let audio = decoder.decode(&packet).map_err(|_| "aac_decode_failed")?;
        let spec = *audio.spec();
        let channels = spec.channels.count();
        if spec.rate == 0
            || spec.rate > 192_000
            || channels == 0
            || channels > 8
            || audio.capacity() > 65_536
        {
            return Err("unsupported_audio_dimensions");
        }
        if previous_spec.is_some_and(|previous| previous != spec) {
            return Err("audio_spec_changed");
        }
        previous_spec = Some(spec);
        let mut samples = SampleBuffer::<f32>::new(audio.capacity() as u64, spec);
        samples.copy_interleaved_ref(audio);
        if samples.samples().iter().any(|sample| !sample.is_finite()) {
            return Err("non_finite_pcm_sample");
        }
        frames += (samples.samples().len() / channels) as u64;
        if frames >= u64::from(spec.rate) {
            println!(
                "{{\"decode\":\"verified_prefix\",\"codec\":\"aac\",\"sample_rate\":{},\"channels\":{},\"frames_per_channel\":{},\"finite_pcm\":true,\"playback\":\"not_attempted\"}}",
                spec.rate, channels, frames
            );
            return Ok(());
        }
    }
    Err("one_second_not_reached_within_packet_budget")
}

fn main() -> ExitCode {
    let mut args = std::env::args_os().skip(1);
    let result = match (args.next(), args.next()) {
        (Some(path), None) => decode(Path::new(&path)),
        _ => Err("usage: sillage-decode-probe <local-m4a-prefix>"),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(code) => {
            eprintln!("Decode probe failed: {code}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{MAX_INPUT_BYTES, validate_sample};

    #[test]
    fn rejects_oversize_and_invalid_inputs() {
        assert_eq!(validate_sample(&[]), Err("invalid_mp4_prefix"));
        let mut bytes = vec![0; MAX_INPUT_BYTES];
        bytes[4..8].copy_from_slice(b"ftyp");
        assert_eq!(validate_sample(&bytes), Ok(()));
        bytes.push(0);
        assert_eq!(validate_sample(&bytes), Err("input_exceeds_64_kib"));
    }
}
