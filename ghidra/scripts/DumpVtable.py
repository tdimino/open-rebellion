output_path = "/tmp/open-rebellion-vtable.txt"
memory = currentProgram.getMemory()
listing = currentProgram.getListing()
function_manager = currentProgram.getFunctionManager()

with open(output_path, "w") as output:
    for target in getScriptArgs():
        table = currentProgram.getAddressFactory().getAddress(target)
        output.write("=== %s ===\n" % target)
        for index in range(0, 48):
            entry = table.add(index * 4)
            try:
                pointer_value = memory.getInt(entry) & 0xffffffff
            except Exception as error:
                output.write("%02d +0x%02x ERROR %s\n" % (index, index * 4, error))
                continue
            pointer = currentProgram.getAddressFactory().getDefaultAddressSpace().getAddress(pointer_value)
            function = function_manager.getFunctionAt(pointer)
            name = function.getName() if function is not None else ""
            output.write("%02d +0x%02x %s %s\n" % (index, index * 4, pointer, name))

print("Wrote %s" % output_path)
