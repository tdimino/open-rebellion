// For every call to the given functions, print the caller, the pushed arguments and the
// instructions before the call. Arguments are the PUSH operands read backwards from the call,
// nearest first (argument 1 first), stopping at the previous CALL or the caller's entry; a MOV
// to ECX is listed as "this". The scan is linear, so pushes on two branches that join before
// the call both appear (FUN_0044f860's tracks 10 and 11). Use it to read a registry built by
// repeated calls, such as FUN_00540540's 58 family registrations in FUN_0051ebf0.
// Output (tab-separated): target, call site, caller, this, args, preceding instructions.
// Usage with analyzeHeadless:
//   -postScript CallSiteArgs.java OUTPUT_FILE BEFORE FUN_00540540 [0x00417610 ...]
//@category OpenRebellion

import ghidra.app.script.GhidraScript;
import ghidra.program.model.address.Address;
import ghidra.program.model.listing.Function;
import ghidra.program.model.listing.Instruction;
import ghidra.program.model.symbol.Reference;

import java.io.File;
import java.io.PrintWriter;
import java.nio.charset.StandardCharsets;
import java.util.ArrayList;
import java.util.List;

public class CallSiteArgs extends GhidraScript {
    @Override
    public void run() throws Exception {
        String[] args = getScriptArgs();
        if (args.length < 3) {
            printerr("usage: CallSiteArgs.java OUTPUT_FILE BEFORE TARGET [TARGET ...]");
            return;
        }
        int before = Integer.parseInt(args[1]);
        var listing = currentProgram.getListing();
        int calls = 0;
        try (PrintWriter out = new PrintWriter(new File(args[0]), StandardCharsets.UTF_8)) {
            out.println("target\tcall_site\tcaller\tthis\targs\tbefore");
            for (int i = 2; i < args.length; i++) {
                Address target = toAddr(args[i].startsWith("FUN_") ? args[i].substring(4) : args[i]);
                List<Address> sites = new ArrayList<>();
                for (Reference reference : getReferencesTo(target)) {
                    if (reference.getReferenceType().isCall()) {
                        sites.add(reference.getFromAddress());
                    }
                }
                sites.sort(null);
                for (Address site : sites) {
                    monitor.checkCancelled();
                    Instruction call = listing.getInstructionAt(site);
                    if (call == null) {
                        continue;
                    }
                    List<String> pushes = new ArrayList<>();
                    String thisOperand = "";
                    List<String> preceding = new ArrayList<>();
                    boolean collecting = true;
                    Function caller = listing.getFunctionContaining(site);
                    Instruction insn = call.getPrevious();
                    for (int n = 0; n < before && insn != null; n++, insn = insn.getPrevious()) {
                        if (caller != null && !caller.getBody().contains(insn.getAddress())) {
                            break;
                        }
                        preceding.add(0, insn.getAddress() + " " + insn);
                        String mnemonic = insn.getMnemonicString();
                        if (mnemonic.equals("CALL")) {
                            collecting = false;
                        }
                        if (!collecting) {
                            continue;
                        }
                        if (mnemonic.equals("PUSH")) {
                            pushes.add(insn.getDefaultOperandRepresentation(0));
                        } else if (thisOperand.isEmpty() && mnemonic.equals("MOV")
                                && insn.getDefaultOperandRepresentation(0).equals("ECX")) {
                            thisOperand = insn.getDefaultOperandRepresentation(1);
                        }
                    }
                    out.printf("%s\t%s\t%s\t%s\t%s\t%s%n", args[i], site,
                        caller == null ? "?" : caller.getName(), thisOperand,
                        String.join(",", pushes), String.join(" ; ", preceding));
                    calls++;
                }
            }
        }
        println("wrote " + calls + " call sites -> " + args[0]);
    }
}
