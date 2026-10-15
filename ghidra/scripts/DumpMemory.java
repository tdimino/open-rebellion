// Print bounded bytes and common little-endian interpretations at one address.
// Usage with analyzeHeadless:
//   -postScript DumpMemory.java ADDRESS LENGTH
//@category OpenRebellion

import ghidra.app.script.GhidraScript;

import java.nio.ByteBuffer;
import java.nio.ByteOrder;

public class DumpMemory extends GhidraScript {
    @Override
    public void run() throws Exception {
        String[] args = getScriptArgs();
        if (args.length != 2) {
            printerr("usage: DumpMemory.java ADDRESS LENGTH");
            return;
        }

        var address = toAddr(args[0]);
        int length = Integer.parseInt(args[1]);
        if (length < 1 || length > 256) {
            printerr("length must be between 1 and 256");
            return;
        }
        byte[] bytes = new byte[length];
        currentProgram.getMemory().getBytes(address, bytes);
        StringBuilder hex = new StringBuilder();
        for (byte value : bytes) {
            if (!hex.isEmpty()) {
                hex.append(' ');
            }
            hex.append(String.format("%02x", value & 0xff));
        }
        println(address + " bytes=" + hex);
        if (length >= 4) {
            ByteBuffer buffer = ByteBuffer.wrap(bytes).order(ByteOrder.LITTLE_ENDIAN);
            println(address + " u32=" + Integer.toUnsignedString(buffer.getInt(0))
                + " f32=" + Float.intBitsToFloat(buffer.getInt(0)));
        }
        if (length >= 8) {
            ByteBuffer buffer = ByteBuffer.wrap(bytes).order(ByteOrder.LITTLE_ENDIAN);
            println(address + " f64=" + buffer.getDouble(0));
        }
    }
}
