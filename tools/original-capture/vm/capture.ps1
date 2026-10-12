<#
.SYNOPSIS
  Lossless client-area capture of the original REBEXE.EXE window inside the A0 guest.

.DESCRIPTION
  Runs inside the Windows 11 Arm64 capture guest. Finds the game's top-level
  window, reads its client rectangle through user32, copies exactly those
  device pixels with GDI (no DWM scaling, no host framebuffer), and writes a
  PNG plus a sidecar JSON with the capture rectangle, Windows build, and the
  SHA-256 of the PNG bytes. Two consecutive captures must hash-equal before a
  frame is kept, mirroring the harness's stableFrame rule.

  Usage (from an elevated or normal PowerShell in the guest):
    powershell -ExecutionPolicy Bypass -File D:\capture.ps1 -Name alliance-shell -Out D:\captures
    powershell -ExecutionPolicy Bypass -File D:\capture.ps1 -Name alliance-shell -Out D:\captures -Launch "C:\Rebellion\REBEXE.exe" -LaunchArgs:-w

  The PNG is never committed. Only its SHA-256 and metadata enter the repo
  (docs/qa/2026-09-10-interface-parity-audit/reference-captures/owned-a0/).

  The frame is the full-screen game. DDrawCompat scales its 640x480 mode to
  the guest's 800x600 desktop; with DisplayFilter=point in
  C:\Rebellion\DDrawCompat-REBEXE.ini that scaling repeats a fixed set of 160
  columns and 120 rows (calibrated 2026-10-11 on two different frames). The
  script copies the 800x600 screen, refuses it unless each of those columns
  and rows equals its neighbour, and drops them, which rebuilds the game's
  640x480 frame exactly. Windowed (-w) the game renders differently (more
  colours, anti-aliased text), so a windowed frame is refused for a cell.
  GDI copies whatever is on screen, so the game must be in front.

  Burst mode (-Burst N -IntervalMs M) records motion, which a stable frame
  cannot: N single screen copies M ms apart, timed by a stopwatch, held in
  memory until the last so encoding does not skew the cadence. Each frame
  is still verified and rebuilt as above; none is hash-stabilised, so the
  method says so. The frames and their 800x600 sources go to NAME.burst.zip,
  their offsets and hashes to the JSON; NAME.png is the last frame.

  Watch mode serves captures without keyboard focus. It runs in the logged-on
  session (the A0Watcher scheduled task, a0-tasks.ps1); each
  <dir>\req\NAME.json (the other parameters, as JSON) is captured by a fresh
  PowerShell process into <dir>\out and answered with NAME.done or NAME.err:
    powershell -ExecutionPolicy Bypass -File C:\a0\capture.ps1 -Watch C:\a0
#>
param(
  [string]$Name = "",
  [string]$Out = "",
  [string]$Watch = "",
  [switch]$StopWatch,
  [switch]$FullClient,
  [string]$Launch = "",
  [string]$LaunchArgs = "-w",
  [string]$WindowTitleMatch = "Rebellion",
  [int]$SettleMs = 500,
  [int]$MaxAttempts = 8,
  [int]$Burst = 0,
  [int]$IntervalMs = 250,
  [string]$CellId = "",
  [string]$Requirement = "",
  [ValidateSet("new-game", "original-save", "editor-assisted-save", "live-journey")]
  [string]$StateSetupKind = "live-journey",
  [string]$StateSetupIdentifier = "",
  [string]$StateSetupNotes = "",
  [string[]]$InputTrace = @(),
  # One watch request (JSON of the parameters above), run in this process.
  [string]$Request = "",
  # Echoed into the JSON, so a late answer to an earlier request is detectable.
  [string]$RequestId = ""
)

$ErrorActionPreference = "Stop"

# Stop a running watcher (its loop is the script it started with); run from
# the guest agent before starting a new one.
if ($StopWatch) {
  Get-CimInstance Win32_Process -Filter "Name = 'powershell.exe'" |
    Where-Object { $_.CommandLine -match 'capture\.ps1.*-watch' -and $_.ProcessId -ne $PID } |
    ForEach-Object { Stop-Process -Id $_.ProcessId -Force; "stopped $($_.ProcessId)" }
  return
}

