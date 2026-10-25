# Service feasibility — static investigation

Date: 2026-10-25. **M0 remains open.** No search request, audio playback, browser
sign-in, account mutation or runtime benchmark was performed.

## Reference examined

TubeFast commit [`c6c55dd6b1ef60f0339737e7279cda2ff3b433e2`](https://github.com/yigitbozyaka/tubefast/tree/c6c55dd6b1ef60f0339737e7279cda2ff3b433e2),
cloned for inspection under ignored `.tools/tubefast/`. No reference code was
executed or copied into Sillage's application modules.

## Capability matrix

| Capability | Source evidence | Sillage conclusion |
| :--- | :--- | :--- |
| `public_search` | `ytm.rs:404`, `music:553`: POST to the music internal API using WEB_REMIX | Implementation path identified; current live behavior unverified |
| `audio_stream` | `ytm.rs:487,507,755`: visitor acquisition, player request, direct audio URL selection | Implementation path identified; current stream availability unverified |
| `account_login` | `auth.rs:234,263,306`: dedicated browser profile, temporary debugging endpoint, YouTube cookies | Not OAuth; requires explicit sign-in and a hardened lifecycle |
| `personal_library` | `ytm.rs:238,288`: account check and personal playlist browsing | Code path exists; scope, account variants and completeness unverified |
| `remote_writes` | `ytm.rs:335`, `app.rs:838`: signed-in like/unlike request and rejection event | Likes exist in this commit despite the README's older limitation; not proof of current success |
| `available_formats` | `ytm.rs:43,755`: chooses itag 140 and requires a direct `url` field | AAC path only in this implementation; no Premium or Opus guarantee |

## Catalog and playback are separate

The catalog uses `music.youtube.com/youtubei/v1/` with WEB_REMIX context. The
player uses `www.youtube.com/youtubei/v1/player` with the client context in
`clients.json`, currently VISIONOS. The inspected player request does not attach
the catalog's account-cookie authorization. Signing in therefore does not by
itself establish higher-quality playback.

The audio selection reads a direct URL. The inspected path contains no JavaScript
signature-deciphering fallback. It can fail if the service no longer returns
that shape. Stream resolution retries visitor acquisition and may refresh client
parameters from the upstream repository. Sillage should begin with embedded,
audited parameters and explicit errors, without silently depending on TubeFast's
mutable configuration endpoint.

`player.rs` uses bounded progressive downloads and a custom PCM source feeding
rodio. It also contains timed buffer waits and periodic output-device checks.
Do not copy this wholesale and claim that Sillage has no polling: use events
where possible and document any necessary bounded wait.

## Authentication boundary

The reference launches Edge/Chrome with a separate profile and a temporary local
debugging port. It reads YouTube cookies through browser debugging, then closes
the launched browser. It signs catalog requests using cookie-derived SAPISIDHASH
and encrypts stored session bytes with Windows DPAPI.

This is a concrete approach to investigate, not an approved Sillage implementation.
Before adopting it, verify browser policy acceptance, bind/control exposure,
temporary-profile ownership, timeout/cancellation, process cleanup, account
selection, session validation and complete removal on logout. No principal
browser profile may be inspected implicitly. Pasted session cookies must not
become a casual diagnostic workflow.

## Network policy to implement

| Purpose | Candidate destinations | Current evidence |
| :--- | :--- | :--- |
| Catalog | Exact host `music.youtube.com`, HTTPS | Source constants |
| Player and visitor ID | Exact host `www.youtube.com`, HTTPS | Source constants |
| Interactive sign-in | Google/YouTube destinations opened by the external browser | Full redirect chain not observed |
| Audio | Validated HTTPS subdomains of `googlevideo.com` | Candidate service policy; actual returned hosts not observed |
| Artwork | Individually reviewed YouTube/Google image hosts, including `i.ytimg.com` | Source paths/examples; live host inventory incomplete |
| Updates | Sillage GitHub releases and reviewed delivery hosts | Architecture decision, no updater exists |

These are not a finished allowlist. Before real requests, parse URLs, reject
credentials, unexpected ports and local/IP destinations, check hostname boundaries,
and revalidate each redirect. Catalog cookies must never be forwarded to audio,
artwork, update or unrelated destinations. No substring-based host checks.
Lyrics and Discord endpoints are outside the initial Sillage scope.

## Decisions and remaining proof

1. Keep the selected native Rust/Slint/audio architecture. No reference evidence
   requires an embedded browser for normal playback.
2. Isolate service-specific client configuration and parsing. Expose unsupported
   formats or expired sessions explicitly, without a hidden WebView fallback.
3. Do not promise account parity, higher bitrates or remote playlist editing.
4. Complete the transport policy before any live service experiment.
5. M0 needs separately authorized functional evidence for public search, stream
   acquisition and playback. Authentication additionally needs an explicit sign-in
   decision; no user account was accessed in this investigation.

## Source links

- [Catalog, player and stream parsing](https://github.com/yigitbozyaka/tubefast/blob/c6c55dd6b1ef60f0339737e7279cda2ff3b433e2/src/ytm.rs)
- [Browser sign-in and DPAPI](https://github.com/yigitbozyaka/tubefast/blob/c6c55dd6b1ef60f0339737e7279cda2ff3b433e2/src/auth.rs)
- [Playback buffering](https://github.com/yigitbozyaka/tubefast/blob/c6c55dd6b1ef60f0339737e7279cda2ff3b433e2/src/player.rs)
- [Like action integration](https://github.com/yigitbozyaka/tubefast/blob/c6c55dd6b1ef60f0339737e7279cda2ff3b433e2/src/app.rs#L838)
- [Embedded client configuration](https://github.com/yigitbozyaka/tubefast/blob/c6c55dd6b1ef60f0339737e7279cda2ff3b433e2/clients.json)
