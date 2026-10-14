// Find indirect CALL instructions using a selected vtable displacement.
// Usage with analyzeHeadless:
//   -postScript FindIndirectCalls.java OUTPUT_FILE DISPLACEMENT START END
//@category OpenRebellion

import ghidra.app.script.GhidraScript;
import ghidra.program.model.listing.Instruction;
import ghidra.program.model.scalar.Scalar;

import java.io.File;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.util.ArrayList;
import java.util.List;

public class FindIndirectCalls extends GhidraScript {
    @Override
    public void run() throws Exception {
        String[] args = getScriptArgs();
        if (args.length != 4) {
            printerr("usage: FindIndirectCalls.java OUTPUT_FILE DISPLACEMENT START END");
            return;
        }

        long displacement = Long.decode(args[1]);
        var start = toAddr(args[2]);
        var end = toAddr(args[3]);
        var listing = currentProgram.getListing();
        var functions = currentProgram.getFunctionManager();
        var iterator = listing.getInstructions(start, true);
        List<String> matches = new ArrayList<>();

        while (iterator.hasNext()) {
            monitor.checkCancelled();
            Instruction instruction = iterator.next();
            if (instruction.getAddress().compareTo(end) > 0) {
                break;
            }
            if (!"CALL".equals(instruction.getMnemonicString())) {
                continue;
            }

            boolean matched = false;
            for (int operand = 0; operand < instruction.getNumOperands() && !matched; operand++) {
                for (Object object : instruction.getOpObjects(operand)) {
                    if (object instanceof Scalar
                        && ((Scalar) object).getUnsignedValue() == displacement) {
                        matched = true;
                        break;
                    }
                }
            }
            if (!matched) {
                continue;
            }

            var function = functions.getFunctionContaining(instruction.getAddress());
            String functionLabel = function == null
                ? "<no-function>"
                : function.getName() + " " + function.getEntryPoint();
            matches.add(instruction.getAddress() + " " + functionLabel + " :: " + instruction);
        }

        File output = new File(args[0]);
        Files.writeString(
            output.toPath(),
            matches.isEmpty() ? "" : String.join("\n", matches) + "\n",
            StandardCharsets.UTF_8
        );
        println("wrote " + matches.size() + " indirect calls to " + output);
    }
}
