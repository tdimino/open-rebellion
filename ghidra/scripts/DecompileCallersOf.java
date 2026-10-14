// Decompile every function that references the named external symbols.
// Usage: -postScript DecompileCallersOf.java OUTPUT_DIR GetTickCount timeGetTime
//@category OpenRebellion

import ghidra.app.decompiler.DecompInterface;
import ghidra.app.script.GhidraScript;
import ghidra.program.model.listing.Function;
import ghidra.program.model.symbol.Reference;
import ghidra.program.model.symbol.Symbol;

import java.io.File;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.util.LinkedHashSet;
import java.util.Set;

public class DecompileCallersOf extends GhidraScript {
    @Override
    public void run() throws Exception {
        String[] args = getScriptArgs();
        File outputDir = new File(args[0]);
        outputDir.mkdirs();
        Set<Function> callers = new LinkedHashSet<>();
        for (int index = 1; index < args.length; index++) {
            for (Symbol symbol : currentProgram.getSymbolTable().getSymbols(args[index])) {
                for (Reference ref : getReferencesTo(symbol.getAddress())) {
                    Function caller = getFunctionContaining(ref.getFromAddress());
                    if (caller != null) {
                        callers.add(caller);
                    }
                    for (Reference thunkRef : getReferencesTo(ref.getFromAddress())) {
                        Function thunkCaller = getFunctionContaining(thunkRef.getFromAddress());
                        if (thunkCaller != null) {
                            callers.add(thunkCaller);
                        }
                    }
                }
            }
        }
        DecompInterface decompiler = new DecompInterface();
        decompiler.openProgram(currentProgram);
        for (Function fn : callers) {
            var result = decompiler.decompileFunction(fn, 60, monitor);
            if (result.decompileCompleted()) {
                Files.writeString(new File(outputDir, fn.getName() + ".c").toPath(),
                        result.getDecompiledFunction().getC(), StandardCharsets.UTF_8);
            }
        }
        println("CALLERS " + callers.size());
        decompiler.dispose();
    }
}
