# Print exact little-endian scalar values from named addresses.
# Usage with analyzeHeadless:
#   -postScript DumpScalars.py f32:0066c958 f32:0066d400
#@category OpenRebellion

from java.lang import Double, Float


for spec in getScriptArgs():
    kind, raw_address = spec.split(":", 1)
    address = toAddr(raw_address)
    if kind == "f64":
        signed_long = getLong(address)
        bits = signed_long & 0xffffffffffffffff
        value = Double.longBitsToDouble(signed_long)
        println("%s bits=0x%016x value=%s" % (spec, bits, value))
        continue

    signed_bits = getInt(address)
    bits = signed_bits & 0xffffffff
    if kind == "f32":
        value = Float.intBitsToFloat(signed_bits)
    elif kind == "u32":
        value = bits
    elif kind == "i32":
        value = bits if bits < 0x80000000 else bits - 0x100000000
    else:
        printerr("unsupported scalar kind: %s" % kind)
        continue
    println("%s bits=0x%08x value=%s" % (spec, bits, value))
