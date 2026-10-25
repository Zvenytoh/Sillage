---
name: Sillage desktop
description: Familiar music browsing with a persistent playback bar
colors:
  canvas: "#000000"
  sidebar: "#121212"
  surface: "#121212"
  elevated: "#242424"
  hover: "#303030"
  text: "#F5F5F5"
  muted: "#B3B3B3"
  accent: "#FFFFFF"
  line: "#303030"
  error: "#FFB4AB"
typography:
  headline:
    fontFamily: Segoe UI
    fontSize: 32px
    fontWeight: 700
  title:
    fontFamily: Segoe UI
    fontSize: 23px
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
    backgroundColor: "{colors.text}"
    textColor: "{colors.canvas}"
    height: 40px
  search-field:
    backgroundColor: "{colors.elevated}"
    textColor: "{colors.text}"
    height: 48px
    rounded: "{rounded.search}"
---

# Sillage desktop design

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
using a consistent 1.8-unit stroke or solid transport shapes.

## Components

`ActionButton` owns hover, disabled opacity, a visible 2 px keyboard focus ring,
Space/Enter activation and an accessible label/action. SVGs are centered in
explicit height-filling containers, including transport and search. `SearchField` uses a
single-line TextInput, text selection colors and a clear action. `Cover` clips
real artwork, using a music icon when it is missing. `GenreTile` starts a
real search on click or keyboard activation; its title is a genre, not a fake album.
`VolumeControl` is monochrome and supports drag, arrows, Home/End and accessibility
actions. Ctrl+K/Ctrl+L focus search. AboutSlint uses the dark palette.

Pochettes load after text results, on a dedicated worker. Up to 32 covers per
search, each resized to at most 320×320 RGBA. Encoded input ≤512 KiB; source
dimensions ≤2048×2048; decoder allocation limit 16 MiB. No disk cache or timer.
New searches cancel old artwork without waiting for its HTTP call. Library and
queue retain at most 24 covers each. Source and GPU allocations are separate.
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
