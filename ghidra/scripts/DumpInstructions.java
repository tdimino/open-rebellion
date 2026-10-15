// Print a bounded instruction listing from one address.
// Usage with analyzeHeadless:
//   -postScript DumpInstructions.java START_ADDRESS COUNT
//@category OpenRebellion

import ghidra.app.script.GhidraScript;

public class DumpInstructions extends GhidraScript {
    @Override
    public void run() throws Exception {
        String[] args = getScriptArgs();
        if (args.length != 2) {
            printerr("usage: DumpInstructions.java START_ADDRESS COUNT");
            return;
        }

        var address = toAddr(args[0]);
        int remaining = Integer.parseInt(args[1]);
        var listing = currentProgram.getListing();
        var functions = currentProgram.getFunctionManager();
        var instructions = listing.getInstructions(address, true);
        while (instructions.hasNext() && remaining-- > 0) {
            monitor.checkCancelled();
            var instruction = instructions.next();
            var function = functions.getFunctionContaining(instruction.getAddress());
            String label = function == null ? "<no function>" : function.getName();
            println(String.format(
                "%s %-18s %s",
                instruction.getAddress(),
                label,
                instruction
            ));
        }
    }
}
