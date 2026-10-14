// Print every instruction whose operands contain one of the given scalars.
// Usage: -postScript FindScalarUses.java 0x8600 0x8604
//@category OpenRebellion

import ghidra.app.script.GhidraScript;
import ghidra.program.model.listing.Function;
import ghidra.program.model.listing.Instruction;
import ghidra.program.model.scalar.Scalar;

import java.util.HashSet;
import java.util.Set;

public class FindScalarUses extends GhidraScript {
    @Override
    public void run() throws Exception {
        Set<Long> wanted = new HashSet<>();
        for (String arg : getScriptArgs()) {
            wanted.add(Long.decode(arg));
        }
        var listing = currentProgram.getListing();
        for (Instruction insn : listing.getInstructions(true)) {
            monitor.checkCancelled();
            for (int op = 0; op < insn.getNumOperands(); op++) {
                for (Object obj : insn.getOpObjects(op)) {
                    if (obj instanceof Scalar scalar && wanted.contains(scalar.getUnsignedValue())) {
                        Function fn = listing.getFunctionContaining(insn.getAddress());
                        println("HIT " + insn.getAddress() + " " + (fn == null ? "?" : fn.getName())
                                + " " + insn);
                    }
                }
            }
        }
    }
}
