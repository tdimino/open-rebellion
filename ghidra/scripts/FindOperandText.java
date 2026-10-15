// Print instructions in [START, END) whose text contains NEEDLE, plus callers of each hit function.
// Usage: -postScript FindOperandText.java START END NEEDLE
//@category OpenRebellion

import ghidra.app.script.GhidraScript;
import ghidra.program.model.listing.Function;
import ghidra.program.model.listing.Instruction;
import ghidra.program.model.symbol.Reference;

import java.util.LinkedHashSet;
import java.util.Set;

public class FindOperandText extends GhidraScript {
    @Override
    public void run() throws Exception {
        String[] args = getScriptArgs();
        var start = toAddr(args[0]);
        var end = toAddr(args[1]);
        String needle = args[2];
        var listing = currentProgram.getListing();
        Set<Function> functions = new LinkedHashSet<>();
        for (Instruction insn : listing.getInstructions(start, true)) {
            if (insn.getAddress().compareTo(end) >= 0) {
                break;
            }
            if (insn.toString().contains(needle)) {
                Function fn = listing.getFunctionContaining(insn.getAddress());
                println("HIT " + insn.getAddress() + " " + (fn == null ? "?" : fn.getName()) + " " + insn);
                if (fn != null) {
                    functions.add(fn);
                }
            }
        }
        for (Function fn : functions) {
            for (Reference ref : getReferencesTo(fn.getEntryPoint())) {
                Function caller = listing.getFunctionContaining(ref.getFromAddress());
                println("CALLER " + fn.getName() + " <- " + ref.getFromAddress() + " "
                        + (caller == null ? "?" : caller.getName()));
            }
        }
    }
}
