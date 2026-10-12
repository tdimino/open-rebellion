<#
.SYNOPSIS
  Prepare the capture guest for the A0 route: scheduled tasks and DDrawCompat.

.DESCRIPTION
  The QEMU guest agent runs as SYSTEM in session 0, which has no desktop:
  a program it starts directly cannot show a window, and a capture taken
  from there is black. A task registered for the logged-on user with an
  interactive logon (schtasks /IT) runs on that user's desktop instead, so
  the guest agent only has to start it (`schtasks /Run /TN A0Watcher`).
  All run elevated: the GOG compatibility layer marks REBEXE.EXE
  RUNASADMIN, and the watcher hashes the elevated game's executable.

    A0Watcher  capture.ps1 -Watch C:\a0 (see capture.ps1), with no console
               window, so nothing covers the game or takes its focus
    A0Game     REBEXE.EXE full screen, in C:\Rebellion
    A0Run      C:\a0\run.ps1 on the desktop, output to C:\a0\run.out, then
               C:\a0\run.done: checks that need the session (display modes,
               window capture trials), from `vm-game.sh a0-run FILE`

  DDrawCompat scales the game's 640x480 mode to the 800x600 desktop.
  DisplayFilter=point makes that scaling repeat whole columns and rows, which
  capture.ps1 verifies and drops to rebuild the game's frame exactly. GOG's
  ini is kept as C:\a0\DDrawCompat-REBEXE.ini.orig the first time.

  Run from the guest agent (vm-game.sh a0-setup); safe to repeat.
#>
param([string]$User = "capture")

$ErrorActionPreference = "Stop"

$ini = "C:\Rebellion\DDrawCompat-REBEXE.ini"
if (-not (Test-Path "C:\a0\DDrawCompat-REBEXE.ini.orig")) { Copy-Item $ini "C:\a0\DDrawCompat-REBEXE.ini.orig" }
if (-not (Select-String -Path $ini -Pattern '^\s*DisplayFilter\s*=\s*point\s*$' -Quiet)) {
  $lines = @(Get-Content $ini | Where-Object { $_ -notmatch '^\s*DisplayFilter\s*=' }) + "DisplayFilter=point"
  Set-Content -Path $ini -Value $lines -Encoding ASCII
}
"ini: " + ((Get-Content $ini) -join "; ")

$principal = New-ScheduledTaskPrincipal -UserId $User -LogonType Interactive -RunLevel Highest
$settings = New-ScheduledTaskSettingsSet -AllowStartIfOnBatteries -DontStopIfGoingOnBatteries `
  -ExecutionTimeLimit ([TimeSpan]::Zero) -MultipleInstances IgnoreNew

# conhost --headless gives PowerShell a console that is never shown.
$watcher = New-ScheduledTaskAction -Execute "conhost.exe" -WorkingDirectory "C:\a0" `
  -Argument "--headless powershell.exe -NoProfile -ExecutionPolicy Bypass -File C:\a0\capture.ps1 -Watch C:\a0"
$game = New-ScheduledTaskAction -Execute "C:\Rebellion\REBEXE.EXE" -WorkingDirectory "C:\Rebellion"
$run = New-ScheduledTaskAction -Execute "conhost.exe" -WorkingDirectory "C:\a0" `
  -Argument "--headless cmd /c powershell.exe -NoProfile -ExecutionPolicy Bypass -File C:\a0\run.ps1 > C:\a0\run.out 2>&1 & echo done > C:\a0\run.done"

Register-ScheduledTask -TaskName "A0Watcher" -Action $watcher -Principal $principal -Settings $settings -Force | Out-Null
Register-ScheduledTask -TaskName "A0Game" -Action $game -Principal $principal -Settings $settings -Force | Out-Null
Register-ScheduledTask -TaskName "A0Run" -Action $run -Principal $principal -Settings $settings -Force | Out-Null
Unregister-ScheduledTask -TaskName "A0GameFull" -Confirm:$false -ErrorAction SilentlyContinue
Get-ScheduledTask -TaskName "A0Watcher", "A0Game", "A0Run" | ForEach-Object { "$($_.TaskName) $($_.State)" }
