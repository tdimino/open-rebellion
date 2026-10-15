// Recover and decompile the functions inside the gaps that DecompileRange.java lists in gaps.tsv.
// Each gap is walked from its start: padding (0xcc, 0x90, 0x00 and MSVC's lea fillers) is
// skipped, a function is created at the next code byte, and the walk resumes after that
// function's body, so every function a gap holds is found, not only its first. Aligned dwords that point at code are
// switch-table entries and are stepped over. Writes FUN_xxxxxxxx.c per function and
// recovered.tsv: the index.tsv columns plus the gap the function was found in. Run read-only:
// the functions exist for this run only.
// Usage with analyzeHeadless:
//   -postScript RecoverGaps.java OUTPUT_DIR GAPS_TSV
//@category OpenRebellion

import ghidra.app.cmd.disassemble.DisassembleCommand;
import ghidra.app.decompiler.DecompInterface;
import ghidra.app.decompiler.DecompileResults;
import ghidra.app.script.GhidraScript;
import ghidra.program.model.address.Address;
import ghidra.program.model.data.StringDataInstance;
import ghidra.program.model.listing.Data;
import ghidra.program.model.listing.Function;
import ghidra.program.model.listing.Instruction;
import ghidra.program.model.mem.MemoryBlock;
import ghidra.program.model.symbol.Reference;

import java.io.File;
import java.io.PrintWriter;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.util.ArrayList;
import java.util.List;
import java.util.TreeSet;

public class RecoverGaps extends GhidraScript {
    @Override
    public void run() throws Exception {
        String[] args = getScriptArgs();
        if (args.length < 2) {
            printerr("usage: RecoverGaps.java OUTPUT_DIR GAPS_TSV");
            return;
        }
        File outputDir = new File(args[0]);
        if (!outputDir.exists() && !outputDir.mkdirs()) {
            printerr("could not create output directory: " + outputDir);
            return;
        }
        List<String> gapLines = Files.readAllLines(new File(args[1]).toPath());

        DecompInterface decompiler = new DecompInterface();
        decompiler.setSimplificationStyle("decompile");
        if (!decompiler.openProgram(currentProgram)) {
            printerr("could not open decompiler for " + currentProgram.getName());
            return;
        }

        var functionManager = currentProgram.getFunctionManager();
        var referenceManager = currentProgram.getReferenceManager();
        var listing = currentProgram.getListing();
        var memory = currentProgram.getMemory();
        int written = 0;
        try (PrintWriter index = new PrintWriter(new File(outputDir, "recovered.tsv"), StandardCharsets.UTF_8)) {
            index.println("entry\tbytes\tname\tcallers\tcaller_list\tcallees\tstrings\tgap");
            for (String line : gapLines) {
                String[] fields = line.split("\t");
                if (fields.length < 2 || fields[0].equals("from")) {
                    continue;
                }
                Address gapStart = toAddr(fields[0]);
                Address gapEnd = toAddr(fields[1]);
                Address position = gapStart;
                while (position.compareTo(gapEnd) < 0) {
                    monitor.checkCancelled();
                    if (isTableEntry(memory, listing, position)) {
                        position = position.add(4);
                        continue;
                    }
                    int value = memory.getByte(position) & 0xff;
                    if (value == 0xcc || value == 0x90 || value == 0x00) {
                        position = position.add(1);
                        continue;
                    }
                    int filler = fillerLength(memory, position);
                    if (filler == 0 && (memory.getByte(position) & 0xff) == 0x8b
                            && (memory.getByte(position.add(1)) & 0xff) == 0xff
                            && isTableEntry(memory, listing, position.add(2))) {
                        filler = 2; // mov edi,edi aligning a switch table
                    }
                    if (filler > 0) {
                        position = position.add(filler);
                        continue;
                    }
                    Function function = functionManager.getFunctionContaining(position);
                    if (function == null) {
                        new DisassembleCommand(position, null, true).applyTo(currentProgram, monitor);
                        function = createFunction(position, null);
                    }
                    if (function == null) {
                        printerr("no function at " + position + " in gap " + gapStart);
                        position = position.add(16 - (position.getOffset() & 15));
                        continue;
                    }
                    Address bodyEnd = function.getBody().getMaxAddress().add(1);
                    if (function.getEntryPoint().equals(position)) {
                        written += write(decompiler, function, outputDir, index, gapStart,
                            functionManager, referenceManager, listing);
                    }
                    position = bodyEnd.compareTo(position) > 0 ? bodyEnd : position.add(1);
                }
            }
        } finally {
            decompiler.dispose();
        }
        println("recovered " + written + " functions -> " + outputDir.getAbsolutePath());
    }

    // MSVC's multi-byte alignment fillers: lea esp,[esp]; lea ebx,[ebx]; lea ecx,[ecx].
    private static final int[][] FILLERS = {
        {0x8d, 0xa4, 0x24, 0x00, 0x00, 0x00, 0x00}, {0x8d, 0x9b, 0x00, 0x00, 0x00, 0x00},
        {0x8d, 0x64, 0x24, 0x00}, {0x8d, 0x49, 0x00},
    };

    private int fillerLength(ghidra.program.model.mem.Memory memory, Address position) {
        for (int[] filler : FILLERS) {
            boolean match = true;
            for (int i = 0; i < filler.length && match; i++) {
                try {
                    match = (memory.getByte(position.add(i)) & 0xff) == filler[i];
                } catch (Exception e) {
                    match = false;
                }
            }
            if (match) {
                return filler.length;
            }
        }
        return 0;
    }

    // A switch's jump table sits in a gap as aligned dwords holding code addresses; disassembling
    // it would invent functions. An aligned dword pointing at an instruction is a table entry.
    private boolean isTableEntry(ghidra.program.model.mem.Memory memory,
                                 ghidra.program.model.listing.Listing listing, Address position) {
        if ((position.getOffset() & 3) != 0) {
            return false;
        }
        try {
            Address target = toAddr(memory.getInt(position) & 0xffffffffL);
            MemoryBlock block = memory.getBlock(target);
            return block != null && block.isExecute()
                && (listing.getInstructionAt(target) != null || listing.getFunctionContaining(target) != null);
        } catch (Exception e) {
            return false;
        }
    }

    private int write(DecompInterface decompiler, Function function, File outputDir, PrintWriter index,
                      Address gap, ghidra.program.model.listing.FunctionManager functionManager,
                      ghidra.program.model.symbol.ReferenceManager referenceManager,
                      ghidra.program.model.listing.Listing listing) throws Exception {
        Address entry = function.getEntryPoint();
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
        int written = 0;
        DecompileResults result = decompiler.decompileFunction(function, 120, monitor);
        if (result != null && result.decompileCompleted()) {
            Files.writeString(new File(outputDir, name + ".c").toPath(),
                result.getDecompiledFunction().getC(), StandardCharsets.UTF_8);
            written = 1;
        } else {
            printerr("decompilation failed for " + name + ": "
                + (result == null ? "unknown decompiler error" : result.getErrorMessage()));
        }
        index.printf("%s\t%d\t%s\t%d\t%s\t%s\t%s\t%s%n", entry, function.getBody().getNumAddresses(), name,
            callers.size(), String.join(",", callers.stream().limit(12).toList()),
            String.join(",", callees), String.join(" | ", strings), gap);
        return written;
    }
}
