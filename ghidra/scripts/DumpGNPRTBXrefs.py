# DumpGNPRTBXrefs.py — Trace every GNPRTB global address to find
# which functions read each parameter. This maps parameter IDs to
# game mechanics.

outpath = "/Users/tomdimino/Desktop/rebellion-gnprtb-xrefs.txt"

fm = currentProgram.getFunctionManager()
rm = currentProgram.getReferenceManager()
af = currentProgram.getAddressFactory()

# GNPRTB parameter bindings from FUN_0053e450
# Format: (param_id, global_address_hex, description_from_metasharp_or_unknown)
gnprtb_bindings = [
    (0xa00, "006b9088", "Space Travel Time: Base"),
    (0xa01, "006b9068", "Unknown param 1"),
    (0xa02, "006b9064", "Unknown param 2"),
    (0xa03, "006b9084", "Unknown param 3"),
    (0xa04, "006b905c", "Unknown param 4"),
    (0xa05, "006b9080", "Unknown param 5"),
    (0xa06, "006b908c", "Unknown param 6"),
    (0xa07, "006b90d8", "Unknown param 7"),
    (0xa08, "006b90a8", "Unknown param 8"),
    (0xa09, "006b90a4", "Unknown param 9"),
    (0xa0a, "006b9098", "Unknown param 10"),
    (0xa0b, "006b90d0", "Unknown param 11"),
    (0xa0c, "006b90d4", "Unknown param 12"),
    (0xa0d, "006b90b8", "Unknown param 13"),
    (0xa0e, "006b9070", "Unknown param 14"),
    (0xa0f, "006b9090", "Unknown param 15"),
    (0xa10, "006b90cc", "Unknown param 16"),
    (0xa11, "006b90c8", "Unknown param 17"),
    (0xa12, "006b9060", "Unknown param 18"),
    (0xa13, "006b90bc", "Unknown param 19"),
    (0xa14, "006b9058", "Unknown param 20"),
    (0xa15, "006b90dc", "Unknown param 21"),
    (0xa16, "006b907c", "Unknown param 22"),
    (0xa17, "006b90c0", "Unknown param 23"),
    (0xa18, "006b90ac", "Unknown param 24"),
    (0xa19, "006b9074", "Unknown param 25"),
    (0xa20, "006b90c4", "Unknown param 26"),
    (0xa21, "006b909c", "Unknown param 27"),
]

# Per-side bindings (Alliance=1, Empire=2)
gnprtb_side_bindings = [
    (0xa00, 1, "006b9094", "Space Travel Time: Base (Alliance)"),
    (0xa00, 2, "006b9078", "Space Travel Time: Base (Empire)"),
    (0xa01, 1, "006b90a0", "Param 1 (Alliance)"),
    (0xa01, 2, "006b90b0", "Param 1 (Empire)"),
    (0xa02, 1, "006b90b4", "Param 2 (Alliance)"),
    (0xa02, 2, "006b906c", "Param 2 (Empire)"),
]

# Also check the bombardment divisor
extra_globals = [
    ("006bb6e8", "Bombardment GNPRTB divisor (from FUN_0055d8c0)"),
    ("00661a88", "Difficulty modifier table (from FUN_0053e190)"),
    ("006be3b8", "Game state root pointer"),
    ("0065d424", "Unit display data reference"),
]

f = open(outpath, "w")
f.write("# GNPRTB Parameter → Function Cross-References\n\n")

for param_id, addr_hex, desc in gnprtb_bindings:
    addr = af.getAddress(addr_hex)
    refs = rm.getReferencesTo(addr)
    callers = set()
    for ref in refs:
        fromAddr = ref.getFromAddress()
        fn = fm.getFunctionContaining(fromAddr)
        if fn is not None:
            callers.add((str(fn.getEntryPoint()), fn.getName(), fn.getBody().getNumAddresses()))

    f.write("## 0x%04x → DAT_%s — %s\n" % (param_id, addr_hex, desc))
    if callers:
        for cAddr, cName, cSize in sorted(callers):
            f.write("  %s (%s, %d bytes)\n" % (cAddr, cName, cSize))
    else:
        f.write("  (no direct references found)\n")
    f.write("\n")

f.write("\n# Per-Side GNPRTB Bindings\n\n")
for param_id, side, addr_hex, desc in gnprtb_side_bindings:
    addr = af.getAddress(addr_hex)
    refs = rm.getReferencesTo(addr)
    callers = set()
    for ref in refs:
        fromAddr = ref.getFromAddress()
        fn = fm.getFunctionContaining(fromAddr)
        if fn is not None:
            callers.add((str(fn.getEntryPoint()), fn.getName(), fn.getBody().getNumAddresses()))

    f.write("## 0x%04x side=%d → DAT_%s — %s\n" % (param_id, side, addr_hex, desc))
    if callers:
        for cAddr, cName, cSize in sorted(callers):
            f.write("  %s (%s, %d bytes)\n" % (cAddr, cName, cSize))
    else:
        f.write("  (no direct references found)\n")
    f.write("\n")

f.write("\n# Extra Global Data References\n\n")
for addr_hex, desc in extra_globals:
    addr = af.getAddress(addr_hex)
    refs = rm.getReferencesTo(addr)
    callers = set()
    for ref in refs:
        fromAddr = ref.getFromAddress()
        fn = fm.getFunctionContaining(fromAddr)
        if fn is not None:
            callers.add((str(fn.getEntryPoint()), fn.getName(), fn.getBody().getNumAddresses()))

    f.write("## DAT_%s — %s\n" % (addr_hex, desc))
    if callers:
        for cAddr, cName, cSize in sorted(callers):
            f.write("  %s (%s, %d bytes)\n" % (cAddr, cName, cSize))
    else:
        f.write("  (no direct references found)\n")
    f.write("\n")

f.close()
print("Done! Written to %s" % outpath)
