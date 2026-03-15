# DumpAllGameFunctions.py — Exhaustive dump of all game-relevant functions
# with their string references and sizes, for complete RE coverage.

from ghidra.program.util import DefinedDataIterator

outpath = "/Users/tomdimino/Desktop/rebellion-all-functions.txt"

fm = currentProgram.getFunctionManager()
rm = currentProgram.getReferenceManager()

# Build a map of all defined strings
string_map = {}  # addr -> string value
for s in DefinedDataIterator.definedStrings(currentProgram):
    try:
        val = str(s.getValue())
        if len(val) > 2:
            string_map[s.getAddress().getOffset()] = val
    except:
        pass

# For each function, find which strings it references
f = open(outpath, "w")
f.write("# All functions in REBEXE.EXE with string references\n")
f.write("# Format: ADDRESS SIZE NAME | string1 | string2 | ...\n\n")

count = 0
game_fns = []  # (addr, name, size, strings)

it = fm.getFunctions(True)
while it.hasNext():
    fn = it.next()
    addr = fn.getEntryPoint().getOffset()
    size = fn.getBody().getNumAddresses()
    name = fn.getName()

    # Skip CRT/thunk functions (tiny, in the first 0x1000 of .text)
    if size < 10:
        continue

    # Find string references from this function
    fn_strings = []
    body = fn.getBody()
    for rng in body:
        start = rng.getMinAddress()
        end = rng.getMaxAddress()
        cur = start
        while cur is not None and cur.compareTo(end) <= 0:
            refs = rm.getReferencesFrom(cur)
            for ref in refs:
                to_offset = ref.getToAddress().getOffset()
                if to_offset in string_map:
                    s = string_map[to_offset]
                    if s not in fn_strings:
                        fn_strings.append(s)
            cur = cur.next()

    if fn_strings or size > 100:
        game_fns.append((addr, name, size, fn_strings))
    count += 1
    if count % 1000 == 0:
        print("Processed %d functions..." % count)

# Sort by size descending
game_fns.sort(key=lambda x: -x[2])

f.write("Total functions processed: %d\n" % count)
f.write("Functions with strings or size>100: %d\n\n" % len(game_fns))

for addr, name, size, strings in game_fns:
    line = "0x%08x %5d %s" % (addr, size, name)
    if strings:
        line += " | " + " | ".join(strings[:10])  # cap at 10 strings per fn
        if len(strings) > 10:
            line += " | ...(%d more)" % (len(strings) - 10)
    f.write(line + "\n")

f.close()
print("Done! %d relevant functions written to %s" % (len(game_fns), outpath))
