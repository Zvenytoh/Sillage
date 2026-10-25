# M0 public probe network policy

This is the policy for `scripts/probe-service.ps1`, not a claim that the future
Rust transport is implemented. Execute the probe only with explicit authorization
for this functional check. It does not open the application or a browser.

| Request | Allowed destination | Method and bound |
| :--- | :--- | :--- |
| Search | `https://music.youtube.com:443/youtubei/v1/search` | POST, response <= 4 MiB |
| Visitor identifier | `https://www.youtube.com:443/youtubei/v1/visitor_id` | POST, response <= 4 MiB |
| Player metadata | `https://www.youtube.com:443/youtubei/v1/player` | POST, response <= 4 MiB |
| Audio prefix | HTTPS port 443, a label-boundary subdomain of `googlevideo.com` returned by the player | GET with Range 0–65535, read <= 64 KiB |

No redirects are followed. A redirect is a failed experiment to investigate,
not permission to expand the destination set automatically. TLS uses platform
certificate validation. URLs with user information or unexpected ports are rejected.
The audio host must match the entire DNS name, not a substring.

Each request and response-body read has a 15-second cancellation deadline.
The HTTP handler does not store or send cookies. The probe sends no account
authorization. Signed URLs and raw responses remain in memory and are not printed,
written to disk or included in error messages. The explicitly selected
`-SaveDecodeSample` option additionally writes only the bounded media prefix to
ignored `.tools/decode-sample.m4a`, refusing to overwrite an existing file.
No retries, updates, lyric services,
telemetry, browser debugging or account writes are involved.

Success establishes only: a public search response contained a video ID; player
metadata exposed a direct itag-140 audio/mp4 stream; and up to 64 KiB were
retrievable with an ISO BMFF `ftyp` prefix.
It does not establish catalog completeness, decoding, seek, playback, Premium
quality, account support, DRM behavior or performance. The first video ID found
in the response is not guaranteed to be the highest-ranked song result.

The probe uses audited reference client values. It may be refused or become stale.
A failure is recorded as an experiment outcome; it must not trigger unreviewed
client impersonation fallbacks or relaxed network validation.

Command explicitly authorized and executed successfully on 2026-10-25:

```powershell
pwsh -NoProfile -File scripts/probe-service.ps1 -Query Bach
```

Exit code: 0. See the [sanitized result](probe-result-2026-10-25.json).
This authorization covered this public search/prefix check; it does not authorize
audio playback, browser/account access, or unrelated functional tests.

The [silent decoding check](decode-check.md) was subsequently authorized and
executed successfully, including the bounded capture/save option. It did not
open an audio output, browser or account. See its recorded result and limitations.