if ($Watch -ne "") {
  $req = Join-Path $Watch "req"
  $outDir = Join-Path $Watch "out"
  $work = Join-Path $Watch "work"
  New-Item -ItemType Directory -Force -Path $req, $outDir, $work | Out-Null
  Write-Output "watching $req"
  while ($true) {
    foreach ($file in Get-ChildItem -Path $req -Filter *.json -ErrorAction SilentlyContinue) {
      $stem = $file.BaseName
      # Take the request out of req\ first: a retry of the same name can land
      # there while this one is still being captured, and must not be deleted.
      $taken = Join-Path $work $file.Name
      Move-Item -Force -Path $file.FullName -Destination $taken
      # A fresh process per request: an edited capture.ps1 applies to the
      # next capture, and nothing (types, variables) carries over.
      $log = Join-Path $work "$stem.log"
      & powershell.exe -NoProfile -ExecutionPolicy Bypass -File $PSCommandPath -Request $taken -Out $outDir *> $log
      # The child answers itself; a crash before it could leaves only the log.
      if (-not (Test-Path (Join-Path $outDir "$stem.done")) -and -not (Test-Path (Join-Path $outDir "$stem.err"))) {
        Set-Content -Path (Join-Path $outDir "$stem.err") -Encoding UTF8 -Value (Get-Content -Raw -Path $log)
      }
      Remove-Item -Path $taken, $log -Force -ErrorAction SilentlyContinue
      Write-Output "$stem $(if (Test-Path (Join-Path $outDir "$stem.done")) { 'done' } else { 'err' })"
    }
    Start-Sleep -Milliseconds 200
  }
}
if ($Request -ne "") {
  $stem = [System.IO.Path]::GetFileNameWithoutExtension($Request)
  try {
    $splat = @{}
    foreach ($p in (Get-Content -Raw -Path $Request | ConvertFrom-Json).PSObject.Properties) { $splat[$p.Name] = $p.Value }
    if ($splat.ContainsKey("InputTrace")) { $splat["InputTrace"] = [string[]]@($splat["InputTrace"]) }
    $splat["Out"] = $Out
    if (-not $splat.ContainsKey("Name")) { $splat["Name"] = $stem }
    $line = & $PSCommandPath @splat
    Set-Content -Path (Join-Path $Out "$stem.done") -Value $line -Encoding UTF8
  } catch {
    Set-Content -Path (Join-Path $Out "$stem.err") -Encoding UTF8 -Value (
      $_.Exception.Message + "`n" + $_.InvocationInfo.PositionMessage + "`n" + $_.ScriptStackTrace)
  }
  return
}
if ($Name -eq "" -or $Out -eq "") { throw "-Name and -Out are required (or -Watch DIR)." }
Add-Type -AssemblyName System.Drawing

Add-Type @"
using System;
using System.Runtime.InteropServices;
public static class Win32 {
  [StructLayout(LayoutKind.Sequential)] public struct RECT { public int Left, Top, Right, Bottom; }
  [StructLayout(LayoutKind.Sequential)] public struct POINT { public int X, Y; }
  [DllImport("user32.dll")] public static extern bool GetClientRect(IntPtr hWnd, out RECT rect);
  [DllImport("user32.dll")] public static extern bool ClientToScreen(IntPtr hWnd, ref POINT pt);
  [DllImport("user32.dll")] public static extern bool SetForegroundWindow(IntPtr hWnd);
  [DllImport("user32.dll")] public static extern IntPtr GetForegroundWindow();
  [DllImport("user32.dll")] public static extern int GetSystemMetrics(int index);
  [DllImport("user32.dll")] public static extern IntPtr WindowFromPoint(POINT pt);
  [DllImport("user32.dll")] public static extern uint GetWindowThreadProcessId(IntPtr hWnd, out uint pid);
}
"@

