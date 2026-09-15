@echo off
rem Relaunch Windows Setup from WinPE with the answer file given explicitly.
rem Needed because QEMU presents the UNATTEND volume as a fixed disk, which
rem Setup's implicit autounattend.xml search skips. The answer file is taken
rem from this script's own drive (the UTM guest tools ISO also ships an
rem autounattend.xml, so never search other drives for it). Run from WinPE:
rem   for %d in (C D E F G H I) do @if exist %d:\go.cmd call %d:\go.cmd
setlocal
set U=%~d0
set S=
for %%d in (C D E F G H I J) do if exist %%d:\setup.exe if exist %%d:\sources\install.wim set S=%%d
if not exist %U%\autounattend.xml echo %U%\autounattend.xml not found & exit /b 1
if "%S%"=="" echo setup.exe not found & exit /b 1
echo Launching %S%:\setup.exe /unattend:%U%\autounattend.xml
%S%:\setup.exe /unattend:%U%\autounattend.xml
