---
name: Sillage desktop
description: Editorial French music browsing with a persistent playback bar
colors:
  canvas: "#090C0E"
  sidebar: "#0B0F11"
  surface: "#111719"
  elevated: "#242A2C"
  hover: "#31383A"
  text: "#F3EEE7"
  muted: "#B9B9B5"
  accent: "#F48575"
  line: "#394043"
  error: "#FFB4AB"
typography:
  headline:
    fontFamily: Georgia
    fontSize: 28px
    fontWeight: 700
  title:
    fontFamily: Georgia
    fontSize: 24px
    fontWeight: 700
  body:
    fontFamily: Segoe UI
    fontSize: 14px
    fontWeight: 400
  label:
    fontFamily: Segoe UI
    fontSize: 12px
    fontWeight: 400
rounded:
  cover: 4px
  row: 6px
  panel: 8px
  search: 24px
spacing:
  small: 8px
  medium: 16px
  large: 24px
components:
  button-primary:
    backgroundColor: "{colors.accent}"
    textColor: "#23110E"
    height: 40px
  search-field:
    backgroundColor: "{colors.elevated}"
    textColor: "{colors.text}"
    height: 42px
    rounded: "{rounded.search}"
---

# Sillage desktop design

## Editorial home revision — 2026-10-25

The user chose flat ink and coral for this revision. The old violet wash is
superseded. Georgia gives the home headline, sections and chart numerals a
distinct reading voice; Segoe UI remains the compact control face. The home
features the leading French chart track, keeps ranks 2–9 compact, and expands
the rest of the actual chart on request. Recent cards sit on a connected trail.
Discovery stays a deliberate 16:9
video rail; song rows keep square crops. Source metadata remains, while cache
age no longer appears as interface copy. Recent listening carries the Sillage
voice. The progression rail uses coral as a simple wake with no extra animation.

The library control now folds only the saved-track list and is absent when
there is no list. The navigation width and labels remain stable. Native visual
inspection, resizing, keyboard focus and pointer behavior are still pending.

## Playback bar geometry refinement

The fixed bar remains 88 px high. Artwork is 56 px at y=16; metadata and the
right-hand actions share the y=44 centerline. Transport controls occupy exactly
232 px, centered above a progress row capped at 620 px. Timestamp boxes have an
explicit 20 px height and the same baseline. Volume has a 96 px allocation.
Progress and volume fills are explicitly anchored at x=0/y=0; their width must
never invoke Slint's automatic centering. Volume fill and thumb use one clamped
fraction. These corrections follow the user's native screenshot; the revised
render and pointer interaction still require native verification.

## Window controls — explicit user revision

The main window has no native title bar (`no-frame: true`). Three integrated
46×32 px caption controls occupy the top-right corner: minimize, maximize/restore,
close, in Windows order. Close uses a red hover state; all controls expose French
accessible labels and keyboard activation. The maximize icon follows the actual
Slint `maximized` property. No separate title strip is drawn.

The branding region and the empty top edge use `WindowMoveArea` for native drag;
double-click toggles maximization. A 6 px resize border is enabled when restored.
The right pane reserves room above its heading for the buttons. Close uses the
existing window-close path, preserving About-window cleanup and state flushing.
This supersedes the earlier instruction to keep the Windows frame. Runtime
dragging, resizing, maximization and caption placement remain unverified.

## Approved French home revision — 2026-10-25

The user selected A Équilibré with violet after comparing mockups, then approved
implementation. This earlier revision established French navigation,
recent listening cards, compact France trends, new music videos, a large
now-playing cover and next tracks, and a fixed transport. The original green
application icon remained separate from the operational accent.

These source dimensions superseded the earlier rebuild notes below: initial
1360×900, minimum 1000×700, sidebar 216/240 px, right pane 300 px reserved from
1200 px, transport 88 px. Home uses four cards, six when its content exceeds
1100 px. Artwork keeps a square container, with wide video artwork contained.
The initial accent was violet #AD91FF, now superseded by the coral palette above.
The native rendering has not yet been inspected.

The user's subsequent native screenshot exposed automatic centering of artwork
over captions, stretched section spacing and uneven trend columns. The refinement
anchors card content explicitly and bounds the scroll content to its intrinsic
height. Discovery uses 16:9 video previews; recents use square covers. Home shows
four, six or eight cards according to available content width. The search hint
is explicitly left aligned. These source changes await native verification.

The older sections describe the preceding reconstruction, not a current visual
approval. Native QA is deferred while the user is gaming.

