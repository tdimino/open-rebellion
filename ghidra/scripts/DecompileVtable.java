// Decompile the functions referenced by the first N slots of a vtable.
// Usage: -postScript DecompileVtable.java OUTPUT_DIR VTABLE_ADDRESS COUNT
//@category OpenRebellion

import ghidra.app.decompiler.DecompInterface;
import ghidra.app.script.GhidraScript;
import ghidra.program.model.address.Address;
import ghidra.program.model.listing.Function;

import java.io.File;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;

public class DecompileVtable extends GhidraScript {
    @Override
    public void run() throws Exception {
        String[] args = getScriptArgs();
        File outputDir = new File(args[0]);
        outputDir.mkdirs();
        Address table = toAddr(args[1]);
        int count = Integer.decode(args[2]);
        DecompInterface decompiler = new DecompInterface();
        decompiler.openProgram(currentProgram);
        StringBuilder index = new StringBuilder();
        for (int slot = 0; slot < count; slot++) {
            Address target = toAddr(getInt(table.add(slot * 4L)) & 0xffffffffL);
            Function fn = getFunctionAt(target);
            if (fn == null) {
                fn = createFunction(target, null);
            }
            index.append(String.format("slot %d (+0x%x) -> %s%n", slot, slot * 4, target));
            if (fn == null) {
                continue;
            }
            var result = decompiler.decompileFunction(fn, 60, monitor);
            if (result.decompileCompleted()) {
                Files.writeString(new File(outputDir, "VT_" + args[1] + "_" + slot + "_" + fn.getName() + ".c").toPath(),
                        result.getDecompiledFunction().getC(), StandardCharsets.UTF_8);
            }
        }
        Files.writeString(new File(outputDir, "VT_" + args[1] + "_index.txt").toPath(), index.toString(),
                StandardCharsets.UTF_8);
        decompiler.dispose();
    }
}
