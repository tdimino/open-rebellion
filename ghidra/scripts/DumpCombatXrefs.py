from ghidra.program.util import DefinedDataIterator

outpath = "/Users/tomdimino/Desktop/rebellion-combat-xrefs.txt"

# Key strings whose cross-references will lead us to combat formulas
targets = [
    'HullValueDamage',
    'ShieldRechargeRateCHAllocated',
    'WeaponRechargeRateCHAllocated',
    'SquadSizeDamage',
    'FleetBattleNotif',
    'FleetBombardNotif',
    'FleetBlockadeNotif',
    'DEATHSTAR_FIRE',
    'STRAT_BATTLE_START',
    'VictoryConditions',
    'DestroyedSabotage',
    'DestroyedAssassination',
    'ControlKindBattleWon',
    'ControlKindUprising',
    'Uprising',
    'Blockade',
    'FinalBattle',
    'SHIP_FIRETURBOLASER',
    'SHIP_FIRETORPEDO',
    'BaseCombat',
    'Damaged',
    'CombatUnitFastRepair',
]

rm = currentProgram.getReferenceManager()
f = open(outpath, "w")

for s in DefinedDataIterator.definedStrings(currentProgram):
    try:
        val = str(s.getValue())
    except:
        continue
    if val not in targets:
        continue

    addr = s.getAddress()
    refs = rm.getReferencesTo(addr)
    f.write("=== %s (at %s) ===\n" % (val, addr))

    for ref in refs:
        fromAddr = ref.getFromAddress()
        fn = currentProgram.getFunctionManager().getFunctionContaining(fromAddr)
        fnName = fn.getName() if fn else "NO_FUNCTION"
        fnAddr = fn.getEntryPoint() if fn else "?"
        f.write("  Referenced from %s in %s (entry: %s)\n" % (fromAddr, fnName, fnAddr))

    f.write("\n")

f.close()
print("Done! Written to %s" % outpath)
