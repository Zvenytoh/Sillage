# Third-party material

## Application status

Sillage publishes original application code under MIT, documentation, generated
artwork and vendored development skills. No application binary is published yet.
Third-party Rust sources are downloaded by Cargo, not vendored here.

## Application dependencies

- Slint 1.18.1: the Royalty-free Desktop/Mobile/Web 2.0 path is selected. An
  accessible AboutSlint widget is implemented; runtime access needs checking
  before binary distribution.
- rodio 0.23.0: MIT OR Apache-2.0; CPAL 0.18.2: Apache-2.0. Retain their applicable notices.
- Symphonia 0.5.5: MPL-2.0; retain covered-source notices and applicable source availability obligations.
- windows-rs 0.62.2: MIT OR Apache-2.0; retain applicable notices.
- ureq 3.5.0: MIT OR Apache-2.0; rustls provides TLS with bundled web PKI roots.
- url 2.5.8 and serde_json 1.0.154: MIT OR Apache-2.0.
- image 0.25.10: MIT OR Apache-2.0; used to bound and resize remote artwork on a worker.

Cargo.lock records exact transitive versions. A full redistribution notice bundle
is still required before publishing an executable/installer. No Segoe UI font file
is distributed. The current binary remains a local build artifact.

## Reference implementation

TubeFast by Yiğit Bozyaka is inspected as a technical reference at commit
`c6c55dd6b1ef60f0339737e7279cda2ff3b433e2`. Its MIT license is available in its
[repository](https://github.com/yigitbozyaka/tubefast/blob/c6c55dd6b1ef60f0339737e7279cda2ff3b433e2/LICENSE).
No application source from that repository has been incorporated into Sillage.
Any later substantial reuse must preserve the applicable copyright and license.

## Development skills

- Graphify: Apache-2.0; license and NOTICE included in `.codex/skills/graphify/`.
- Caveman: MIT-licensed skill; license included in `.codex/skills/caveman/`.
- Selected Superpowers skills: MIT; licenses included alongside the vendored skills.
- Ponytail: user-supplied skill with MIT declared in its frontmatter.

See [tooling provenance](docs/TOOLING.md) for pinned or local source details.
The Caveman engine and proxy are not included.

## Before the first executable

Replace this preliminary inventory with the exact Cargo.lock dependencies,
features, licenses, notices and asset attributions. Preserve component licensing
separately from the application's license. Slint's selected attribution path and
Symphonia's covered-source requirements must be addressed before distribution.
