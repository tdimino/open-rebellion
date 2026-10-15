// Print the 32-bit float (and hex dword) stored at each given address.
// Usage with analyzeHeadless:
//   -postScript DumpFloatsAt.java ADDRESS [ADDRESS ...]
//@category OpenRebellion

import ghidra.app.script.GhidraScript;

public class DumpFloatsAt extends GhidraScript {
    @Override
    public void run() throws Exception {
        for (String arg : getScriptArgs()) {
            var address = toAddr(arg);
            int bits = currentProgram.getMemory().getInt(address);
            println(String.format("%s = %08x = %s", arg, bits, Float.intBitsToFloat(bits)));
        }
    }
}
