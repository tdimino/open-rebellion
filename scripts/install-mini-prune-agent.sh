#!/usr/bin/env bash
# Install, on the Mac Mini, a launchd agent that runs scripts/prune-target.py
# on its repository copy every day at 04:30, as a backstop to the prune
# after each scripts/mini-cargo.sh build. It skips while a cargo or rustc
# runs, and logs to ~/or-native/build-cache.log like the per-build prune.
#
#   scripts/install-mini-prune-agent.sh            install or update
#   scripts/install-mini-prune-agent.sh --remove   unload and delete
set -euo pipefail

host="${OPEN_REBELLION_BUILD_HOST:-mac-mini-ts}"
dir="${OPEN_REBELLION_BUILD_DIR:-or-native/open-rebellion}"
label="com.open-rebellion.prune-target"

if [ "${1:-}" = "--remove" ]; then
  ssh "$host" "launchctl bootout gui/\$(id -u)/$label 2>/dev/null; rm -f ~/Library/LaunchAgents/$label.plist"
  exit 0
fi

# shellcheck disable=SC2029
ssh "$host" "set -e; home=\$HOME; plist=\$home/Library/LaunchAgents/$label.plist
mkdir -p \$home/Library/LaunchAgents
cat > \$plist <<PLIST
<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">
<plist version=\"1.0\">
<dict>
  <key>Label</key><string>$label</string>
  <key>ProgramArguments</key>
  <array>
    <string>/usr/bin/python3</string>
    <string>\$home/$dir/scripts/prune-target.py</string>
    <string>--log</string>
    <string>\$home/or-native/build-cache.log</string>
  </array>
  <key>StartCalendarInterval</key><dict><key>Hour</key><integer>4</integer><key>Minute</key><integer>30</integer></dict>
  <key>StandardErrorPath</key><string>\$home/or-native/prune-target.err</string>
  <key>StandardOutPath</key><string>/dev/null</string>
</dict>
</plist>
PLIST
launchctl bootout gui/\$(id -u)/$label 2>/dev/null || true
launchctl bootstrap gui/\$(id -u) \$plist
launchctl print gui/\$(id -u)/$label | grep -E 'state|path' | head -3"
