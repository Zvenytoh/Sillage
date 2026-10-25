# M1 local-player surface

Mode: Operate. Authority: the approved project scope, current application icon
and README identity. This is an implementation direction, not an approved screenshot.

- Pine `#10241F`, paper `#F2F2E8`, citron `#DFFF61`, secondary text `#52675B`.
- Use Windows Segoe UI for an operational desktop surface; no font file is redistributed.
- Keep a 208 px navigation rail, flexible content and a fixed 128 px playback region.
- Initial window 1040×660 logical px, minimum 800×560. Windows owns the standard frame.
- One file-opening action; filename, duration, status, Play/Pause, Stop and volume.
- No fake albums, placeholder catalog navigation, invented listening data or progress animation.
- Standard Slint buttons and slider provide keyboard and accessibility behavior.
- Prepare files paused at 25% volume. Do not open an output device on application startup.
- Opening/decoding happens on a worker. No periodic UI timer or network request.
- Error text remains visible and names a recovery action. About exposes the AboutSlint widget.

The UI has not been launched or visually inspected. DPI, truncation, focus,
contrast in rendered controls and native modal behavior remain runtime QA items.
