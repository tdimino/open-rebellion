from java.lang import Float


output_path = "/tmp/open-rebellion-float-constants.txt"
targets = getScriptArgs()
memory = currentProgram.getMemory()

with open(output_path, "w") as output:
    for target_text in targets:
        address = currentProgram.getAddressFactory().getAddress(target_text.replace("0x", ""))
        signed_bits = memory.getInt(address)
        bits = signed_bits & 0xffffffff
        output.write(
            "%s bits=0x%08x float=%s\n"
            % (address, bits, Float.intBitsToFloat(signed_bits))
        )

print("Wrote %s" % output_path)
