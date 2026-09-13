# Print a bounded 32-bit pointer table and resolve each target symbol/function.
# Usage: -postScript DumpPointerTable.py START_ADDRESS COUNT
#@category OpenRebellion

args = getScriptArgs()
if len(args) != 2:
    printerr("usage: DumpPointerTable.py START_ADDRESS COUNT")
    exit()

address = toAddr(args[0])
count = int(args[1])
functions = currentProgram.getFunctionManager()
symbols = currentProgram.getSymbolTable()

for index in range(count):
    slot = address.add(index * 4)
    target = toAddr(getInt(slot) & 0xffffffff)
    function = functions.getFunctionAt(target)
    symbol = symbols.getPrimarySymbol(target)
    label = function.getName() if function is not None else (symbol.getName() if symbol is not None else "<unresolved>")
    println("%02d %s -> %s %s" % (index, slot, target, label))
