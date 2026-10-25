---
name: Sillage desktop
description: Familiar music browsing with a persistent playback bar
colors:
  canvas: "#080808"
  sidebar: "#101010"
  surface: "#171717"
  elevated: "#242424"
  hover: "#303030"
  text: "#F5F5F5"
  muted: "#B3B3B3"
  accent: "#DFFF61"
  line: "#303030"
  error: "#FFB4AB"
typography:
  headline:
    fontFamily: Segoe UI
    fontSize: 30px
    fontWeight: 700
  title:
    fontFamily: Segoe UI
    fontSize: 20px
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

Mode: Operate. On 2026-10-25 the user rejected the cream/pine application UI
and explicitly requested Spotify and YouTube Music as the design references.
The replacement uses their familiar music-app composition: dark navigation,
search at the top, artwork beside dense results, and persistent playback below.
The existing app icon remains Sillage's identity. This document records source
implementation; it is not a screenshot approval or a runtime quality verdict.

## Colors

Near-black surfaces and white type carry the interface. Citron is reserved for
selected-track text, a small source icon and keyboard focus. The previous cream
canvas and green navigation surface are retired from the application UI.
The README banner and app icon remain separate existing assets.

## Typography

Segoe UI is the Windows operational typeface; no font file is redistributed.
Headings are 30/700, section titles 20/700, song titles 14/600 and artist labels
12/400. Long titles and artists elide instead of widening a row.

## Layout

Initial window: 1280×800 logical pixels; minimum: 800×560. Keep the Windows frame.
The navigation rail prefers 216 px within 180–232 px constraints. Main content
uses 24 px padding. Outer panels have an 8 px gutter. Search stays above content.
The playback bar spans the full window at 96 px: cover/title left, transport in
the center, duration and volume right. Results use 68 px virtualized rows and
48 px covers. Now-playing artwork is 56 px.

## Elevation & Depth

Flat tonal layering, with no decorative blur, animated backgrounds or shadows.
Panels, selected rows, search focus and hover are separated by surface value.

## Shapes

Panel/button corners are 8 px, covers 4 px and results 6 px. Search and compact
transport buttons are rounded. Icons are original local SVGs on a 24-unit grid,
using a consistent 1.8-unit stroke or solid transport shapes.

## Components

`ActionButton` owns hover, disabled opacity, a visible 2 px keyboard focus ring,
Space/Enter activation and an accessible label/action. `SearchField` uses a
single-line TextInput, text selection colors and a clear action. `Cover` clips
real artwork, using a music icon when it is missing. `DiscoveryTile` starts a
real search on click or keyboard activation; its title is a genre, not a fake album.
The standard Slint volume slider and AboutSlint use the dark palette.

Pochettes load after text results, on the search worker. Up to 32 covers per
search, each resized to at most 192×192 RGBA. Encoded input ≤512 KiB; source
dimensions ≤2048×2048; decoder allocation limit 16 MiB. No disk cache or timer.
New searches cancel old artwork; an in-flight HTTP call remains timeout-bounded.

## Do's and Don'ts

- Preserve this music-app composition for the album, artist and playlist views.
- Give content, covers, focus and readable errors priority over decoration.
- Keep controls connected to real behavior. Add queue/seek/next only with their implementation.
- Do not restore the cream/pine form layout or place a giant logo above results.
- Do not invent recommendations, library counts, albums, progression or account status.
- Do not infer rendered contrast, DPI fit or visual correctness from a build.