function Get-GameWindow {
  # The game process first: a console window that launched this script carries
  # the command line, and therefore the word "Rebellion", in its own title.
  $procs = Get-Process -Name "REBEXE" -ErrorAction SilentlyContinue | Where-Object { $_.MainWindowHandle -ne 0 }
  if (-not $procs) {
    $procs = Get-Process | Where-Object {
      $_.MainWindowHandle -ne 0 -and $_.MainWindowTitle -match $WindowTitleMatch -and
      $_.ProcessName -notin @("cmd", "powershell", "pwsh", "conhost", "WindowsTerminal", "explorer")
    }
  }
  if (-not $procs) { return $null }
  return ($procs | Select-Object -First 1)
}

if ($Launch -ne "") {
  Start-Process -FilePath $Launch -ArgumentList $LaunchArgs -WorkingDirectory (Split-Path $Launch)
  $deadline = (Get-Date).AddSeconds(60)
  do { Start-Sleep -Milliseconds 500; $proc = Get-GameWindow } while (-not $proc -and (Get-Date) -lt $deadline)
} else {
  $proc = Get-GameWindow
}
if (-not $proc) { throw "No window matching '$WindowTitleMatch' (and no REBEXE process with a window)." }
if ($CellId -ne "") {
  if ($CellId -notmatch '^TAC-0[1-7]-C\d{3}$') { throw "Invalid tactical cell ID '$CellId'." }
  if ($Requirement -eq "") { throw "-Requirement is required with -CellId." }
  if ($StateSetupIdentifier -eq "") { throw "-StateSetupIdentifier is required with -CellId." }
  if ($InputTrace.Count -eq 0) { throw "-InputTrace is required with -CellId." }
}

$hwnd = $proc.MainWindowHandle
[void][Win32]::SetForegroundWindow($hwnd)
Start-Sleep -Milliseconds $SettleMs
if ([Win32]::GetForegroundWindow() -ne $hwnd) {
  throw "The game window is not in front; GDI would copy whatever covers it."
}

$rect = New-Object Win32+RECT
[void][Win32]::GetClientRect($hwnd, [ref]$rect)
$origin = New-Object Win32+POINT
[void][Win32]::ClientToScreen($hwnd, [ref]$origin)
$clientW = $rect.Right - $rect.Left
$clientH = $rect.Bottom - $rect.Top
$screenW = [Win32]::GetSystemMetrics(0)
$screenH = [Win32]::GetSystemMetrics(1)
$commandLine = (Get-CimInstance Win32_Process -Filter "ProcessId = $($proc.Id)").CommandLine
$windowed = $commandLine -match '\s-w(\s|$)'
if ($FullClient -and $CellId -ne "") { throw "-FullClient is a probe; it cannot carry a cell." }
if ($windowed) {
  # Windowed, the game draws at the desktop's colour depth with anti-aliased
  # text (2026-10-11 comparison): a probe only.
  if (-not $FullClient) { throw "The game is windowed (-w), which renders differently; launch it full screen (A0Game)." }
  $srcX = $origin.X; $srcY = $origin.Y; $w = $clientW; $h = $clientH
} else {
  # Full screen, DDrawCompat reports the game's 640x480 client at 0,0 while it
  # scales the image to the whole desktop, so the screen is what is captured.
  $srcX = 0; $srcY = 0; $w = $screenW; $h = $screenH
}

$ini = "C:\Rebellion\DDrawCompat-REBEXE.ini"
$iniSha = (Get-FileHash -Algorithm SHA256 -Path $ini).Hash.ToLowerInvariant()
if (-not $windowed -and -not $FullClient) {
  if ($w -ne 800 -or $h -ne 600) { throw "The screen is ${w}x$h; the rebuild is calibrated for 800x600." }
  if (-not (Select-String -Path $ini -Pattern '^\s*DisplayFilter\s*=\s*point\s*$' -Quiet)) {
    throw "$ini does not set DisplayFilter=point; the scaled frame cannot be rebuilt exactly."
  }
}

