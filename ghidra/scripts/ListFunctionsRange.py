# List functions whose entry points fall within an address interval.
# Usage: -postScript ListFunctionsRange.py START_ADDRESS END_ADDRESS
#@category OpenRebellion

args = getScriptArgs()
if len(args) != 2:
    printerr("usage: ListFunctionsRange.py START_ADDRESS END_ADDRESS")
    exit()

start = toAddr(args[0])
end = toAddr(args[1])
functions = currentProgram.getFunctionManager().getFunctions(start, True)
while functions.hasNext():
    function = functions.next()
    entry = function.getEntryPoint()
    if entry.compareTo(end) > 0:
        break
    println("%s %s %d" % (entry, function.getName(), function.getBody().getNumAddresses()))
