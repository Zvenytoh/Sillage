# M1 local-file player — implementation record

## Implemented source

- Compiled Slint window: icon, pine/paper/citron surfaces, native file picker,
  filename, duration, status/errors, Play/Pause, Stop, volume and AboutSlint.
- Opening prepares a file paused. The output device opens only on Play.
- MP3, AAC/MP4, WAV, FLAC and Ogg Vorbis enabled through rodio/Symphonia.
- Command and decoder workers; command/event channels bounded to 32 entries.
- PCM queue: 16 chunks × 1,024 frames, at most eight channels. Stereo queue
  payload <= 128 KiB; eight-channel payload <= 512 KiB. Decoder internals and
  the callback's current chunk add memory beyond those queue bounds.
- Callback: nonblocking queue reads, silence on temporary underflow, no explicit
  file read, decoder invocation or thread join. Decoder format changes/nonfinite
  samples terminate this slice instead of silently mixing incompatible chunks.
- Completion/error callbacks retain an atomic fallback when the wake-up queue
  fills. Generations suppress old-track/replay callbacks.
- Stop/shutdown drop sources before joining the decoder. UI command sends never
  wait for channel capacity. A startup single-shot timer starts the event bridge;
  there is no periodic UI timer.

These are implementation facts, not runtime or performance validation.

## Versions and licensing

The user selected MIT on 2026-10-25. Root LICENSE covers original application code.
Slint 1.18.1 requires Rust 1.92, the manifest's declared floor. rodio 0.23.0,
CPAL 0.18.2 and Symphonia 0.5.5 are locked. The Symphonia family matches the
previously successful silent experiment. The floor itself was not tested with
Rust 1.92; local compilation uses Rust 1.99.1, Windows GNU.

rodio recording is disabled. Slint selects Winit/FemtoVG and accessibility, without
selecting Qt, WebView, Skia, WGPU, MCP or system-tray features. Cargo.lock includes
other-target packages; its package count is not a runtime resource measurement.

## Roadmap ruling

The user requested more substantial progress while preparing the output check.
Local M1 implementation therefore proceeds alongside M0, without service or
account assumptions. M0 stays open and connected-service tasks 2–4 have not begun.

The event boundary is `PlaybackEvent { generation, kind }`. Callback tokens live
independently of the Playback owner to avoid retaining workers through the bridge.
Static self-review covered source shutdown, full wake-up queues, stale callbacks
and a failed startup before the event loop starts. No independent agent was used.

## Checks and remaining validation

Formatting, Clippy/all-target type checking and Release compilation are the
delivery's build checks; actual command results are recorded in STATUS/chat.
The final checks passed with Rust 1.99.1 GNU: `cargo fmt --all -- --check`,
`cargo clippy --locked --all-targets -- -D warnings`, and
`cargo build --locked --release --bins`, using local `.tools/app-target`.
The final local player binary is 13,327,872 bytes; the output-check binary is
590,336 bytes. Those file sizes do not establish RAM, startup or audio performance.
The [build record](m1-build-results-2026-10-25.json) includes hashes of both local
artifacts. Neither binary is committed or published as a GitHub release.
Three integration cases are prepared and type-checked, without execution:
invalid volume, pause-before-open and file release on Stop. They do not prove
hardware pause-position behavior. No `cargo test` has been run.

Pending runtime checks: launch/close/startup errors, picker ownership/cancellation,
file loading, actual playback/pause/stop, volume, end notification, file replacement,
device errors, About lifecycle, keyboard/focus, DPI and small-window layout.
Nothing has been visually inspected in the running application. Performance,
audible quality, underrun recovery and callback timing remain unmeasured.

This is a single-file slice. Seek is explicitly unsupported by the PCM source
until a decoder command/acknowledgement path exists; no seek UI or fake progress
bar is shown. Queue, persistence, catalog and accounts remain in later tasks.
`try_recv` is a nonblocking API, not a real-time latency guarantee.
