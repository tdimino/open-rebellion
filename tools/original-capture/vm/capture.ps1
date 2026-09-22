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
#>
param(
  [Parameter(Mandatory = $true)][string]$Name,
  [Parameter(Mandatory = $true)][string]$Out,
  [string]$Launch = "",
  [string]$LaunchArgs = "-w",
  [string]$WindowTitleMatch = "Rebellion",
  [int]$SettleMs = 500,
  [int]$MaxAttempts = 8,
  [string]$CellId = "",
  [string]$Requirement = "",
  [ValidateSet("new-game", "original-save", "editor-assisted-save", "live-journey")]
  [string]$StateSetupKind = "live-journey",
  [string]$StateSetupIdentifier = "",
  [string]$StateSetupNotes = "",
  [string[]]$InputTrace = @()
)

$ErrorActionPreference = "Stop"
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

$rect = New-Object Win32+RECT
[void][Win32]::GetClientRect($hwnd, [ref]$rect)
$origin = New-Object Win32+POINT
[void][Win32]::ClientToScreen($hwnd, [ref]$origin)
$w = $rect.Right - $rect.Left
$h = $rect.Bottom - $rect.Top
if ($w -le 0 -or $h -le 0) { throw "Client rect is empty ($w x $h)." }

function Capture-Bytes {
  $bmp = New-Object System.Drawing.Bitmap $w, $h, ([System.Drawing.Imaging.PixelFormat]::Format24bppRgb)
  $g = [System.Drawing.Graphics]::FromImage($bmp)
  $g.CopyFromScreen($origin.X, $origin.Y, 0, 0, (New-Object System.Drawing.Size $w, $h), [System.Drawing.CopyPixelOperation]::SourceCopy)
  $g.Dispose()
  $ms = New-Object System.IO.MemoryStream
  $bmp.Save($ms, [System.Drawing.Imaging.ImageFormat]::Png)
  $bmp.Dispose()
  return $ms.ToArray()
}

function Sha256Hex([byte[]]$bytes) {
  $sha = [System.Security.Cryptography.SHA256]::Create()
  return (($sha.ComputeHash($bytes) | ForEach-Object { $_.ToString("x2") }) -join "")
}

New-Item -ItemType Directory -Force -Path $Out | Out-Null
$prev = $null
$stable = $null
for ($i = 0; $i -lt $MaxAttempts; $i++) {
  $bytes = Capture-Bytes
  $hash = Sha256Hex $bytes
  if ($prev -eq $hash) { $stable = $bytes; break }
  $prev = $hash
  Start-Sleep -Milliseconds 250
}
if (-not $stable) { throw "Frame never stabilised across $MaxAttempts captures; do not register this state." }

$png = Join-Path $Out "$Name.png"
[System.IO.File]::WriteAllBytes($png, $stable)
$meta = [ordered]@{
  name            = $Name
  sha256          = (Sha256Hex $stable)
  width           = $w
  height          = $h
  client_origin   = @($origin.X, $origin.Y)
  screen          = @([Win32]::GetSystemMetrics(0), [Win32]::GetSystemMetrics(1))
  process         = $proc.ProcessName
  executable_sha256 = (Get-FileHash -Algorithm SHA256 -Path $proc.Path).Hash.ToLowerInvariant()
  window_title    = $proc.MainWindowTitle
  windows_build   = (Get-ItemProperty "HKLM:\SOFTWARE\Microsoft\Windows NT\CurrentVersion").BuildLabEx
  captured_at_utc = (Get-Date).ToUniversalTime().ToString("o")
  method          = "GDI CopyFromScreen of client rect; two consecutive hash-equal frames"
}
if ($CellId -ne "") {
  $meta.cell_id = $CellId
  $meta.requirement = $Requirement
  $meta.state_setup = [ordered]@{
    kind = $StateSetupKind
    identifier = $StateSetupIdentifier
    notes = $StateSetupNotes
  }
  $meta.input_trace = @($InputTrace)
}
$meta | ConvertTo-Json -Depth 3 | Set-Content -Path (Join-Path $Out "$Name.json") -Encoding UTF8
Write-Output ("{0} {1}x{2} sha256={3}" -f $png, $w, $h, $meta.sha256)
