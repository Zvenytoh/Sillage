# Audio output check — prepared, not executed

No audio device has been opened and no tone has been played by the agent.

`src/bin/audio-output-check.rs` is separate from the UI and uses the same rodio
0.23.0 / CPAL 0.18.2 output stack as M1. It requires the exact argument
`--play-tone`; missing or different arguments fail before opening a device.
It opens only the default device, allowing configuration fallback on that device.
It does not change Windows master volume or use recording/microphone features.

The signal is a generated 440 Hz sine wave, 750 ms long, amplitude 0.05.
There is no file input, network, account, browser or application window.
A temporary completion loop checks every 20 ms and fails after five seconds or
an output error. This deadline starts after device opening, not around driver
initialization. This diagnostic loop is not used by the application.

After explicit authorization, run:

```powershell
& .\.tools\app-target\release\audio-output-check.exe --play-tone
```

Exit 0 and `output: submitted_and_drained` would establish that the output stack
accepted and consumed the signal without reporting an error. The user must
confirm audibility. It would not prove streamed AAC playback, device switching,
latency, audio quality, DRM or performance. No result is published until execution.
Authorization for this check does not authorize desktop automation or sign-in.
