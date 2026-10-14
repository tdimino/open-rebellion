from ghidra.program.model.mem import MemoryAccessException

output_path = "/tmp/open-rebellion-listing-data.txt"
listing = currentProgram.getListing()
memory = currentProgram.getMemory()
address_factory = currentProgram.getAddressFactory()
arguments = getScriptArgs()

with open(output_path, "w") as output:
    for argument in arguments:
        if ":" in argument:
            start_text, end_text = argument.split(":", 1)
            start = address_factory.getAddress(start_text)
            end = address_factory.getAddress(end_text)
            output.write("=== LISTING %s:%s ===\n" % (start_text, end_text))
            instruction = listing.getInstructionAt(start)
            if instruction is None:
                instruction = listing.getInstructionAfter(start)
            while instruction is not None and instruction.getAddress().compareTo(end) <= 0:
                output.write("%s  %s\n" % (instruction.getAddress(), instruction))
                instruction = instruction.getNext()
        else:
            address = address_factory.getAddress(argument)
            output.write("=== DATA %s ===\n" % argument)
            try:
                values = [(memory.getByte(address.add(offset)) & 0xff) for offset in range(16)]
                output.write(" ".join(["%02x" % value for value in values]) + "\n")
            except MemoryAccessException as error:
                output.write("ERROR %s\n" % error)

print("Wrote %s" % output_path)
