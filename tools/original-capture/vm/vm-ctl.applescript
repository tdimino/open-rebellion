-- Applies the command in ~/VMs/rebellion/vm-ctl.cmd to the capture VM.
-- UTM refuses AppleScript from an SSH session, so a GUI-domain launch agent
-- (com.openrebellion.vmctl) runs this file; vm-game.sh writes the command
-- and kickstarts the agent. Commands, one line:
--   start | stop | status
--   share PATH            set the SPICE WebDAV shared directory
--   click X Y [right]     UTM's absolute input-automation click
--   exec CMDLINE          run cmd.exe /c CMDLINE through the guest agent;
--                         prints the exit code, stdout and stderr
--   pull GUEST HOST       copy a guest file to the host (guest agent)
--   push HOST GUEST       copy a host file into the guest (guest agent)
-- Compile with: osacompile -o vm-ctl.scpt vm-ctl.applescript
set line_ to do shell script "head -1 /Users/tamarru/VMs/rebellion/vm-ctl.cmd"
set AppleScript's text item delimiters to " "
set parts to text items of line_
set AppleScript's text item delimiters to ""
set action to item 1 of parts
tell application "UTM"
	-- vm-ctl.vm, when present, names another VM (a trial clone); otherwise
	-- the capture VM.
	set vmName to do shell script "cat /Users/tamarru/VMs/rebellion/vm-ctl.vm 2>/dev/null || echo 'Rebellion A0 Capture'"
	if not (exists virtual machine named vmName) then return action & " -> no such vm"
	set vm to virtual machine named vmName
	if action is "start" then
		start vm
	else if action is "stop" then
		stop vm by force
	else if action is "share" then
		set dirPath to text ((length of action) + 2) thru -1 of line_
		update registry vm with {POSIX file dirPath}
		return "shared " & dirPath
	else if action is "click" then
		set x to (item 2 of parts) as integer
		set y to (item 3 of parts) as integer
		if (count of parts) > 3 and item 4 of parts is "right" then
			input mouse click of vm at {x, y} with mouse button right
		else
			input mouse click of vm at {x, y} with mouse button left
		end if
		return "clicked " & x & "," & y
	else if action is "exec" then
		set cmdLine to text ((length of action) + 2) thru -1 of line_
		set proc to execute vm at "cmd.exe" with arguments {"/c", cmdLine} with output capturing
		repeat 60 times
			set res to get result proc
			if exited of res then exit repeat
			delay 0.5
		end repeat
		return "exit " & (exit code of res) & linefeed & "stdout: " & (output text of res) & linefeed & "stderr: " & (error text of res)
	else if action is "pull" then
		set f to open file vm at (item 2 of parts) for reading
		pull f to (POSIX file (item 3 of parts))
		return "pulled " & (item 2 of parts)
	else if action is "push" then
		set f to open file vm at (item 3 of parts) for writing
		push f from (POSIX file (item 2 of parts))
		return "pushed " & (item 3 of parts)
	end if
	return action & " -> " & (status of vm as text)
end tell
