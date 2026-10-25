# Sillage

## Platform

Native Windows 11 desktop application, initially x64.

## Stack

Rust + Slint; Symphonia decoding through rodio for local playback. No WebView.
The user approved following the recorded roadmap and requested more substantial
implementation progress while runtime permissions remain separate.

## Users and purpose

A personal YouTube Music listener wants a native, responsive, resource-conscious
player with a carefully finished interface. Low consumption during long sessions
matters. Performance budgets are targets, not measured results.

## Current surface

M1 is the local-file playback slice. It has no YouTube catalog or account UI.
Opening a file prepares it paused. Audio output starts only on explicit Play.
The application and the audible-output check are not authorized to launch yet.

## Preserved commitments

Use the generated Sillage app icon and established pine/citron identity.
Provide keyboard-accessible controls, useful errors and an accessible Slint
attribution. Application code is MIT, chosen by the user on 2026-10-25.
