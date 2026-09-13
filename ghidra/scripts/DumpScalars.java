// Print exact little-endian scalar values from named addresses.
// Usage with analyzeHeadless:
//   -postScript DumpScalars.java f32:0066c958 f32:0066d400
//@category OpenRebellion

import ghidra.app.script.GhidraScript;

public class DumpScalars extends GhidraScript {
    @Override
    public void run() throws Exception {
        for (String spec : getScriptArgs()) {
            String[] parts = spec.split(":", 2);
            if (parts.length != 2) {
                printerr("invalid scalar specification: " + spec);
                continue;
            }

            String kind = parts[0];
            var address = toAddr(parts[1]);
            if ("f64".equals(kind)) {
                long bits = getLong(address);
                println(String.format(
                    "%s bits=0x%016x value=%s",
                    spec,
                    bits,
                    Double.longBitsToDouble(bits)
                ));
                continue;
            }

            int signedBits = getInt(address);
            long bits = Integer.toUnsignedLong(signedBits);
            String value;
            switch (kind) {
                case "f32":
                    value = Float.toString(Float.intBitsToFloat(signedBits));
                    break;
                case "u32":
                    value = Long.toString(bits);
                    break;
                case "i32":
                    value = Integer.toString(signedBits);
                    break;
                default:
                    printerr("unsupported scalar kind: " + kind);
                    continue;
            }
            println(String.format("%s bits=0x%08x value=%s", spec, bits, value));
        }
    }
}
