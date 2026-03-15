from ghidra.program.util import DefinedDataIterator

outpath = "/Users/tomdimino/Desktop/rebellion-strings.txt"
keywords = ['combat','battle','damage','attack','shield','weapon','fleet','hull',
            'fire','turbo','laser','bombard','missile','torpedo','hit point','armor',
            'victory','defeat','death star','gnprt','sdprt','mission','research',
            'jedi','force','rebel','empire','uprising','blockade','probe','sensor',
            'diplomacy','espionage','sabotage','assassin','recruit','rescue','abduct',
            'loyalty','betray','popular','headquarter']

f = open(outpath, "w")
count = 0
for s in DefinedDataIterator.definedStrings(currentProgram):
    try:
        val = s.getValue()
        if val is None:
            continue
        low = str(val).lower()
        for kw in keywords:
            if kw in low:
                f.write("%s: %s\n" % (s.getAddress(), val))
                count += 1
                break
    except:
        pass

f.close()
print("Done! %d strings written to %s" % (count, outpath))
