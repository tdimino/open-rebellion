@echo off
rem Install the owned GOG build of Star Wars: Rebellion inside the A0 capture guest.
rem
rem Run from the payload drive as Administrator:
rem   D:\install.cmd
rem
rem Mirrors goggame-1421404828.script: copies the game to C:\Rebellion, applies
rem the GOG compatibility files from __support\add (d3drm.dll, DDrawCompat
rem ddraw.dll and its REBEXE ini), and writes the six LucasArts registry values
rem that point the game at its data. REBEXE.EXE is 32-bit, so on 64-bit
rem Windows it reads HKLM\Software through the WOW6432Node view; the values
rem are written with /reg:32 (a plain reg add from a 64-bit prompt lands in
rem the 64-bit view and the game reports "CD Not Found"). DirectPlay is
rem enabled through DISM as the GOG script does; failure there is harmless
rem for single-player captures.

setlocal
set SRC=%~dp0game
set DST=C:\Rebellion

echo Copying %SRC% to %DST% ...
robocopy "%SRC%" "%DST%" /E /NFL /NDL /NJH /NJS /R:1 /W:1 >nul
if errorlevel 8 (
  echo robocopy failed
  exit /b 1
)

echo Applying GOG compatibility files ...
copy /Y "%DST%\__support\add\d3drm.dll" "%DST%\d3drm.dll" >nul
copy /Y "%DST%\__support\add\ddraw.dll" "%DST%\ddraw.dll" >nul
copy /Y "%DST%\__support\add\DDrawCompat-REBEXE.ini" "%DST%\DDrawCompat-REBEXE.ini" >nul

echo Writing registry ...
set KEY=HKLM\Software\LucasArts Entertainment Company LLC\Star Wars Rebellion
reg add "%KEY%" /v InstalledLocation /t REG_SZ /d "%DST%" /f /reg:32 >nul
reg add "%KEY%" /v DataLocation /t REG_SZ /d "%DST%" /f /reg:32 >nul
reg add "%KEY%" /v DllLocation /t REG_SZ /d "%DST%" /f /reg:32 >nul
reg add "%KEY%" /v TooBigLocation /t REG_SZ /d "%DST%" /f /reg:32 >nul
reg add "%KEY%" /v EXEPath /t REG_SZ /d "%DST%\REBEXE.EXE" /f /reg:32 >nul
reg add "%KEY%" /v EncyLocation /t REG_SZ /d "%DST%\EDATA" /f /reg:32 >nul

echo Applying GOG compatibility flags to REBEXE.EXE ...
reg add "HKLM\Software\Microsoft\Windows NT\CurrentVersion\AppCompatFlags\Layers" /v "%DST%\REBEXE.EXE" /t REG_SZ /d "~ DISABLETHEMES DISABLEDWM HIGHDPIAWARE RUNASADMIN" /f >nul

echo Enabling DirectPlay (optional, may fail offline) ...
dism /Online /Enable-Feature /FeatureName:DirectPlay /All /NoRestart >nul 2>&1

echo Done. Launch with: "%DST%\REBEXE.EXE" -w
endlocal
