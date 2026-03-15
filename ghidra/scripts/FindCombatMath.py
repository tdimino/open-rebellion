# Find functions that reference both hull damage and shield values
# by looking for functions that call multiple combat setters

from ghidra.program.model.symbol import RefType

outpath = "/Users/tomdimino/Desktop/rebellion-combat-math.txt"
fm = currentProgram.getFunctionManager()
rm = currentProgram.getReferenceManager()
listing = currentProgram.getListing()

# Key combat notification strings and their addresses (from our xref scan)
combat_strings = {
    "HullValueDamage": 0x006a9610,
    "ShieldRechargeRateCHAllocated": 0x006a964c,
    "WeaponRechargeRateCHAllocated": 0x006a9698,
    "SquadSizeDamage": 0x006a9820,
    "Damaged": 0x006a9318,
    "FleetBattleNotif": 0x006a944c,
    "FleetBombardNotif": 0x006a9488,
    "FleetAssaultNotif": 0x006a94a4,
}

# Also search for functions with many comparisons and arithmetic
# that access object fields at combat-relevant offsets
f = open(outpath, "w")

# Strategy: find the largest functions in the 0x4f0000-0x540000 range
# that contain multiplication or division (combat damage formulas use these)
count = 0
it = fm.getFunctions(True)
big_fns = []
while it.hasNext():
    fn = it.next()
    addr = fn.getEntryPoint().getOffset()
    size = fn.getBody().getNumAddresses()
    if 0x4f0000 <= addr <= 0x540000 and size > 200:
        big_fns.append((addr, fn.getName(), size))

big_fns.sort(key=lambda x: -x[2])

f.write("=== Top 50 largest functions in combat region ===\n\n")
for addr, name, size in big_fns[:50]:
    f.write("0x%08x: %s (%d bytes)\n" % (addr, name, size))

# Now find functions that reference GNPRTB-like global data
# GNPRTB is 213 entries * 44 bytes each = 9372 bytes
# Look for references to the .data section in large combat functions
f.write("\n=== Functions referencing .data globals (likely GNPRTB access) ===\n\n")

data_block = currentProgram.getMemory().getBlock(".data")
if data_block:
    data_start = data_block.getStart().getOffset()
    data_end = data_block.getEnd().getOffset()
    f.write("Data section: 0x%08x - 0x%08x\n\n" % (data_start, data_end))

f.write("\n=== System battle/blockade/bombard area functions ===\n\n")
# The SystemBattle/Blockade/Bombard area (0x511000-0x513000) is where
# system control changes happen during combat
it2 = fm.getFunctions(True)
while it2.hasNext():
    fn = it2.next()
    addr = fn.getEntryPoint().getOffset()
    size = fn.getBody().getNumAddresses()
    if 0x510000 <= addr <= 0x520000 and size > 100:
        f.write("0x%08x: %s (%d bytes)\n" % (addr, fn.getName(), size))

f.close()
print("Done! Written to %s" % outpath)