Add-Type -ReferencedAssemblies System.Drawing @"
using System;
using System.Drawing;
using System.Drawing.Imaging;
using System.Runtime.InteropServices;
public static class A0Rebuild {
  // DDrawCompat's point scaling of 640x480 to 800x600 repeats these columns
  // and rows of the 800x600 frame (each equals the one before it).
  public static bool IsDupCol(int x) { return x % 5 == 3; }
  public static readonly int[] DupRows = {
    3, 8, 12, 18, 23, 28, 33, 38, 43, 48, 53, 58, 63, 67, 73, 78, 83, 88, 93, 98,
    103, 108, 112, 118, 122, 128, 133, 138, 143, 147, 153, 157, 163, 168, 173, 178, 183, 188, 193, 198,
    203, 208, 213, 218, 222, 228, 233, 238, 243, 247, 253, 258, 263, 268, 272, 278, 283, 288, 292, 298,
    303, 308, 313, 317, 323, 328, 333, 338, 343, 348, 353, 358, 363, 368, 373, 378, 383, 388, 393, 397,
    402, 408, 413, 418, 423, 428, 433, 438, 442, 447, 453, 458, 463, 468, 473, 478, 483, 488, 492, 497,
    503, 508, 513, 518, 523, 528, 533, 537, 542, 548, 553, 558, 563, 568, 573, 578, 583, 587, 592, 598 };
  public static Bitmap Rebuild(Bitmap full) {
    var rect = new Rectangle(0, 0, 800, 600);
    var data = full.LockBits(rect, ImageLockMode.ReadOnly, PixelFormat.Format24bppRgb);
    var src = new byte[data.Stride * 600];
    Marshal.Copy(data.Scan0, src, 0, src.Length);
    int stride = data.Stride;
    full.UnlockBits(data);
    var dupRow = new bool[600];
    foreach (int y in DupRows) dupRow[y] = true;
    for (int y = 0; y < 600; y++) {
      for (int x = 0; x < 800; x++) {
        int i = y * stride + x * 3;
        if (IsDupCol(x) && (src[i] != src[i - 3] || src[i + 1] != src[i - 2] || src[i + 2] != src[i - 1]))
          throw new Exception("column " + x + " differs from column " + (x - 1) + " at row " + y + ": not the calibrated point scaling");
        if (dupRow[y] && (src[i] != src[i - stride] || src[i + 1] != src[i + 1 - stride] || src[i + 2] != src[i + 2 - stride]))
          throw new Exception("row " + y + " differs from row " + (y - 1) + " at column " + x + ": not the calibrated point scaling");
      }
    }
    var outBmp = new Bitmap(640, 480, PixelFormat.Format24bppRgb);
    var od = outBmp.LockBits(new Rectangle(0, 0, 640, 480), ImageLockMode.WriteOnly, PixelFormat.Format24bppRgb);
    var dst = new byte[od.Stride * 480];
    int oy = 0;
    for (int y = 0; y < 600; y++) {
      if (dupRow[y]) continue;
      int ox = 0;
      for (int x = 0; x < 800; x++) {
        if (IsDupCol(x)) continue;
        int i = y * stride + x * 3, o = oy * od.Stride + ox * 3;
        dst[o] = src[i]; dst[o + 1] = src[i + 1]; dst[o + 2] = src[i + 2];
        ox++;
      }
      oy++;
    }
    Marshal.Copy(dst, 0, od.Scan0, dst.Length);
    outBmp.UnlockBits(od);
    return outBmp;
  }
}
"@

function Png-Bytes($bmp) {
  $ms = New-Object System.IO.MemoryStream
  $bmp.Save($ms, [System.Drawing.Imaging.ImageFormat]::Png)
  return $ms.ToArray()
}

function Capture-Bitmap {
  $bmp = New-Object System.Drawing.Bitmap $w, $h, ([System.Drawing.Imaging.PixelFormat]::Format24bppRgb)
  $g = [System.Drawing.Graphics]::FromImage($bmp)
  $g.CopyFromScreen($srcX, $srcY, 0, 0, (New-Object System.Drawing.Size $w, $h), [System.Drawing.CopyPixelOperation]::SourceCopy)
  $g.Dispose()
  return $bmp
}

