#requires -Version 7.4
<#
Public, read-only M0 probe. Run only after explicit functional-test authorization.
No browser, account, playback, persistent cookies, or response-body logging.
Client values reference TubeFast c6c55dd6b1ef60f0339737e7279cda2ff3b433e2.
#>
[CmdletBinding()]
param(
    [ValidateLength(1, 120)][string]$Query = 'Bach',
    [switch]$SaveDecodeSample
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$samplePath = Join-Path $PSScriptRoot '../.tools/decode-sample.m4a'
if ($SaveDecodeSample -and (Test-Path -LiteralPath $samplePath)) {
    throw 'Decode sample already exists. Preserve or remove it explicitly before capturing another.'
}
$stage = 'setup'
$handler = [System.Net.Http.HttpClientHandler]::new()
$handler.AllowAutoRedirect = $false
$handler.UseCookies = $false
$client = [System.Net.Http.HttpClient]::new($handler)
$client.Timeout = [TimeSpan]::FromSeconds(15)

function Invoke-BoundedRequest {
    param([uri]$Uri, [string]$Json, [switch]$Audio, [string]$ClientId, [string]$ClientVersion, [string]$Visitor)
    if ($Uri.Scheme -ne 'https' -or $Uri.Port -ne 443 -or $Uri.UserInfo) {
        throw 'Destination policy rejected.'
    }
    if ($Audio) {
        if ($Uri.DnsSafeHost -notmatch '^(?:[a-z0-9-]+\.)+googlevideo\.com$') {
            throw 'Audio destination rejected.'
        }
    } elseif ($Uri.DnsSafeHost -notin @('music.youtube.com', 'www.youtube.com')) {
        throw 'Metadata destination rejected.'
    }
    $request = [System.Net.Http.HttpRequestMessage]::new()
    $request.RequestUri = $Uri
    $request.Method = if ($Audio) { [System.Net.Http.HttpMethod]::Get } else { [System.Net.Http.HttpMethod]::Post }
    $agent = if ($Audio -or $ClientId -eq '101') {
        'Mozilla/5.0 (Macintosh; Intel Mac OS X 15_7_3) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/26.0 Safari/605.1.15'
    } else {
        'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/140.0.0.0 Safari/537.36'
    }
    $request.Headers.TryAddWithoutValidation('User-Agent', $agent) | Out-Null
    if ($ClientId) { $request.Headers.TryAddWithoutValidation('X-YouTube-Client-Name', $ClientId) | Out-Null }
    if ($ClientVersion) { $request.Headers.TryAddWithoutValidation('X-YouTube-Client-Version', $ClientVersion) | Out-Null }
    if ($Visitor) { $request.Headers.TryAddWithoutValidation('X-Goog-Visitor-Id', $Visitor) | Out-Null }
    if ($Audio) {
        $request.Headers.Range = [System.Net.Http.Headers.RangeHeaderValue]::new(0, 65535)
    } else {
        $request.Content = [System.Net.Http.StringContent]::new($Json, [Text.Encoding]::UTF8, 'application/json')
        $request.Headers.TryAddWithoutValidation('Origin', 'https://music.youtube.com') | Out-Null
    }
    $deadline = [System.Threading.CancellationTokenSource]::new([TimeSpan]::FromSeconds(15))
    $response = $null
    $body = $null
    $memory = [IO.MemoryStream]::new()
    try {
        $response = $client.SendAsync($request, [System.Net.Http.HttpCompletionOption]::ResponseHeadersRead, $deadline.Token).GetAwaiter().GetResult()
        $status = [int]$response.StatusCode
        if (($Audio -and $status -notin @(200, 206)) -or (-not $Audio -and $status -ne 200)) { throw "HTTP status $status." }
        $limit = if ($Audio) { 65536 } else { 4 * 1024 * 1024 }
        $body = $response.Content.ReadAsStreamAsync($deadline.Token).GetAwaiter().GetResult()
        $buffer = [byte[]]::new(16384)
        while ($memory.Length -lt $limit) {
            $remaining = [int][Math]::Min($buffer.Length, $limit - $memory.Length)
            $read = $body.ReadAsync($buffer, 0, $remaining, $deadline.Token).GetAwaiter().GetResult()
            if ($read -eq 0) { break }
            $memory.Write($buffer, 0, $read)
        }
        if (-not $Audio -and $body.ReadAsync($buffer, 0, 1, $deadline.Token).GetAwaiter().GetResult() -ne 0) {
            throw 'Metadata exceeds 4 MiB.'
        }
        if ($Audio) {
            $prefix = $memory.ToArray()
            if ($prefix.Length -lt 12 -or [Text.Encoding]::ASCII.GetString($prefix, 4, 4) -ne 'ftyp') {
                throw 'Expected an ISO BMFF container prefix.'
            }
            return [pscustomobject]@{ status = $status; prefix_bytes = $memory.Length; sample = $prefix }
        }
        return [Text.Encoding]::UTF8.GetString($memory.ToArray()) | ConvertFrom-Json -Depth 64 -AsHashtable
    } finally {
        if ($body) { $body.Dispose() }
        if ($response) { $response.Dispose() }
        $memory.Dispose()
        $request.Dispose()
        $deadline.Dispose()
    }
}

function Find-VideoId {
    param($Node)
    if ($Node -is [System.Collections.IDictionary]) {
        if ($Node.Contains('videoId') -and $Node['videoId'] -is [string] -and $Node['videoId'] -cmatch '^[A-Za-z0-9_-]{11}$') {
            return $Node['videoId']
        }
        foreach ($value in $Node.Values) {
            $found = Find-VideoId $value
            if ($found) { return $found }
        }
    } elseif ($Node -is [System.Collections.IEnumerable] -and $Node -isnot [string]) {
        foreach ($value in $Node) {
            $found = Find-VideoId $value
            if ($found) { return $found }
        }
    }
    return $null
}

try {
    $stage = 'public_search'
    $searchBody = @{ context = @{ client = @{ clientName = 'WEB_REMIX'; clientVersion = '1.20260707.12.00'; hl = 'en' } }; query = $Query } | ConvertTo-Json -Depth 8 -Compress
    $search = Invoke-BoundedRequest -Uri 'https://music.youtube.com/youtubei/v1/search?prettyPrint=false' -Json $searchBody -ClientId '67' -ClientVersion '1.20260707.12.00'
    $videoId = Find-VideoId $search
    if (-not $videoId) { throw 'No public video ID found.' }

    $stage = 'visitor_id'
    $visitorBody = @{ context = @{ client = @{ clientName = 'WEB'; clientVersion = '2.20260707.00.00'; hl = 'en' } } } | ConvertTo-Json -Depth 8 -Compress
    $visitorReply = Invoke-BoundedRequest -Uri 'https://www.youtube.com/youtubei/v1/visitor_id?prettyPrint=false' -Json $visitorBody
    $visitor = $visitorReply['responseContext']['visitorData']
    if (-not $visitor) { throw 'No visitor ID returned.' }

    $stage = 'stream_resolution'
    $playerBody = @{
        context = @{ client = @{ clientName = 'VISIONOS'; clientVersion = '1.02'; deviceMake = 'Apple'; deviceModel = 'RealityDevice17,1'; osName = 'visionOS'; osVersion = '26.5.23O471'; visitorData = $visitor; hl = 'en' } }
        videoId = $videoId
        contentCheckOk = $true
        racyCheckOk = $true
    } | ConvertTo-Json -Depth 8 -Compress
    $player = Invoke-BoundedRequest -Uri 'https://www.youtube.com/youtubei/v1/player?prettyPrint=false' -Json $playerBody -ClientId '101' -ClientVersion '1.02' -Visitor $visitor
    if ($player['playabilityStatus']['status'] -ne 'OK') { throw 'Track is not playable.' }
    $audio = @($player['streamingData']['adaptiveFormats'] | Where-Object { $_['itag'] -eq 140 -and $_['url'] -and $_['mimeType'] -like 'audio/mp4*' })
    if ($audio.Count -eq 0) { throw 'No direct AAC stream returned.' }

    $stage = 'bounded_audio_prefix'
    $prefix = Invoke-BoundedRequest -Uri ([uri]$audio[0]['url']) -Audio
    if ($prefix.prefix_bytes -eq 0) { throw 'Empty audio response.' }
    if ($SaveDecodeSample) {
        $stage = 'save_local_prefix'
        $null = [IO.Directory]::CreateDirectory((Split-Path -Parent $samplePath))
        $sampleFile = [IO.File]::Open($samplePath, [IO.FileMode]::CreateNew, [IO.FileAccess]::Write, [IO.FileShare]::None)
        try { $sampleFile.Write($prefix.sample, 0, $prefix.sample.Length) } finally { $sampleFile.Dispose() }
    }
    [ordered]@{
        utc = [DateTime]::UtcNow.ToString('o')
        public_search = 'video_id_found'
        stream_resolution = 'direct_itag_140'
        audio_prefix_bytes = $prefix.prefix_bytes
        audio_http_status = $prefix.status
        container_prefix = 'ftyp'
        local_decode_sample_saved = [bool]$SaveDecodeSample
        decode = 'not_attempted'
        playback = 'not_attempted'
        account = 'not_accessed'
    } | ConvertTo-Json
} catch {
    # Exception messages may contain signed request URLs. Never emit them.
    [Console]::Error.WriteLine("Probe failed at $stage. Type: $($_.Exception.GetType().Name). No response or URL logged.")
    exit 1
} finally {
    $client.Dispose()
    $handler.Dispose()
}
