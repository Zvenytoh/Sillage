# Dependency and licensing investigation

Date: 2026-10-25. This records source inspection, not a resolved Sillage dependency
graph. There is no Cargo manifest or lockfile in Sillage yet.

## Reference versions

TubeFast's inspected lockfile fixes egui/eframe 0.33.3, rodio 0.21.1,
Symphonia 0.5.5 and ureq 2.12.1. Its manifest requires Rust 1.90 and activates
AAC/MP4 decoding while disabling several default features.
These are reference versions, not a recommendation to treat them as the latest
or to pin Sillage blindly to that dependency graph.

## Selected Sillage components

| Component | License evidence | Action before distribution |
| :--- | :--- | :--- |
| Slint | Royalty-free Desktop/Mobile/Web 2.0, GPLv3 and paid licensing paths exist | Select the applicable path for the pinned version and retain attribution |
| Symphonia | MPL-2.0 in the upstream repository | Retain notices and assess source obligations for included or modified covered files |
| rodio | MIT OR Apache-2.0 in upstream | Retain the chosen license and notices; verify the release rather than using development HEAD |
| windows-rs | Microsoft MIT license inspected | Retain license and inspect actual selected crates |
| ureq, serde, serde_json, image decoding dependencies | Exact versions not selected | Verify their packaged licenses and transitive tree when producing Cargo.lock |

The [Slint royalty-free terms](https://slint.dev/terms-and-conditions) provide
an attribution choice: an accessible AboutSlint widget or a discoverable public
webpage badge. A proposed Sillage implementation would use an About screen.
The future application's own license is still undecided; this investigation
does not silently place all dependencies under MIT.

The [rodio development README](https://github.com/RustAudio/rodio) warns of a core
engine rewrite and example/release differences. Use a released crate and its
matching API documentation, not examples copied from development HEAD.

## Assets

The current icon is a generated raster. The README banner contains rendered
system-font text; no Segoe UI font file is redistributed. No font or icon set has
been chosen for the executable. Verify licenses when that choice is made.
Do not import UltraMusic's assets simply because the projects share an author.

## Next dependency gate

- Resolve compatible released versions with current primary documentation.
- Set an explicit Rust MSRV, edition, minimal features and a committed Cargo.lock.
- Inventory transitive licenses and required platform components.
- Record the Sillage code-license decision before publishing application code.
- Do not add a JavaScript runtime, remote configuration executor or heavy service
  SDK without a demonstrated requirement and a reviewed scope change.

Sources: [Symphonia](https://github.com/pdeljanov/Symphonia#license),
[rodio](https://github.com/RustAudio/rodio#license),
[windows-rs MIT](https://github.com/microsoft/windows-rs/blob/master/license-mit),
[TubeFast lockfile](https://github.com/yigitbozyaka/tubefast/blob/c6c55dd6b1ef60f0339737e7279cda2ff3b433e2/Cargo.lock).
