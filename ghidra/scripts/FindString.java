// Find each text in memory (ASCII and UTF-16LE) and print every match with the functions
// that reference it. Use it to go from a string the game shows or logs to the code using it.
// Usage with analyzeHeadless:
//   -postScript FindString.java "Invalid CommandKind value" CharacterCommandKindNotif
//@category OpenRebellion

import ghidra.app.script.GhidraScript;
import ghidra.program.model.address.Address;
import ghidra.program.model.listing.Function;
import ghidra.program.model.symbol.Reference;

import java.nio.charset.StandardCharsets;

public class FindString extends GhidraScript {
    @Override
    public void run() throws Exception {
        String[] args = getScriptArgs();
        if (args.length < 1) {
            printerr("usage: FindString.java TEXT [TEXT ...]");
            return;
        }
        for (String text : args) {
            find(text, "ascii", text.getBytes(StandardCharsets.US_ASCII));
            find(text, "utf16", text.getBytes(StandardCharsets.UTF_16LE));
        }
    }

    private void find(String text, String encoding, byte[] bytes) throws Exception {
        var memory = currentProgram.getMemory();
        Address position = currentProgram.getMinAddress();
        while (position != null) {
            monitor.checkCancelled();
            Address match = memory.findBytes(position, bytes, null, true, monitor);
            if (match == null) {
                return;
            }
            println("STR " + encoding + " \"" + text + "\" @ " + match);
            for (Reference reference : getReferencesTo(match)) {
                Function function = getFunctionContaining(reference.getFromAddress());
                println("  REF <- " + reference.getFromAddress() + " in "
                    + (function == null ? "?" : function.getName() + " @ " + function.getEntryPoint()));
            }
            position = match.add(1);
        }
    }
}
