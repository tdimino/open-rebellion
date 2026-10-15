// List every instruction in executable memory with an immediate shaped like a DatId: a family
// byte in [FAMILY_LO, FAMILY_HI] above a record number in [1, MAX_RECORD]. This finds the
// record ids the code names literally (Luke 0x30000141, Coruscant 0x90000109), the evidence
// behind ghidra/notes/tables/hardcodedids.tsv.
// Output (tab-separated): value, family, address, function, instruction.
// Usage with analyzeHeadless:
//   -postScript FindDatIds.java OUTPUT_FILE [FAMILY_LO FAMILY_HI [MAX_RECORD]]
//   defaults: 0x08 0xfa 0xfff
//@category OpenRebellion

import ghidra.app.script.GhidraScript;
import ghidra.program.model.listing.Function;
import ghidra.program.model.listing.Instruction;
import ghidra.program.model.mem.MemoryBlock;
import ghidra.program.model.scalar.Scalar;

import java.io.File;
import java.io.PrintWriter;
import java.nio.charset.StandardCharsets;

public class FindDatIds extends GhidraScript {
    @Override
    public void run() throws Exception {
        String[] args = getScriptArgs();
        if (args.length < 1) {
            printerr("usage: FindDatIds.java OUTPUT_FILE [FAMILY_LO FAMILY_HI [MAX_RECORD]]");
            return;
        }
        long familyLo = args.length > 2 ? Long.decode(args[1]) : 0x08;
        long familyHi = args.length > 2 ? Long.decode(args[2]) : 0xfa;
        long maxRecord = args.length > 3 ? Long.decode(args[3]) : 0xfff;
        var listing = currentProgram.getListing();
        int hits = 0;
        try (PrintWriter out = new PrintWriter(new File(args[0]), StandardCharsets.UTF_8)) {
            out.println("value\tfamily\taddress\tfunction\tinstruction");
            for (MemoryBlock block : currentProgram.getMemory().getBlocks()) {
                if (!block.isExecute()) {
                    continue;
                }
                for (Instruction insn : listing.getInstructions(
                        new ghidra.program.model.address.AddressSet(block.getStart(), block.getEnd()), true)) {
                    monitor.checkCancelled();
                    for (int op = 0; op < insn.getNumOperands(); op++) {
                        Scalar scalar = insn.getScalar(op);
                        if (scalar == null || scalar.bitLength() > 32) {
                            continue;
                        }
                        long value = scalar.getUnsignedValue() & 0xffffffffL;
                        long family = value >>> 24;
                        long record = value & 0xffffff;
                        if (family < familyLo || family > familyHi || record == 0 || record > maxRecord) {
                            continue;
                        }
                        Function function = listing.getFunctionContaining(insn.getAddress());
                        out.printf("0x%08x\t0x%02x\t%s\t%s\t%s%n", value, family, insn.getAddress(),
                            function == null ? "?" : function.getName(), insn);
                        hits++;
                    }
                }
            }
        }
        println("wrote " + hits + " DatId-shaped immediates -> " + args[0]);
    }
}
