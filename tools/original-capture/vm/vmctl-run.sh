#!/bin/sh
# Run one vm-ctl.applescript command through the GUI-domain launch agent
# (UTM refuses AppleScript from SSH) and print what it logged.
#   vmctl-run.sh "COMMAND" [TIMEOUT_SECONDS]
# Lives in ~/VMs/rebellion on the Mac Mini beside vm-ctl.scpt.
cd "$HOME/VMs/rebellion" || exit 1
before=$(wc -l < vm-ctl.log)
printf '%s\n' "$1" > vm-ctl.cmd
launchctl kickstart "gui/$(id -u)/com.openrebellion.vmctl"
i=0
sleep 1
while pgrep -f "osascript $HOME/VMs/rebellion/vm-ctl.scpt" >/dev/null; do
  i=$((i + 1))
  [ "$i" -ge "${2:-60}" ] && { echo "vm-ctl: timed out" >&2; exit 1; }
  sleep 1
done
tail -n +"$((before + 1))" vm-ctl.log
