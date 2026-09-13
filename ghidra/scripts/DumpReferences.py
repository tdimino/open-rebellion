# Print references to selected addresses with containing function and instruction.
# Usage: -postScript DumpReferences.py 0066c958 0066d400
#@category OpenRebellion

listing = currentProgram.getListing()
function_manager = currentProgram.getFunctionManager()
reference_manager = currentProgram.getReferenceManager()

for raw_address in getScriptArgs():
    address = toAddr(raw_address)
    println("references to %s" % address)
    references = reference_manager.getReferencesTo(address)
    found = False
    for reference in references:
        found = True
        source = reference.getFromAddress()
        function = function_manager.getFunctionContaining(source)
        function_label = "<no function>"
        if function is not None:
            function_label = "%s %s" % (function.getName(), function.getEntryPoint())
        instruction = listing.getInstructionAt(source)
        instruction_label = str(instruction) if instruction is not None else "<no instruction>"
        println("  %s %s %s :: %s" %
                (reference.getReferenceType(), source, function_label, instruction_label))
    if not found:
        println("  <none>")
