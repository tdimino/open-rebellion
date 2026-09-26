# Decompile selected functions from the current Ghidra program.
# Usage with analyzeHeadless:
#   -postScript DecompileTargets.py OUTPUT_DIR FUN_005d9eb0 FUN_005da080
#@category OpenRebellion

from ghidra.app.decompiler import DecompInterface
from java.io import File, FileWriter


def target_address(target):
    value = target[4:] if target.startswith("FUN_") else target
    return toAddr(value)


args = getScriptArgs()
if len(args) < 2:
    printerr("usage: DecompileTargets.py OUTPUT_DIR FUNCTION [FUNCTION ...]")
    exit()

output_dir = File(args[0])
if not output_dir.exists() and not output_dir.mkdirs():
    printerr("could not create output directory: %s" % output_dir)
    exit()

decompiler = DecompInterface()
decompiler.setSimplificationStyle("decompile")
if not decompiler.openProgram(currentProgram):
    printerr("could not open decompiler for %s" % currentProgram.getName())
    exit()

try:
    function_manager = currentProgram.getFunctionManager()
    reference_manager = currentProgram.getReferenceManager()
    for target in args[1:]:
        monitor.checkCancelled()
        function = function_manager.getFunctionAt(target_address(target))
        if function is None:
            printerr("function not found: %s" % target)
            continue

        result = decompiler.decompileFunction(function, 120, monitor)
        if result is None or not result.decompileCompleted():
            error = "unknown decompiler error"
            if result is not None and result.getErrorMessage():
                error = result.getErrorMessage()
            printerr("decompilation failed for %s: %s" % (target, error))
            continue

        destination = File(output_dir, "%s.c" % target)
        file_writer = FileWriter(destination)
        try:
            file_writer.write(result.getDecompiledFunction().getC())
        finally:
            file_writer.close()
        println("decompiled %s -> %s" % (target, destination.getAbsolutePath()))

        callers = set()
        for reference in reference_manager.getReferencesTo(function.getEntryPoint()):
            caller = function_manager.getFunctionContaining(reference.getFromAddress())
            if caller is not None:
                callers.add("%s %s" % (caller.getName(), caller.getEntryPoint()))
        caller_file = File(output_dir, "%s.callers.txt" % target)
        file_writer = FileWriter(caller_file)
        try:
            file_writer.write("\n".join(sorted(callers)))
            if callers:
                file_writer.write("\n")
        finally:
            file_writer.close()

        callees = set()
        for callee in function.getCalledFunctions(monitor):
            callees.add("%s %s" % (callee.getName(), callee.getEntryPoint()))
        callee_file = File(output_dir, "%s.callees.txt" % target)
        file_writer = FileWriter(callee_file)
        try:
            file_writer.write("\n".join(sorted(callees)))
            if callees:
                file_writer.write("\n")
        finally:
            file_writer.close()
finally:
    decompiler.dispose()
