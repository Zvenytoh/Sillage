# Silent decoding check — successful authorized execution

## Result on 2026-10-25

The user explicitly authorized the silent check. A fresh public capture returned
65,536 bytes with HTTP 206 at `2026-10-25T16:17:07.3773630Z`. After the source
declaration correction described below, the decoder exited 0 and produced
45,056 finite PCM frames per channel at 44,100 Hz, stereo: approximately
1.022 seconds of decoded samples. This is decoded duration, not a speed measurement.

See the [sanitized result](decode-result-2026-10-25.json). No output device,
browser or account was opened; no decoded audio file was written. The compressed
capture remains local and ignored by Git.

### Initial failure and correction

The first decoder invocation failed with `mp4_probe_failed_or_incomplete`.
Inspection found complete `ftyp` (24 bytes), `moov` (699), `sidx` (8,072) and
`moof` (1,816) boxes, followed by an intentionally incomplete `mdat`.
The original Cursor declared the prefix seekable with a complete-file length.
Symphonia's MP4 reader therefore scanned beyond the available media.

The source now uses Symphonia's `ReadOnlySource` wrapper: an honest declaration
of progressive, non-seekable input. No bytes, limits, parser errors or success
criteria were changed. The same captured file then passed the silent check.
No additional network request was made for the corrected decoder invocation.

## Purpose and scope

Decode an explicitly captured public AAC/MP4 prefix into in-memory PCM, without
opening any audio device. The utility is separate from the future application:
`tools/decode-probe/`. Its only direct dependency is Symphonia 0.5.5 with the
`aac` and `isomp4` features. Its Cargo.lock is committed. It has no rodio, CPAL,
networking, UI or browser dependency.

Version 0.5.5 is the audited reference baseline for this experiment; it is not
a claim to be the newest release or the final application's version selection.
The utility uses Rust edition 2024 and declares Rust 1.90; compilation was
performed with Rust 1.99.1, not with the declared minimum toolchain.

## Executed procedure

The following capture/decode procedure was explicitly authorized and executed.
It is not an authorization for audible playback or unrelated tests.

1. Capture at most 64 KiB with the existing public probe's explicit save option:

   ```powershell
   pwsh -NoProfile -File scripts/probe-service.ps1 -Query Bach -SaveDecodeSample
   ```

   This repeats the public search/player sequence. It saves only the media prefix
   to ignored `.tools/decode-sample.m4a`, never a signed URL or metadata response.
   An existing sample is refused, not overwritten. There is no browser or account.

2. Run the locally prepared decoder against that file:

   ```powershell
   & .\.tools\decode-target\debug\sillage-decode-probe.exe .\.tools\decode-sample.m4a
   ```

   The executable is a local build artifact, not a committed or published binary.
   It makes no network request, opens no audio output and writes no decoded file.

3. Record only the short JSON result and exit code. Success requires at least
   one second of finite AAC PCM frames per channel with consistent audio dimensions.
   A failed or incomplete prefix remains a failed experiment; do not increase
   the network limit, skip corruption or report partial decoding as success.

## Limits and success criteria

- Regular local file, at most 65,536 bytes; ISO BMFF `ftyp` prefix required.
- AAC codec required; sample rate 1–192,000 Hz and 1–8 channels.
- At most 256 demuxed packets; at least one second of frames per channel.
- PCM values must be finite; an audio-spec change fails the check.
- Decoded buffer capacity is checked before allocating the float copy.
- A five-second cooperative budget is checked between packets. It is not a hard
  process timeout and does not interrupt the parser or a decoder call.
- Truncation, missing MP4 indexes, malformed data and decoder errors fail with
  a fixed error code. Source paths, URLs and third-party error strings are not printed.

Some valid MP4 files may need data absent from a 64 KiB prefix. Such a failure
would not prove that full-stream decoding is impossible. Conversely, success
would establish one decoded prefix, not full-track integrity, seeking, output
device behavior, audible quality, gapless playback or performance.

## Checks actually performed

On 2026-10-25, without running either functional probe:

- `rustfmt --check --edition 2024 tools/decode-probe/src/main.rs`: passed.
- `cargo generate-lockfile --manifest-path tools/decode-probe/Cargo.toml`: generated the pinned development-tool graph.
- `cargo check --locked --manifest-path tools/decode-probe/Cargo.toml --target-dir .tools/decode-target --all-targets`: passed.
- `cargo clippy --locked --manifest-path tools/decode-probe/Cargo.toml --target-dir .tools/decode-target --all-targets -- -D warnings`: passed.
- `cargo build --locked --manifest-path tools/decode-probe/Cargo.toml --target-dir .tools/decode-target`: passed.
- PowerShell AST parsing of `scripts/probe-service.ps1`: passed without execution.

The existing GNU Rust toolchain was read from the local tooling installation;
all new Cargo caches and artifacts were written inside Sillage's ignored `.tools/`.
The first check failed because GCC split a space-containing default-manifest
path. A local GCC specs override retaining `crtend.o` and omitting that manifest
resolved the build-tool failure. No UltraMusic file or global configuration changed.
This temporary override is not an application release-build configuration.

One input-boundary unit test was authored and type-checked, but **not executed**.
No `cargo test`, sample capture, decoder invocation, browser, player or audio
output was run during the initial preparation. The separately authorized capture
and decoder execution are recorded above. After the source correction, rustfmt,
Clippy with warnings denied and the build passed again.

## API references

The exact 0.5.5 sources were checked because current upstream examples also
describe the changed 0.6 API:

- [Symphonia 0.5.5 decode example](https://github.com/pdeljanov/Symphonia/blob/v0.5.5/symphonia/examples/basic-interleaved.rs)
- [Symphonia 0.5.5 features and license](https://github.com/pdeljanov/Symphonia/blob/v0.5.5/symphonia/Cargo.toml)
