outpath = "/Users/tomdimino/Desktop/rebellion-combat-region.txt"

fm = currentProgram.getFunctionManager()
f = open(outpath, "w")

# Dump ALL functions in the capital ship / combat region (0x4fc000 - 0x510000)
# This is where HullValueDamage, ShieldRechargeRate, WeaponRechargeRate,
# FleetBattle, FleetBombard, and Damaged handlers live
count = 0
it = fm.getFunctions(True)
while it.hasNext():
    fn = it.next()
    addr = fn.getEntryPoint().getOffset()
    if 0x4f0000 <= addr <= 0x540000:
        f.write("0x%08x: %s (%d bytes)\n" % (addr, fn.getName(), fn.getBody().getNumAddresses()))
        count += 1

f.write("\n--- Total: %d functions in combat region ---\n" % count)
f.close()
print("Done! %d functions written to %s" % (count, outpath))
