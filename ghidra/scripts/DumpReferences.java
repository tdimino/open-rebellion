// Print references to selected addresses with containing function and instruction.
// Usage with analyzeHeadless:
//   -postScript DumpReferences.java 0066c958 0066d400
//@category OpenRebellion

import ghidra.app.script.GhidraScript;
import ghidra.program.model.listing.Function;
import ghidra.program.model.listing.Instruction;

public class DumpReferences extends GhidraScript {
    @Override
    public void run() throws Exception {
        var listing = currentProgram.getListing();
        var functions = currentProgram.getFunctionManager();
        var references = currentProgram.getReferenceManager();

        for (String rawAddress : getScriptArgs()) {
            var address = toAddr(rawAddress.replace("0x", ""));
            println("references to " + address);
            var iterator = references.getReferencesTo(address);
            boolean found = false;
            while (iterator.hasNext()) {
                monitor.checkCancelled();
                found = true;
                var reference = iterator.next();
                var source = reference.getFromAddress();
                Function function = functions.getFunctionContaining(source);
                String functionLabel = function == null
                    ? "<no function>"
                    : function.getName() + " " + function.getEntryPoint();
                Instruction instruction = listing.getInstructionContaining(source);
                String instructionLabel = instruction == null
                    ? "<no instruction>"
                    : instruction.toString();
                println("  " + reference.getReferenceType() + " " + source + " "
                    + functionLabel + " :: " + instructionLabel);
            }
            if (!found) {
                println("  <none>");
            }
        }
    }
}
