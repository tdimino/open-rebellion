// Find instruction operands that contain selected scalar values.
// Usage with analyzeHeadless:
//   -postScript FindScalarReferences.java OUTPUT_FILE 3020 3040 3500
//@category OpenRebellion

import ghidra.app.script.GhidraScript;
import ghidra.program.model.listing.Function;
import ghidra.program.model.listing.Instruction;
import ghidra.program.model.scalar.Scalar;

import java.io.File;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.util.ArrayList;
import java.util.Collections;
import java.util.HashSet;
import java.util.List;
import java.util.Set;

public class FindScalarReferences extends GhidraScript {
    @Override
    public void run() throws Exception {
        String[] args = getScriptArgs();
        if (args.length < 2) {
            printerr("usage: FindScalarReferences.java OUTPUT_FILE SCALAR [SCALAR ...]");
            return;
        }

        Set<Long> targets = new HashSet<>();
        for (int index = 1; index < args.length; index++) {
            targets.add(Long.decode(args[index]));
        }

        List<String> matches = new ArrayList<>();
        var listing = currentProgram.getListing();
        var functions = currentProgram.getFunctionManager();
        var iterator = listing.getInstructions(true);
        while (iterator.hasNext()) {
            monitor.checkCancelled();
            Instruction instruction = iterator.next();
            for (int operand = 0; operand < instruction.getNumOperands(); operand++) {
                for (Object object : instruction.getOpObjects(operand)) {
                    if (!(object instanceof Scalar)) {
                        continue;
                    }
                    Scalar scalar = (Scalar) object;
                    long value = scalar.getUnsignedValue();
                    if (!targets.contains(value)) {
                        continue;
                    }
                    Function function = functions.getFunctionContaining(instruction.getAddress());
                    String functionLabel = function == null
                        ? "<no-function>"
                        : function.getName() + " " + function.getEntryPoint();
                    matches.add(String.format(
                        "%d 0x%x %s %s :: %s",
                        value,
                        value,
                        instruction.getAddress(),
                        functionLabel,
                        instruction
                    ));
                }
            }
        }

        Collections.sort(matches);
        File output = new File(args[0]);
        Files.writeString(
            output.toPath(),
            matches.isEmpty() ? "" : String.join("\n", matches) + "\n",
            StandardCharsets.UTF_8
        );
        println("wrote " + matches.size() + " scalar references to " + output.getAbsolutePath());
    }
}
