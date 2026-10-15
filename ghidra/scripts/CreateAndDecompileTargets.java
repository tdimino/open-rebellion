// Recover and decompile selected function entry points that Ghidra did not
// discover automatically. Existing functions are left unchanged.
// Usage with analyzeHeadless:
//   -postScript CreateAndDecompileTargets.java OUTPUT_DIR 005d3e90
//@category OpenRebellion

import ghidra.app.decompiler.DecompInterface;
import ghidra.app.script.GhidraScript;
import ghidra.program.model.listing.Function;

import java.io.File;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;

public class CreateAndDecompileTargets extends GhidraScript {
    @Override
    public void run() throws Exception {
        String[] args = getScriptArgs();
        if (args.length < 2) {
            printerr("usage: CreateAndDecompileTargets.java OUTPUT_DIR ADDRESS [ADDRESS ...]");
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
            for (int index = 1; index < args.length; index++) {
                monitor.checkCancelled();
                String rawAddress = args[index].startsWith("FUN_")
                    ? args[index].substring(4)
                    : args[index];
                var address = toAddr(rawAddress);
                Function function = functionManager.getFunctionAt(address);
                if (function == null) {
                    function = createFunction(address, "FUN_" + rawAddress.toLowerCase());
                }
                if (function == null) {
                    printerr("could not recover function at " + address);
                    continue;
                }

                var result = decompiler.decompileFunction(function, 120, monitor);
                if (result == null || !result.decompileCompleted()) {
                    String error = result == null ? "unknown error" : result.getErrorMessage();
                    printerr("decompilation failed at " + address + ": " + error);
                    continue;
                }

                File output = new File(outputDir, function.getName() + ".c");
                Files.writeString(
                    output.toPath(),
                    result.getDecompiledFunction().getC(),
                    StandardCharsets.UTF_8
                );
                println("recovered " + function.getName() + " -> " + output);
            }
        } finally {
            decompiler.dispose();
        }
    }
}
