// List every family extraction `SHR reg,0x18` and what the code does with the family byte in
// the next AFTER instructions: a "compare" against literals (CMP or SUB with an immediate, as
// in a range test 0x14..0x1b), an "index" into a table (a scaled-index memory operand), or
// "other". Use it to audit where the original keys a rule on a DAT family.
// Output (tab-separated): address, function, kind, literals, following instructions.
// Usage with analyzeHeadless:
//   -postScript FamilyChecks.java OUTPUT_FILE [AFTER]   (AFTER defaults to 10)
//@category OpenRebellion

import ghidra.app.script.GhidraScript;
import ghidra.program.model.listing.Function;
import ghidra.program.model.listing.Instruction;
import ghidra.program.model.scalar.Scalar;

import java.io.File;
import java.io.PrintWriter;
import java.nio.charset.StandardCharsets;
import java.util.ArrayList;
import java.util.List;

public class FamilyChecks extends GhidraScript {
    @Override
    public void run() throws Exception {
        String[] args = getScriptArgs();
        if (args.length < 1) {
            printerr("usage: FamilyChecks.java OUTPUT_FILE [AFTER]");
            return;
        }
        int after = args.length > 1 ? Integer.parseInt(args[1]) : 10;
        var listing = currentProgram.getListing();
        int hits = 0;
        try (PrintWriter out = new PrintWriter(new File(args[0]), StandardCharsets.UTF_8)) {
            out.println("address\tfunction\tkind\tliterals\tafter");
            for (Instruction insn : listing.getInstructions(true)) {
                monitor.checkCancelled();
                if (!insn.getMnemonicString().equals("SHR") || insn.getNumOperands() != 2) {
                    continue;
                }
                Scalar shift = insn.getScalar(1);
                if (shift == null || shift.getUnsignedValue() != 0x18) {
                    continue;
                }
                String kind = "other";
                List<String> literals = new ArrayList<>();
                List<String> following = new ArrayList<>();
                Instruction next = insn.getNext();
                for (int n = 0; n < after && next != null; n++, next = next.getNext()) {
                    String text = next.toString();
                    following.add(text);
                    String mnemonic = next.getMnemonicString();
                    if ((mnemonic.equals("CMP") || mnemonic.equals("SUB")) && next.getNumOperands() == 2) {
                        Scalar literal = next.getScalar(1);
                        if (literal != null) {
                            literals.add(String.format("0x%x", literal.getUnsignedValue()));
                            if (kind.equals("other")) {
                                kind = "compare";
                            }
                        }
                    } else if (text.contains("*0x") && text.contains("[") && kind.equals("other")) {
                        kind = "index";
                    }
                    if (mnemonic.equals("RET") || mnemonic.equals("CALL")) {
                        break;
                    }
                }
                Function function = listing.getFunctionContaining(insn.getAddress());
                out.printf("%s\t%s\t%s\t%s\t%s%n", insn.getAddress(),
                    function == null ? "?" : function.getName(), kind,
                    String.join(",", literals), String.join(" ; ", following));
                hits++;
            }
        }
        println("wrote " + hits + " family extractions -> " + args[0]);
    }
}
