outpath = "/Users/tomdimino/Desktop/rebellion-callers.txt"

# Functions we want to find callers of (the damage/shield setters)
targets = {
    "FUN_005029a0": "HullValueDamage setter",
    "FUN_005029f0": "ShieldRechargeRate setter",
    "FUN_00502a40": "WeaponRechargeRate setter",
    "FUN_005038e0": "SquadSizeDamage setter",
    "FUN_004fc3a0": "Damaged handler",
    "FUN_004ff7a0": "FleetBattle trigger",
    "FUN_004ff840": "FleetBombard trigger",
    "FUN_00511ec0": "ControlKindBattleWon",
    "FUN_00512440": "CombatUnitFastRepair",
}

fm = currentProgram.getFunctionManager()
rm = currentProgram.getReferenceManager()
f = open(outpath, "w")

for fnName, desc in targets.items():
    addr = currentProgram.getAddressFactory().getAddress(fnName.replace("FUN_", ""))
    fn = fm.getFunctionAt(addr)
    if fn is None:
        f.write("=== %s (%s) — NOT FOUND ===\n\n" % (fnName, desc))
        continue

    f.write("=== %s (%s) ===\n" % (fnName, desc))
    refs = rm.getReferencesTo(addr)
    callers = set()
    for ref in refs:
        fromAddr = ref.getFromAddress()
        callerFn = fm.getFunctionContaining(fromAddr)
        if callerFn is not None:
            callers.add((str(callerFn.getEntryPoint()), callerFn.getName()))

    for cAddr, cName in sorted(callers):
        f.write("  Called by %s (at %s)\n" % (cName, cAddr))
    f.write("\n")

f.close()
print("Done! Written to %s" % outpath)
