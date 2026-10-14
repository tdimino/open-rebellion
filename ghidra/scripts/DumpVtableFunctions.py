from ghidra.app.decompiler import DecompInterface
from ghidra.util.task import ConsoleTaskMonitor


output_path = "/tmp/open-rebellion-vtable-functions.txt"
args = getScriptArgs()

if len(args) < 1:
    raise ValueError("usage: DumpVtableFunctions.py <table-address> [entry-count]")

table_address = currentProgram.getAddressFactory().getAddress(args[0].replace("0x", ""))
entry_count = int(args[1], 0) if len(args) > 1 else 32
listing = currentProgram.getListing()
function_manager = currentProgram.getFunctionManager()
memory = currentProgram.getMemory()
monitor = ConsoleTaskMonitor()
decompiler = DecompInterface()
decompiler.openProgram(currentProgram)

with open(output_path, "w") as output:
    output.write("TABLE %s ENTRIES %d\n" % (table_address, entry_count))
    for index in range(entry_count):
        slot = table_address.add(index * 4)
        target_value = memory.getInt(slot) & 0xffffffff
        target = currentProgram.getAddressFactory().getDefaultAddressSpace().getAddress(target_value)
        function = function_manager.getFunctionAt(target)
        data = listing.getDataAt(slot)
        output.write(
            "[%02d] %s -> %s %s data=%s\n"
            % (
                index,
                slot,
                target,
                function.getName() if function is not None else "<not-function>",
                data,
            )
        )
        if function is not None:
            result = decompiler.decompileFunction(function, 120, monitor)
            if result.decompileCompleted():
                output.write(result.getDecompiledFunction().getC())
            else:
                output.write("DECOMPILE FAILED: %s\n" % result.getErrorMessage())
        output.write("\n")

decompiler.dispose()
print("Wrote %s" % output_path)
