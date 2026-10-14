# Create and decompile selected function starts in the current Ghidra program.
# Usage: -postScript CreateAndDecompileTargets.py OUTPUT_DIR ADDRESS [ADDRESS ...]
#@category OpenRebellion

from ghidra.app.decompiler import DecompInterface
from ghidra.program.model.symbol import SourceType
from java.io import File, FileWriter


args = getScriptArgs()
if len(args) < 2:
    printerr("usage: CreateAndDecompileTargets.py OUTPUT_DIR ADDRESS [ADDRESS ...]")
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
    manager = currentProgram.getFunctionManager()
    for target in args[1:]:
        monitor.checkCancelled()
        address = toAddr(target[4:] if target.startswith("FUN_") else target)
        function = manager.getFunctionAt(address)
        if function is None:
            if not disassemble(address):
                printerr("could not disassemble: %s" % target)
                continue
            function = createFunction(address, "FUN_%s" % address)
        if function is None:
            printerr("function not found or created: %s" % target)
            continue

        result = decompiler.decompileFunction(function, 120, monitor)
        if result is None or not result.decompileCompleted():
            error = result.getErrorMessage() if result is not None else "unknown decompiler error"
            printerr("decompilation failed for %s: %s" % (target, error))
            continue

        destination = File(output_dir, "%s.c" % target)
        writer = FileWriter(destination)
        try:
            writer.write(result.getDecompiledFunction().getC())
        finally:
            writer.close()
        println("decompiled %s -> %s" % (target, destination.getAbsolutePath()))
finally:
    decompiler.dispose()
