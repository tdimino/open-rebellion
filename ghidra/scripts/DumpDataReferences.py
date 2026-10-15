output_path = "/tmp/open-rebellion-data-references.txt"
targets = getScriptArgs()
reference_manager = currentProgram.getReferenceManager()
function_manager = currentProgram.getFunctionManager()
listing = currentProgram.getListing()

with open(output_path, "w") as output:
    for target_text in targets:
        target = currentProgram.getAddressFactory().getAddress(target_text.replace("0x", ""))
        output.write("=== %s ===\n" % target)
        for reference in reference_manager.getReferencesTo(target):
            source = reference.getFromAddress()
            function = function_manager.getFunctionContaining(source)
            instruction = listing.getInstructionContaining(source)
            output.write(
                "%s %s %s %s\n"
                % (
                    source,
                    function.getName() if function is not None else "<not-function>",
                    reference.getReferenceType(),
                    instruction,
                )
            )
        output.write("\n")

print("Wrote %s" % output_path)
