// Decompile every function whose entry lies in [START, END), for mapping an address range.
// Skips Unwind@ funclets and thunks. Writes FUN_xxxxxxxx.c per function and index.tsv:
// entry, body bytes, name, caller count, callers, callees, referenced strings; and gaps.tsv:
// spans of 32 bytes or more between function bodies that hold no function (undefined code
// such as a refresh Ghidra never defined; recover it with CreateAndDecompileTargets.java).
// Usage with analyzeHeadless:
//   -postScript DecompileRange.java OUTPUT_DIR 0x005e0000 0x005f0000 [MIN_BYTES]
//@category OpenRebellion

import ghidra.app.decompiler.DecompInterface;
import ghidra.app.decompiler.DecompileResults;
import ghidra.app.script.GhidraScript;
import ghidra.program.model.address.Address;
import ghidra.program.model.data.StringDataInstance;
import ghidra.program.model.listing.Data;
import ghidra.program.model.listing.Function;
import ghidra.program.model.listing.Instruction;
import ghidra.program.model.symbol.Reference;

import java.io.File;
import java.io.PrintWriter;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.util.ArrayList;
import java.util.List;
import java.util.TreeSet;

public class DecompileRange extends GhidraScript {
    @Override
    public void run() throws Exception {
        String[] args = getScriptArgs();
        if (args.length < 3) {
            printerr("usage: DecompileRange.java OUTPUT_DIR START END [MIN_BYTES]");
            return;
        }
        File outputDir = new File(args[0]);
        if (!outputDir.exists() && !outputDir.mkdirs()) {
            printerr("could not create output directory: " + outputDir);
            return;
        }
        Address start = toAddr(args[1]);
        Address end = toAddr(args[2]);
        long minBytes = args.length > 3 ? Long.parseLong(args[3]) : 0;

        DecompInterface decompiler = new DecompInterface();
        decompiler.setSimplificationStyle("decompile");
        if (!decompiler.openProgram(currentProgram)) {
            printerr("could not open decompiler for " + currentProgram.getName());
            return;
        }

        var functionManager = currentProgram.getFunctionManager();
        var referenceManager = currentProgram.getReferenceManager();
        var listing = currentProgram.getListing();
        int written = 0;
        Address covered = start;
        try (PrintWriter index = new PrintWriter(new File(outputDir, "index.tsv"), StandardCharsets.UTF_8);
             PrintWriter gaps = new PrintWriter(new File(outputDir, "gaps.tsv"), StandardCharsets.UTF_8)) {
            index.println("entry\tbytes\tname\tcallers\tcaller_list\tcallees\tstrings");
            gaps.println("from\tto\tbytes\tfirst_instruction");
            for (Function function : functionManager.getFunctions(start, true)) {
                monitor.checkCancelled();
                Address entry = function.getEntryPoint();
                if (entry.compareTo(end) >= 0) {
                    break;
                }
                if (entry.subtract(covered) >= 32) {
                    Instruction first = listing.getInstructionAfter(covered);
                    gaps.printf("%s\t%s\t%d\t%s%n", covered, entry, entry.subtract(covered),
                        first != null && first.getAddress().compareTo(entry) < 0
                            ? first.getAddress() + " " + first : "no instructions");
                }
                Address bodyEnd = function.getBody().getMaxAddress().add(1);
                if (bodyEnd.compareTo(covered) > 0) {
                    covered = bodyEnd;
                }
                long bytes = function.getBody().getNumAddresses();
                if (function.isThunk() || function.getName().startsWith("Unwind@") || bytes < minBytes) {
                    continue;
                }

                TreeSet<String> callers = new TreeSet<>();
                for (Reference reference : referenceManager.getReferencesTo(entry)) {
                    Function caller = functionManager.getFunctionContaining(reference.getFromAddress());
                    callers.add(caller == null ? reference.getFromAddress().toString() : caller.getName());
                }
                TreeSet<String> callees = new TreeSet<>();
                for (Function callee : function.getCalledFunctions(monitor)) {
                    callees.add(callee.getName());
                }
                List<String> strings = new ArrayList<>();
                for (Instruction instruction : listing.getInstructions(function.getBody(), true)) {
                    for (Reference reference : instruction.getReferencesFrom()) {
                        Data data = listing.getDataAt(reference.getToAddress());
                        if (data == null || !data.hasStringValue()) {
                            continue;
                        }
                        String text = StringDataInstance.getStringDataInstance(data).getStringValue();
                        if (text != null && strings.size() < 8) {
                            strings.add(text.replaceAll("[\\t\\r\\n]", " "));
                        }
                    }
                }

                String name = function.getName();
                DecompileResults result = decompiler.decompileFunction(function, 120, monitor);
                if (result != null && result.decompileCompleted()) {
                    Files.writeString(new File(outputDir, name + ".c").toPath(),
                        result.getDecompiledFunction().getC(), StandardCharsets.UTF_8);
                    written++;
                } else {
                    printerr("decompilation failed for " + name + ": "
                        + (result == null ? "unknown decompiler error" : result.getErrorMessage()));
                }
                index.printf("%s\t%d\t%s\t%d\t%s\t%s\t%s%n", entry, bytes, name, callers.size(),
                    String.join(",", callers.stream().limit(12).toList()),
                    String.join(",", callees), String.join(" | ", strings));
            }
        } finally {
            decompiler.dispose();
        }
        println("decompiled " + written + " functions in " + start + "-" + end + " -> " + outputDir.getAbsolutePath());
    }
}
