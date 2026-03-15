# FindAllFunctions.py — Ghidra script to discover undiscovered functions
# in REBEXE.EXE by scanning the .text section for function prologues.
#
# Run via: Script Manager → Run (green play button)
# Or paste into Console - Scripting (Window → Python)

from ghidra.app.cmd.function import CreateFunctionCmd
from ghidra.program.model.address import AddressSet

program = getCurrentProgram()
listing = program.getListing()
mem = program.getMemory()
fm = program.getFunctionManager()

# Get the .text section bounds
text_block = mem.getBlock(".text")
if text_block is None:
    print("ERROR: No .text section found")
else:
    start = text_block.getStart()
    end = text_block.getEnd()
    print("Scanning .text: %s - %s (%d bytes)" % (start, end, text_block.getSize()))

    # Count existing functions
    existing = fm.getFunctionCount()
    print("Functions before: %d" % existing)

    # Strategy 1: Use Ghidra's built-in function finder on undefined bytes
    from ghidra.app.plugin.core.analysis import AutoAnalysisManager
    mgr = AutoAnalysisManager.getAnalysisManager(program)

    # Strategy 2: Scan for common x86 function prologues
    # PUSH EBP; MOV EBP, ESP = 55 8B EC
    # PUSH EBX; ... = 53
    # SUB ESP, ... = 83 EC xx or 81 EC xx xx xx xx
    found = 0
    addr = start
    while addr is not None and addr.compareTo(end) < 0:
        # Skip if already in a function
        fn = fm.getFunctionContaining(addr)
        if fn is not None:
            # Jump past this function
            body = fn.getBody()
            addr = body.getMaxAddress().next()
            continue

        # Check if this looks like a function prologue
        try:
            b0 = mem.getByte(addr) & 0xff
            b1 = mem.getByte(addr.add(1)) & 0xff
            b2 = mem.getByte(addr.add(2)) & 0xff
        except:
            addr = addr.add(1)
            continue

        is_prologue = False

        # PUSH EBP; MOV EBP, ESP (55 8B EC)
        if b0 == 0x55 and b1 == 0x8B and b2 == 0xEC:
            is_prologue = True
        # PUSH EBP; MOV EBP, ESP (55 89 E5) — alternate encoding
        elif b0 == 0x55 and b1 == 0x89 and b2 == 0xE5:
            is_prologue = True
        # MOV EDI, EDI; PUSH EBP (8B FF 55) — MSVC hotpatch prologue
        elif b0 == 0x8B and b1 == 0xFF and b2 == 0x55:
            is_prologue = True

        if is_prologue:
            cmd = CreateFunctionCmd(addr)
            if cmd.applyTo(program):
                found += 1
                fn = fm.getFunctionAt(addr)
                if fn is not None:
                    addr = fn.getBody().getMaxAddress().next()
                    continue

        addr = addr.add(1)

    new_total = fm.getFunctionCount()
    print("Functions after: %d (found %d new)" % (new_total, found))
    print("Done!")