# Focus is not visibility: an always-on-top window can cover the game. The
# full-screen image is a presentation window that hit-testing passes
# through, so it is checked by focus and by the scaling pattern instead.
function Assert-Visible {
  if ($srcX -lt 0 -or $srcY -lt 0 -or $srcX + $w -gt $screenW -or $srcY + $h -gt $screenH) {
    throw "Capture rect ($srcX,$srcY ${w}x$h) is not fully on the ${screenW}x${screenH} screen."
  }
  if ($windowed) {
    # Parenthesised: a comma binds tighter than minus in PowerShell.
    foreach ($p in @(@(0, 0), @(($w - 1), 0), @(0, ($h - 1)), @(($w - 1), ($h - 1)), @([int]($w / 2), [int]($h / 2)))) {
      $pt = New-Object Win32+POINT
      $pt.X = $srcX + $p[0]; $pt.Y = $srcY + $p[1]
      $owner = 0
      [void][Win32]::GetWindowThreadProcessId([Win32]::WindowFromPoint($pt), [ref]$owner)
      if ($owner -ne $proc.Id) { throw "Another window covers the game at ($($pt.X),$($pt.Y))." }
    }
  }
  if ([Win32]::GetForegroundWindow() -ne $hwnd) { throw "The game window is not in front." }
}

function Sha256Hex([byte[]]$bytes) {
  $sha = [System.Security.Cryptography.SHA256]::Create()
  return (($sha.ComputeHash($bytes) | ForEach-Object { $_.ToString("x2") }) -join "")
}

New-Item -ItemType Directory -Force -Path $Out | Out-Null
Assert-Visible
$frames = $null
if ($Burst -ne 0) {
  if ($windowed -or $FullClient) { throw "A burst is an A0 capture: full screen, rebuilt." }
  if ($Burst -lt 2 -or $Burst -gt 60) { throw "-Burst takes 2 to 60 frames." }
  if ($IntervalMs -lt 100) { throw "-IntervalMs is at least 100: one screen copy takes tens of ms." }
  if ($Burst * $IntervalMs -gt 30000) { throw "A burst spans at most 30 s; vm-game.sh sizes its wait to that." }
  $clock = [System.Diagnostics.Stopwatch]::StartNew()
  $start = (Get-Date).ToUniversalTime()
  $shots = New-Object System.Collections.ArrayList
  for ($i = 0; $i -lt $Burst; $i++) {
    while ($clock.ElapsedMilliseconds -lt $i * $IntervalMs) { Start-Sleep -Milliseconds 1 }
    $t = $clock.ElapsedMilliseconds
    [void]$shots.Add(@($t, (Capture-Bitmap)))
  }
  Assert-Visible
  $dir = Join-Path $Out "$Name.burst"
  if (Test-Path $dir) { Remove-Item -Recurse -Force $dir }
  New-Item -ItemType Directory -Force -Path $dir | Out-Null
  $frames = @()
  for ($i = 0; $i -lt $shots.Count; $i++) {
    $t = $shots[$i][0]; $bmp = $shots[$i][1]
    $screenBytes = Png-Bytes $bmp
    $stable = Png-Bytes ([A0Rebuild]::Rebuild($bmp))
    $bmp.Dispose()
    $stem = "f{0:D2}" -f $i
    [System.IO.File]::WriteAllBytes((Join-Path $dir "$stem.png"), $stable)
    [System.IO.File]::WriteAllBytes((Join-Path $dir "$stem.screen.png"), $screenBytes)
    $frames += [ordered]@{
      index           = $i
      t_ms            = $t
      captured_at_utc = $start.AddMilliseconds($t).ToString("o")
      sha256          = (Sha256Hex $stable)
      screen_sha256   = (Sha256Hex $screenBytes)
    }
  }
  $zip = Join-Path $Out "$Name.burst.zip"
  if (Test-Path $zip) { Remove-Item -Force $zip }
  Compress-Archive -Path (Join-Path $dir "*") -DestinationPath $zip
  Remove-Item -Recurse -Force $dir
  # The last frame stands for the cell, like a still.
  $w = 640; $h = 480
  [System.IO.File]::WriteAllBytes((Join-Path $Out "$Name.screen.png"), $screenBytes)
} else {
  $prev = $null
  $frame = $null
  for ($i = 0; $i -lt $MaxAttempts; $i++) {
    $bmp = Capture-Bitmap
    $bytes = Png-Bytes $bmp
    $hash = Sha256Hex $bytes
    if ($prev -eq $hash -or $FullClient) { $frame = $bmp; $screenBytes = $bytes; break }
    $bmp.Dispose()
    $prev = $hash
    Start-Sleep -Milliseconds 250
  }
  if (-not $frame) { throw "Frame never stabilised across $MaxAttempts captures; do not register this state." }
  Assert-Visible
  if ($windowed -or $FullClient) {
    $stable = $screenBytes
  } else {
    $rebuilt = [A0Rebuild]::Rebuild($frame)
    $stable = Png-Bytes $rebuilt
    $w = 640; $h = 480
    [System.IO.File]::WriteAllBytes((Join-Path $Out "$Name.screen.png"), $screenBytes)
  }
  $frame.Dispose()
}

