# Print a bounded instruction listing from one address.
# Usage: -postScript DumpInstructions.py START_ADDRESS COUNT
#@category OpenRebellion

args = getScriptArgs()
if len(args) != 2:
    printerr("usage: DumpInstructions.py START_ADDRESS COUNT")
    exit()

start = toAddr(args[0])
remaining = int(args[1])
instructions = currentProgram.getListing().getInstructions(start, True)
while instructions.hasNext() and remaining > 0:
    instruction = instructions.next()
    function = currentProgram.getFunctionManager().getFunctionContaining(instruction.getAddress())
    label = function.getName() if function is not None else "<no function>"
    println("%s %-18s %s" % (instruction.getAddress(), label, instruction))
    remaining -= 1
