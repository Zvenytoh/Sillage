# Sillage application icon

## Direction

**The sound leaves a wake.** Two broad, offset forms create a flowing S and a
channel of negative space. The icon uses a dark pine tile and a citron symbol,
with transparent corners outside the rounded square. It is an application
identifier, rather than a complete corporate identity system.

The master was generated with the built-in ImageGen tool on 2026-10-25.
It is a **1,254 × 1,254 RGBA PNG**, not a vector drawing. The artwork has slight
color variation from generation; the palette below describes its direction,
not an assertion that every pixel uses one of two exact RGB values.

| Role | Reference color |
| :--- | :--- |
| Pine | `#10241F` |
| Citron | `#DFFF61` |
| README paper | `#F2F2E8` |

## Files

- [Generated raster master](../assets/sillage-master.png)
- [512 px application icon](../assets/sillage.png)
- [Windows ICO](../assets/sillage.ico): 16, 20, 24, 32, 40, 48, 64, 128 and 256 px PNG frames
- PNG exports at the same sizes: `assets/icons/sillage-{size}.png`
- [README banner](../assets/readme-banner.png): 1,600 × 560 px, composed from the same icon and a typographic layout
- [Inspection sheet](../assets/icon-review.png): actual-size icons on light/dark backgrounds and enlarged small-size pixels

The exports preserve the generated artwork and its alpha channel. No SVG master,
monochrome variant, installer integration or full identity kit is claimed.
README typography uses system Segoe UI/Arial in a presentation layout, rather
than an outlined custom wordmark.

## Inspection performed

The generated master, exported review sheet and README banner were opened as
image files. Actual-size 16, 32, 64 and 128 px examples were inspected on light
and dark backgrounds. The internal channel remains visible; the fine terminal
points naturally reduce at 16 px. Use the ICO's size-specific frames rather
than repeatedly rescaling a large image.

The symbol was compared conceptually with the bundled library's Spotify,
SoundCloud and Tidal descriptions: it does not use their wave-circle, bar-cloud
or diamond constructions. This limited comparison is not a trademark clearance
or reverse-image search. No application was launched and no desktop automation
was used. A taskbar/tray runtime review remains pending authorization.

A hand-refined vector or a simplified tiny-size cut may be useful before a public
release; neither is represented as already produced.

## Generation prompt

Sent to the built-in tool with `transparent_background: true`:

> Use case: logo-brand. Create a finished desktop APPLICATION ICON for a native music player called Sillage. Output one single large square icon, no concept sheet, no text, no lettering, no wordmark, no caption. The name means a wake, the trace something leaves as it moves. Concept: 'the sound leaves a wake'. Design a distinctive broad S-shaped flowing wake made from two bold offset interlocking curved ribbon forms and one clear diagonal negative-space channel. It must read as one compact memorable symbol at 16 pixels, not three thin parallel squiggles. Use a dense near-square silhouette, robust curves, generous open internal gaps, strongly controlled geometry and optical centering. The forms should suggest motion and music through rhythm rather than depicting musical equipment. Avoid generic stock S letter, Spotify waves, Shazam emblem, YouTube play button, headphones, music note, equalizer, spiral, lightning bolt, infinity loop, generic AI sparkle. Visual execution: precise flat vector-like artwork, an acid yellow citron symbol (#DFFF61) on a very dark pine-black (#10241F) rounded-square app tile. Tile fills 90 percent of canvas with generous rounded corners, exterior is genuinely transparent. Symbol fills about 63 percent of tile, with balanced optical margins. Two colors only, solid fills, no gradients, no texture, no glow, no drop shadow, no reflections, no 3D, no mockup, no perspective. Professional deliberate app-icon design, bold contemporary silhouette, slightly organic flowing curves balanced with one precise diagonal cut, simple enough to recognize instantly. Front-facing flat master asset, crisp smooth antialiased edges, high resolution.