$png = Join-Path $Out "$Name.png"
[System.IO.File]::WriteAllBytes($png, $stable)
$meta = [ordered]@{
  name            = $Name
  sha256          = (Sha256Hex $stable)
  width           = $w
  height          = $h
  client_origin   = @($origin.X, $origin.Y)
  client_size     = @($clientW, $clientH)
  windowed        = $windowed
  screen          = @($screenW, $screenH)
  screen_sha256   = (Sha256Hex $screenBytes)
  ddrawcompat_ini_sha256 = $iniSha
  process         = $proc.ProcessName
  executable_sha256 = (Get-FileHash -Algorithm SHA256 -Path $proc.Path).Hash.ToLowerInvariant()
  window_title    = $proc.MainWindowTitle
  windows_build   = (Get-ItemProperty "HKLM:\SOFTWARE\Microsoft\Windows NT\CurrentVersion").BuildLabEx
  captured_at_utc = (Get-Date).ToUniversalTime().ToString("o")
  request_id      = $RequestId
  method          = $(if ($frames) { "burst: $Burst single GDI CopyFromScreen copies of the full-screen game at 800x600 (DDrawCompat DisplayFilter=point), $IntervalMs ms apart by a stopwatch, not hash-stabilised; each frame's 160 calibrated repeated columns and 120 rows verified equal to their neighbours and dropped, giving the game's 640x480 frame exactly; this frame is the last" } elseif ($windowed -or $FullClient) { "probe: GDI CopyFromScreen, not A0" } else { "GDI CopyFromScreen of the full-screen game at 800x600 (DDrawCompat DisplayFilter=point), two consecutive hash-equal frames; the 160 calibrated repeated columns and 120 rows verified equal to their neighbours and dropped, giving the game's 640x480 frame exactly" })
}
if ($CellId -ne "") {
  $meta.cell_id = $CellId
  $meta.requirement = $Requirement
}
# A frame without a cell (a transition's before-frame) keeps its setup too.
if ($StateSetupIdentifier -ne "") {
  $meta.state_setup = [ordered]@{
    kind = $StateSetupKind
    identifier = $StateSetupIdentifier
    notes = $StateSetupNotes
  }
}
if ($InputTrace.Count -gt 0) { $meta.input_trace = @($InputTrace) }
if ($frames) { $meta.burst = [ordered]@{ interval_ms = $IntervalMs; frames = @($frames) } }
$meta | ConvertTo-Json -Depth 5 | Set-Content -Path (Join-Path $Out "$Name.json") -Encoding UTF8
Write-Output ("{0} {1}x{2} sha256={3}" -f $png, $w, $h, $meta.sha256)
