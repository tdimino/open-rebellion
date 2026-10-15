from ghidra.app.decompiler import DecompInterface
from ghidra.util.task import ConsoleTaskMonitor

output_path = "/tmp/open-rebellion-interface-functions.txt"
targets = getScriptArgs()

decompiler = DecompInterface()
decompiler.openProgram(currentProgram)
function_manager = currentProgram.getFunctionManager()
reference_manager = currentProgram.getReferenceManager()
monitor = ConsoleTaskMonitor()

with open(output_path, "w") as output:
    for target in targets:
        address_text = target.replace("FUN_", "")
        address = currentProgram.getAddressFactory().getAddress(address_text)
        function = function_manager.getFunctionAt(address)
        if function is None:
            function = function_manager.getFunctionContaining(address)
        if function is None:
            try:
                function = createFunction(address, target)
            except Exception:
                function = None
        output.write("=== %s ===\n" % target)
        if function is None:
            output.write("NOT FOUND\n\n")
            continue

        output.write("ENTRY %s NAME %s\n" % (function.getEntryPoint(), function.getName()))
        callers = set()
        for reference in reference_manager.getReferencesTo(address):
            caller = function_manager.getFunctionContaining(reference.getFromAddress())
            if caller is not None:
                callers.add((str(caller.getEntryPoint()), caller.getName()))
        output.write("CALLERS\n")
        for caller_address, caller_name in sorted(callers):
            output.write("  %s %s\n" % (caller_address, caller_name))

        result = decompiler.decompileFunction(function, 120, monitor)
        if result.decompileCompleted():
            output.write("DECOMPILE\n")
            output.write(result.getDecompiledFunction().getC())
        else:
            output.write("DECOMPILE FAILED: %s\n" % result.getErrorMessage())
        output.write("\n\n")

decompiler.dispose()
print("Wrote %s" % output_path)
