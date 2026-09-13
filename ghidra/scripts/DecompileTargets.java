// Decompile selected functions from the current Ghidra program.
// Usage with analyzeHeadless:
//   -postScript DecompileTargets.java OUTPUT_DIR FUN_005d9eb0 FUN_005da080
//@category OpenRebellion

import ghidra.app.decompiler.DecompInterface;
import ghidra.app.decompiler.DecompileResults;
import ghidra.app.script.GhidraScript;
import ghidra.program.model.address.Address;
import ghidra.program.model.listing.Function;

import java.io.File;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

public class DecompileTargets extends GhidraScript {
    private Address targetAddress(String target) {
        return toAddr(target.startsWith("FUN_") ? target.substring(4) : target);
    }

    @Override
    public void run() throws Exception {
        String[] args = getScriptArgs();
        if (args.length < 2) {
            printerr("usage: DecompileTargets.java OUTPUT_DIR FUNCTION [FUNCTION ...]");
            return;
        }

        File outputDir = new File(args[0]);
        if (!outputDir.exists() && !outputDir.mkdirs()) {
            printerr("could not create output directory: " + outputDir);
            return;
        }

        DecompInterface decompiler = new DecompInterface();
        decompiler.setSimplificationStyle("decompile");
        if (!decompiler.openProgram(currentProgram)) {
            printerr("could not open decompiler for " + currentProgram.getName());
            return;
        }

        try {
            var functionManager = currentProgram.getFunctionManager();
            var referenceManager = currentProgram.getReferenceManager();
            for (int index = 1; index < args.length; index++) {
                monitor.checkCancelled();
                String target = args[index];
                Function function = functionManager.getFunctionAt(targetAddress(target));
                if (function == null) {
                    printerr("function not found: " + target);
                    continue;
                }

                DecompileResults result = decompiler.decompileFunction(function, 120, monitor);
                if (result == null || !result.decompileCompleted()) {
                    String error = result == null ? "unknown decompiler error" : result.getErrorMessage();
                    printerr("decompilation failed for " + target + ": " + error);
                    continue;
                }

                Files.writeString(
                    new File(outputDir, target + ".c").toPath(),
                    result.getDecompiledFunction().getC(),
                    StandardCharsets.UTF_8
                );

                List<String> callers = new ArrayList<>();
                for (var reference : referenceManager.getReferencesTo(function.getEntryPoint())) {
                    Function caller = functionManager.getFunctionContaining(reference.getFromAddress());
                    if (caller != null) {
                        callers.add(caller.getName() + " " + caller.getEntryPoint());
                    }
                }
                Collections.sort(callers);
                Files.writeString(
                    new File(outputDir, target + ".callers.txt").toPath(),
                    callers.isEmpty() ? "" : String.join("\n", callers) + "\n",
                    StandardCharsets.UTF_8
                );

                List<String> callees = new ArrayList<>();
                for (Function callee : function.getCalledFunctions(monitor)) {
                    callees.add(callee.getName() + " " + callee.getEntryPoint());
                }
                Collections.sort(callees);
                Files.writeString(
                    new File(outputDir, target + ".callees.txt").toPath(),
                    callees.isEmpty() ? "" : String.join("\n", callees) + "\n",
                    StandardCharsets.UTF_8
                );
                println("decompiled " + target + " -> " + outputDir.getAbsolutePath());
            }
        } finally {
            decompiler.dispose();
        }
    }
}
