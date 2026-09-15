-- Create the Open Rebellion A0 capture guest in UTM through its AppleScript bridge.
--
-- Run on the Mac that hosts UTM, from a logged-in desktop session:
--   osascript create-vm.applescript <windows.iso> <unattend.iso> <payload.img> <vnc-bind>
-- Example:
--   osascript create-vm.applescript ~/VMs/rebellion/Win11_25H2_English_Arm64.iso \
--     ~/VMs/rebellion/unattend.iso ~/VMs/rebellion/payload.img 100.125.235.4:1
--
-- The VNC bind exposes the guest framebuffer on the given address:display so the
-- install and the captures can be driven remotely (vncdotool) without the UTM
-- window. Bind it to a Tailscale address, never to 0.0.0.0.
--
-- Drives: USB CD (Windows ISO), USB CD (UNATTEND ISO holding autounattend.xml),
-- NVMe 64 GiB system disk (Windows Arm64 has an inbox NVMe driver; no virtio
-- storage driver is needed at setup), USB raw FAT with an MBR partition table
-- (game payload and capture output). Display: virtio-ramfb.
--
-- The answer file must sit on a CD: Windows Setup's implicit autounattend.xml
-- search covers removable media only, and QEMU presents usb-storage disks as
-- fixed, so a FAT volume is never searched. The UTM guest tools ISO is not
-- attached because it ships its own autounattend.xml, which Setup would pick
-- up first. A FAT image without a partition table is invisible to Windows on
-- a fixed disk, hence the MBR requirement on payload.img.

on run argv
  if (count of argv) < 4 then error "usage: windows.iso unattend.iso payload.img vnc-bind"
  set windowsIso to POSIX file (item 1 of argv)
  set unattendIso to POSIX file (item 2 of argv)
  set payloadImg to POSIX file (item 3 of argv)
  set vncBind to item 4 of argv

  tell application "UTM"
    set vmName to "Rebellion A0 Capture"
    if (exists virtual machine named vmName) then error "A VM named '" & vmName & "' already exists; delete it first."

    set cfg to {name:vmName, architecture:"aarch64", machine:"virt", memory:6144, cpu cores:4, hypervisor:true, uefi:true, ¬
      drives:{ ¬
        {removable:true, interface:USB, source:windowsIso}, ¬
        {removable:true, interface:USB, source:unattendIso}, ¬
        {interface:NVMe, guest size:65536}, ¬
        {interface:USB, raw:true, source:payloadImg} ¬
      }, ¬
      displays:{{hardware:"virtio-ramfb", dynamic resolution:false, native resolution:false}}, ¬
      qemu additional arguments:{{argument string:"-vnc " & vncBind}}}

    set vm to make new virtual machine with properties {backend:qemu, configuration:cfg}
    return "created " & (id of vm)
  end tell
end run
