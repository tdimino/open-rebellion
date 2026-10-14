// Print a bounded pointer table with resolved function names.
// Usage with analyzeHeadless:
//   -postScript DumpPointerTable.java ADDRESS ENTRY_COUNT OUTPUT_FILE
//@category OpenRebellion

import ghidra.app.script.GhidraScript;

import java.io.File;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;

public class DumpPointerTable extends GhidraScript {
    @Override
    public void run() throws Exception {
        String[] args = getScriptArgs();
        if (args.length != 3) {
            printerr("usage: DumpPointerTable.java ADDRESS ENTRY_COUNT OUTPUT_FILE");
            return;
        }

        var start = toAddr(args[0]);
        int entryCount = Integer.decode(args[1]);
        var memory = currentProgram.getMemory();
        var functionManager = currentProgram.getFunctionManager();
        StringBuilder output = new StringBuilder();

        for (int index = 0; index < entryCount; index++) {
            var slot = start.add((long) index * 4);
            long rawTarget = Integer.toUnsignedLong(memory.getInt(slot));
            var target = toAddr(rawTarget);
            var function = functionManager.getFunctionAt(target);
            output.append(String.format(
                "[%02d] %s -> %s %s%n",
                index,
                slot,
                target,
                function == null ? "<not-function>" : function.getName()
            ));
        }

        File destination = new File(args[2]);
        Files.writeString(destination.toPath(), output.toString(), StandardCharsets.UTF_8);
        println("wrote " + destination);
    }
}
