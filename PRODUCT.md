# Sillage

Native Windows desktop client for public YouTube Music search and playback.
Initial target: Windows 11 x64. Application code is MIT, chosen by the user.
Rust + Slint; no WebView, embedded Chromium, Electron or frontend JavaScript.
The user's priorities are responsiveness, low resource use and a polished music UI.
Symphonia through rodio handles audio. Performance budgets remain targets,
not measured results. Keyboard accessibility, useful errors and accessible
AboutSlint attribution are preserved requirements. Local files open paused;
online playback starts only after the user selects Play on a result.

Spotify Desktop is the primary visual and interaction reference. The user rejected
both the cream/pine UI and the first sparse dark list, and explicitly rejected
citron/green as the application accent. Preserve the existing app icon; the
operational palette is black/charcoal/white. `DESIGN.md` records implementation.

Current source includes public search/streaming, a saved local library and a
session queue. No account sign-in, cloud library sync, remote album/artist pages,
playlist editing, stream seeking or protected offline download is claimed.

Runtime permission is separate. The latest request explicitly authorizes opening
and visually checking the app for the French home redesign, superseding the
previous static-only choice for that UI scope. Audio playback, unit/functional
tests and account access still require their own authorization. Their TubeFast
screenshot is a design reference, not evidence of Sillage's current rendering.
The new French home design remains a brainstorming draft. No subagents.
