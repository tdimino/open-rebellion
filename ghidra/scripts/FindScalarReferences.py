# Find instruction operands that contain selected scalar values.
# Usage with analyzeHeadless:
#   -postScript FindScalarReferences.py OUTPUT_FILE 0xddc2 0xddc3
#@category OpenRebellion

from ghidra.program.model.scalar import Scalar
args = getScriptArgs()
if len(args) < 2:
    printerr("usage: FindScalarReferences.py OUTPUT_FILE SCALAR [SCALAR ...]")
else:
    targets = set(int(value, 0) for value in args[1:])
    matches = []
    listing = currentProgram.getListing()
    functions = currentProgram.getFunctionManager()
    instructions = listing.getInstructions(True)
    while instructions.hasNext():
        monitor.checkCancelled()
        instruction = instructions.next()
        for operand in range(instruction.getNumOperands()):
            for obj in instruction.getOpObjects(operand):
                if not isinstance(obj, Scalar):
                    continue
                value = obj.getUnsignedValue()
                if value not in targets:
                    continue
                function = functions.getFunctionContaining(instruction.getAddress())
                function_label = (
                    "<no-function>"
                    if function is None
                    else "%s %s" % (function.getName(), function.getEntryPoint())
                )
                matches.append(
                    "%d 0x%x %s %s :: %s"
                    % (value, value, instruction.getAddress(), function_label, instruction)
                )
    matches.sort()
    text = "" if not matches else "\n".join(matches) + "\n"
    with open(args[0], "w") as output:
        output.write(text)
    println("wrote %d scalar references to %s" % (len(matches), args[0]))