## Overview

Mode: Operate. On 2026-10-25 the user supplied a screenshot rejecting the first
dark redesign too: a sparse rail, a stretched list, missing product structure,
misaligned transport and unwanted citron. Spotify Desktop is now the primary
composition reference. The rebuild introduces global navigation, a local saved
library, top-result search, a now-playing/queue pane and complete session transport.
The existing app icon remains Sillage's identity. This document records source
implementation; it is not a screenshot approval or a runtime quality verdict.

## Colors

Black and charcoal surfaces, white type and neutral state indicators carry the
interface. No citron/green selection, focus, icon tint or volume control remains
in the operational UI. Color is carried by real artwork and subdued genre tiles.
The README banner and app icon remain separate existing assets.

## Typography

Segoe UI is the Windows operational typeface; no font file is redistributed.
Home headings are 32/700, section titles 23/700, song titles 14/500 and artist labels
12/400. Long titles and artists elide instead of widening a row.

## Layout

Initial window: 1440×900 logical pixels; minimum: 1000×700. Keep the Windows frame.
Global navigation occupies 64 px, with a home action and a 480 px maximum search
field. The library is 240 px, 280 px from 1500 px window width, or 72 px collapsed.
The optional 304 px right pane reserves content width at 1280 px and above;
below that it acts as a closable drawer. Main content uses 24 px padding and
8 px outer gutters. The chrome is explicitly positioned so child text cannot
stretch the grid. Search remains outside the scrolling content.
The playback bar spans the full window at 96 px: cover/title left, transport in
the center, queue/panel/volume actions right. Results use 62 px virtualized rows
and 40 px covers. The bar cover is 56 px; the right pane displays 272 px artwork.
Search All has a top-result card beside song previews and the full list below;
Songs switches to the full list. Home offers real search shortcuts; Saved songs
shows the persisted local library.

Home also presents real track cards from the last search, without invented
recommendations or listening history.

## Elevation & Depth

Flat tonal layering, with no decorative blur, animated backgrounds or shadows.
Panels, selected rows, search focus and hover are separated by surface value.
The progress rail is contained in the middle transport column; no border spans
through the cover, title or volume controls.

## Shapes

Panel/button corners are 8 px, covers 4 px and results 6 px. Search and compact
transport buttons are rounded. Icons are original local SVGs on a 24-unit grid,
using a consistent 1.8-unit stroke or solid transport shapes. The sidebar wordmark
is plain white text without the separate violet brand glyph.

## Components

`ActionButton` owns hover, disabled opacity, a visible 2 px keyboard focus ring,
Space/Enter activation and an accessible label/action. SVGs are centered in
explicit height-filling containers, including transport and search. `SearchField` uses a
single-line TextInput, text selection colors and a clear action. `Cover` clips
real artwork, using a music icon when it is missing. Song rows, saved songs,
recent listens and square cards prefer square album thumbnails and render within
square bounds; video discovery remains 16:9. The service prioritizes square
thumbnail sources closest to the target size. The shuffle and repeat buttons
indicate their active state in neutral white/gray; the other selected controls
retain the current accent. `VolumeControl` uses the current accent fill and supports drag,
arrows, Home/End and accessibility actions. Ctrl+K/Ctrl+L focus search.
The in-app About screen is removed; the public README displays Slint's official
attribution badge instead.

Pochettes load after text results, on a dedicated worker. File input ≤512 KiB;
decoded dimensions ≤2048×2048; allocation ≤16 MiB. The decoded cache is ≤32 MiB;
the encoded disk cache is ≤128 MiB/1024 files. New searches do not cancel images
for items still visible or currently playing. GPU allocations are measured separately.
The library stores at most 500 songs on this PC; the session queue is capped at
500. Saved songs are not a synced YouTube account library. Shuffle, repeat-one,
previous/next and queue selection are wired to playback. Progress counts decoded
PCM actually consumed; inserted buffering silence does not advance it. Rendering
the position is skipped when the window is hidden/minimized. Seeking is read-only
and explicitly marked unavailable.

## Do's and Don'ts

- Preserve this music-app composition for the album, artist and playlist views.
- Give content, covers, focus and readable errors priority over decoration.
- Keep controls connected to real behavior. Do not imply remote playlists, accounts or seeking work.
- Do not restore the cream/pine form layout or place a giant logo above results.
- Do not invent recommendations, library counts, albums, progression or account status.
- Do not infer rendered contrast, DPI fit or visual correctness from a build.
