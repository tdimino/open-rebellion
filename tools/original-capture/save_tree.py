#!/usr/bin/env python3
"""Read an original-game save as its object tree, and move one record
between containers.

    save_tree.py check SAVE.sav [SAVE.sav ...]
    save_tree.py jail  SOURCE.sav OUTPUT.sav
    save_tree.py assassin SOURCE.sav OUTPUT.sav
    save_tree.py garrison contested|repulsed|contested-visible SOURCE.sav OUTPUT.sav
    save_tree.py maintenance empire DELTA SOURCE.sav OUTPUT.sav
    save_tree.py undefended SOURCE.sav OUTPUT.sav

`check` parses each save and rebuilds it from the parse (every list count
from its objects, every marker from its position); it fails unless the
rebuilt bytes equal the file, or unless cutting one character out of its
list and putting it back rebuilds the file too. `jail` is pinned to two
library saves: in `alliance-day731-imperial-deathstar-built.sav` it moves
Leia (`0x31000241`), an Imperial captive at Coruscant, into the Empire Jail
(`0x98000480`); in `empire-day660-deathstar-pre-alert.sav` it moves
Palpatine (`0x34000280`), an Alliance captive at `0x9000010a`, into the
Alliance Jail (`0x98000440`). It does so in every graph that holds the
character, never overwrites, and re-parses its output to prove that only
that move changed. The layout and the
acceptance gates are in `agent_docs/original-save-editing.md` ("Recipe 5: a
container change"); an edit is a hypothesis until the original loads,
re-saves and reloads it.

`assassin` is pinned to `empire-sixteen-tie-day415.sav`: it sets the effective
combat of one free Imperial minor (Daala, `0x38000393`, 84) to 220, Vader's,
so that an Assassination of Han Solo (`0x33000243`, combat 100, free at
Carida, `0x900000ad`, listed in the Empire's Carida window) rolls the top
ASSNMSTB row (difference 100 and over: 55 percent). The roll reads slot
`+0x1f0` (`FUN_004edc40`), the short at object `+0x86` (`FUN_004ef940`
writes the 16 skill shorts `+0x7c`..`+0x9a` in order, so it is the high half
of master word 31 in the stream, the object's words counted from the child
list's first field word). It writes that half and nothing else, and
re-parses its output to prove it.

`garrison` is pinned to `alliance-day1169-deathstar-trench-run.sav`: it moves
one Imperial regiment from `0x92000068` into Imperial-held Ord Pardon
(`0x920000a5`), where the Alliance fleet in orbit (Fleet `0xc2c`, six
regiments) can then assault a defender. `contested` moves Imperial Army
Regiment `0x100009db` (attack 3, defence 5); `repulsed` moves Dark Trooper
Regiment `0x10001eea` (defence 8). The move is made in the master copy and the
Empire's own view (the two graphs whose tree holds Ord Pardon's list); the
Alliance's copy sits in the unlocated bucket (`0xf2`), which the script
requires and leaves alone. The master location word becomes Ord Pardon.

`maintenance` takes any Empire save: it raises the Empire's maintenance
capacity by DELTA. The capacity is recomputed from the facilities
(`FUN_0052fff0` sums the completed, enabled mines' and refineries' capacity
pairs into side `+0x60` and `+0x68`, and the change hook `FUN_005323c0` sets
side `+0x58` to the smaller), and it runs at every destroy and usable-bit
change, so a raised side word alone would be reset. The edit adds DELTA to
the capacity word of the lowest-id plain (flags `0x30047`), completed, enabled mine and
refinery at Coruscant (`0x90000109`, the Empire's reserved capital, which a
capture cannot strip from the Empire while the Empire holds it), or, when
Coruscant holds no such pair, at the Empire system holding the most of them
(the lowest key on a tie), and keeps every
stored sum consistent: the Empire side object's (`0xf3000010`) `+0x58`,
`+0x60` and `+0x68` capacities, in the master copy and in the Empire's own
view when the save holds one. It checks the sums before writing, writes
nothing else, re-parses its output to prove it, and prints the facilities
and side words it changed.

The format, from the loader:

- An object is read by its container's child loop `FUN_0053a1e0`: a u32
  family (the class tag `FUN_00540500` looks up in the registry
  `FUN_0051ebf0` fills), the object's own fields (vtable `+0x14`: id
  `+0x18`, graph `+0x24`, class `+0x2c`, ...), then the object's own child
  list.
- A child list is a u32 flag (0: no list); then, when set, a u32 the list
  object keeps (`FUN_005841a0`, list `+0xc`), a u32 count, and, when the
  count is not 0, a u32 marker before the first child.
- The marker is `FUN_005f4d40`: the stream's position (`FUN_00616d70`),
  checked on read. It is the only position-dependent word: `FUN_00616d70`
  has no other caller, and the section loaders (`FUN_00409480`,
  `FUN_00409700`, `FUN_0041dd60`, `FUN_0051cec0`) write the same kind of
  marker. Moving bytes therefore needs every marker after the move
  rewritten to its new position, and nothing else.
- Children are parented by their place in the list (`FUN_0060f5d0` stores
  the container at child `+0x1c`), not by their location word.
"""

import bisect
import collections
import hashlib
import struct
import sys
from pathlib import Path

LIBRARY = Path(__file__).resolve().parent / "saves"
# source SHA-256 and size -> (character key, jail abode key)
JAIL_EDITS = {
    ("502ac1cbdd49b9d46e2b340122f9b3565a49fa09f8e409fb0e6d8ae25ed67904", 589_112): (0x31000241, 0x98000480),
    ("5973cf91e8ee5cd3684c1dc83966c2d049d456081b7623a09f64d7f43b778de1", 1_040_089): (0x34000280, 0x98000440),
}
# source SHA-256 and size -> (agent key, its system, old combat, new combat,
# target key, the target's system): a free minor, the combat it is raised to,
# and the enemy major it should assassinate at a system its side sees
ASSASSIN_EDITS = {
    ("1f8381e59acf166390195bd5cc47e0811f66dd68065c4a2d7dfa29ad072f9baa", 855_040): (0x38000393, 0x900000AB, 84, 220, 0x33000243, 0x900000AD),
}
# source SHA-256 and size -> (character key, its system, the destination):
# a free major moved to a system of its own side that holds no regiment, no
# squadron and only empty fleets, so a mission against it meets no defender
# (`ghidra/notes/decoy-roll.md`, the decoy and FOILTB phases walk regiments,
# and ships and fighters when the mission's side has nothing there)
UNDEFENDED_EDITS = {
    # `assassin`'s output: Han Solo from Carida to 0x90000097
    ("bf5ef29b23fde7c6cce42efa31a389b35eba664489eb4647a82abc8f8926c212", 855_040): (0x33000243, 0x900000AD, 0x90000097),
}
# source SHA-256 and size -> edit name -> (regiment key, its system, the
# destination system, the graph the Alliance's bucket copy is re-keyed to,
# or None to leave it in the bucket)
GARRISON_EDITS = {
    ("ad68f6f4d049a74362e4b57c1573a960e12fd3d5b938d47d81babec7c45dd9a0", 1_396_732): {
        "contested": (0x100009DB, 0x92000068, 0x920000A5, None),
        "repulsed": (0x10001EEA, 0x92000068, 0x920000A5, None),
        # the Alliance's own copy, in the unlocated bucket, is re-keyed to
        # graph 0x91 and moved into Ord Pardon's list of that tree, the
        # Alliance's view of the system, so its report lists the defender
        "contested-visible": (0x100009DB, 0x92000068, 0x920000A5, 0x91),
    },
}
# The Empire's side object (`0xf3000010`) and Coruscant, the literal capital
# (`ghidra/notes/scenario-literals.md`)
EMPIRE_SIDE_ID = 0x10
CORUSCANT = 0x90000109
# Side object words, master copy and view copy: `+0x58` capacity, `+0x60`
# mines' capacity, `+0x68` refineries' capacity (the stream keeps the side's
# pairs in offset order, `+0x58` at master word 12 and view word 7). A
# facility's capacity pair (`+0x64`) is master words 19 and 20, view 10 and 11.
SIDE_CAPACITY_WORDS = {"master": (12, 14, 16), "view": (7, 9, 11)}
FACILITY_CAPACITY_WORD = {"master": 19, "view": 10}
# Master word 31 holds effective combat (`+0x86`) in its high half; word 24
# is the mission key (`+0x68`, 2 when none) and word 41 the flags (`+0xac`,
# bit 0 captive); word 35's low half is injury (`+0x94`).
COMBAT_WORD = 31
MISSION_WORD = 24
FLAGS_WORD = 41
INJURY_WORD = 35
# A master copy's location word (fields word 9, `save_edit.py`'s payload
# word 9): the system, fleet or ship holding the object.
LOCATION_WORD = 9

# The 58 families `FUN_0051ebf0` registers with `FUN_00540540`.
FAMILIES = {
    0x08, 0x10, 0x14, 0x18, 0x1C, 0x20, 0x22, 0x23, 0x24, 0x25, 0x28, 0x29,
    0x2A, 0x2C, 0x2D, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x38, 0x3C, 0x41,
    0x42, 0x43, 0x44, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x61,
    0x62, 0x63, 0x64, 0x65, 0x69, 0x6A, 0x71, 0x72, 0x73, 0x80, 0x90, 0x92,
    0x98, 0xA0, 0xA2, 0xA4, 0xF1, 0xF2, 0xF3, 0xF8, 0xF9, 0xFA,
}


def require(condition, detail="layout check failed"):
    """An assert that `python3 -O` cannot remove: every check here guards
    a write into an original save."""
    if not condition:
        raise SystemExit(f"refusing: {detail}")


def u32(data, offset):
    return struct.unpack_from("<I", data, offset)[0]


def header_length(data, at):
    """The length of the object header at `at` (family, id, graph, class,
    named flag, and the name when the flag is set), or 0 when `at` holds no
    object header."""
    if at + 20 > len(data):
        return 0
    family, ident, graph, _cls, named = struct.unpack_from("<5I", data, at)
    if family not in FAMILIES or ident >= 1 << 24 or graph >> 8 not in (0x800000, 0x800001):
        return 0
    if named == 0:
        return 20
    if named != 1 or at + 22 > len(data):
        return 0
    length = struct.unpack_from("<H", data, at + 20)[0]
    name = data[at + 22 : at + 22 + length]
    if length > 80 or len(name) != length or not all(32 <= b < 127 for b in name):
        return 0
    return 22 + length


def view_root(data, at):
    """True when `at` starts a view root: the object `FUN_00513a50` reads
    without a family word before the list of a graph's sectors (id, graph,
    class, named flag 0, then seven words, `FUN_004f9340`)."""
    if at < 4 or at + 44 > len(data) or header_length(data, at - 4):
        return False
    ident, graph, _cls, named = struct.unpack_from("<4I", data, at)
    return ident < 0x100 and graph >> 8 == 0x800000 and graph & 0x30 != 0 and named == 0


class Scan:
    """Every object header, view root and self-referencing word of a save."""

    def __init__(self, data):
        self.data = data
        self.markers = set()
        self.headers = {}
        for at in range(0, len(data) - 3):
            if u32(data, at) == at:
                self.markers.add(at)
            length = header_length(data, at)
            if length:
                self.headers[at] = length
        self.list_markers = {
            m for m in self.markers if m + 4 in self.headers and m >= 12 and u32(data, m - 4) > 0
        }
        roots = {m - 56 for m in self.list_markers if view_root(data, m - 56)}
        self.boundaries = sorted(set(self.headers) | roots)

    def next_boundary(self, after):
        i = bisect.bisect_left(self.boundaries, after)
        return self.boundaries[i] if i < len(self.boundaries) else len(self.data)

    def kind(self, at):
        return u32(self.data, at), u32(self.data, at + 8) & 0x130

    def sizes(self):
        """For each object whose end the next object fixes, its field
        length, and the shorter length that holds when the bytes before the
        next object are an empty list (flag, kept word, count 0) instead of
        the flag 0 alone."""
        seen = collections.defaultdict(list)
        data = self.data
        for at, length in self.headers.items():
            body = at + length
            nxt = self.next_boundary(body)
            if nxt >= len(data):
                continue
            if nxt - 4 in self.list_markers:
                size, empty = nxt - 16 - body, None
            else:
                size = nxt - 4 - body
                empty = size - 8 if size >= 8 and u32(data, nxt - 12) and u32(data, nxt - 8) and not u32(data, nxt - 4) else None
            if size >= 0:
                seen[self.kind(at)].append((size, empty))
        return seen


_LENGTHS = None


def library_lengths():
    """The field length of each fixed (family, graph view) kind, pooled over
    the library saves: the length every object of the kind has, read with
    its empty list where one stands before the next object. Missions (`0x41`-`0x69`),
    the build managers (`0xa0`-`0xa4`, whose queue entries carry names) and
    the master copy of `0xf3` vary and get none."""
    global _LENGTHS
    if _LENGTHS is None:
        pooled = collections.defaultdict(list)
        for path in sorted(LIBRARY.glob("*.sav")):
            for kind, sizes in Scan(path.read_bytes()).sizes().items():
                pooled[kind].extend(sizes)
        _LENGTHS = {}
        for kind, sizes in pooled.items():
            size = collections.Counter(s for s, _ in sizes).most_common(1)[0][0]
            if all(size in (s, empty) for s, empty in sizes):
                _LENGTHS[kind] = size
        # The readers branch only on master against view (`+0x24 & 0x30`,
        # `FUN_004f9340`, `FUN_005621b0`), so the two views of a kind read
        # the same fields: a view never seen ending before another object
        # (the side object `0xfa` closes the last graph) takes the other's.
        for (family, graph), size in list(_LENGTHS.items()):
            if graph & 0x30 and (family, graph ^ 0x30) not in pooled:
                _LENGTHS[(family, graph ^ 0x30)] = size
    return _LENGTHS


class Chunk:
    """Bytes copied from the save, with the original offset they came from,
    so that the markers among them can be rewritten after a move."""

    def __init__(self, data, start, end):
        self.start, self.bytes = start, data[start:end]


class Node:
    """One object: its header and fields as copied bytes, and its child list."""

    def __init__(self, header, fields, children, parent):
        self.header, self.fields, self.children, self.parent = header, fields, children, parent

    start = property(lambda self: self.header.start)
    family = property(lambda self: u32(self.header.bytes, 0))
    ident = property(lambda self: u32(self.header.bytes, 4))
    graph = property(lambda self: u32(self.header.bytes, 8))

    def word(self, index):
        return u32(self.fields.bytes, 4 * index)

    def set_word(self, index, value):
        raw = bytearray(self.fields.bytes)
        struct.pack_into("<I", raw, 4 * index, value)
        self.fields.bytes = bytes(raw)

    def key(self):
        return (self.family << 24) | self.ident


class ChildList:
    """A list as `FUN_0053a1e0` reads it: flag, kept word, count, marker."""

    def __init__(self, start, flag, kept, nodes, owner):
        self.start, self.flag, self.kept, self.nodes, self.owner = start, flag, kept, nodes, owner


class Parse:
    """A save as ordered segments: copied bytes and the graphs' trees. A
    rebuild writes each list's count from its nodes and every marker from
    its new position."""

    def __init__(self, data):
        self.data = data
        self.scan = Scan(data)
        self.lengths = library_lengths()
        self.used = set()
        self.tree_ends = []  # objects followed by bytes outside the tree
        self.segments = []
        at = 0
        for marker in sorted(self.scan.list_markers):
            if marker - 12 < at:
                continue
            self.segments.append(Chunk(data, at, marker - 12))
            child_list, at = self.parse_list(marker - 12, None)
            self.segments.append(child_list)
        self.segments.append(Chunk(data, at, len(data)))

    def parse_list(self, at, owner):
        data = self.data
        flag = u32(data, at)
        if flag == 0:
            return None, at + 4
        kept, count = u32(data, at + 4), u32(data, at + 8)
        child_list = ChildList(at, flag, kept, [], owner)
        at += 12
        if count:
            require(at in self.scan.list_markers, f"no marker at {at:#x}")
            self.used.add(at)
            at += 4
            for _ in range(count):
                node, at = self.parse_node(at, child_list)
                child_list.nodes.append(node)
        return child_list, at

    def parse_node(self, at, parent):
        data, scan = self.data, self.scan
        length = scan.headers.get(at)
        require(length, f"no object header at {at:#x}")
        body = at + length
        nxt = scan.next_boundary(body)
        usual = self.lengths.get(scan.kind(at))
        if nxt - 4 in scan.list_markers and nxt in scan.headers and nxt - 16 >= body:
            # The object's own list holds the next object as its first child.
            fields_end = nxt - 16
            require(usual is None or fields_end - body == usual, f"object at {at:#x}: {fields_end - body} field bytes, usual {usual}")
        elif usual is not None:
            # A kind with a fixed field length: its list flag (0) or an
            # empty list (flag, kept word, count 0) follows.
            fields_end = body + usual
        else:
            # A kind of varying length (missions, managers with queues) ends
            # where the next object starts, after its list flag.
            fields_end = nxt - 4
            require(u32(data, fields_end) == 0, f"object at {at:#x}: no list flag before {nxt:#x}")
        node = Node(Chunk(data, at, body), Chunk(data, body, fields_end), None, parent)
        node.children, end = self.parse_list(fields_end, node)
        if not (node.children and node.children.nodes):
            # A leaf ends at the next object, or ends its tree.
            require(end <= nxt, f"object at {at:#x} runs past {nxt:#x}")
            if end != nxt:
                self.tree_ends.append(node)
        return node, end

    def nodes(self):
        stack = [s for s in self.segments if isinstance(s, ChildList)]
        while stack:
            child_list = stack.pop()
            for node in child_list.nodes:
                yield node
                if node.children is not None:
                    stack.append(node.children)

    def find(self, family, ident):
        return [n for n in self.nodes() if n.family == family and n.ident == ident]

    def rebuild(self):
        out = bytearray()
        markers = self.scan.markers

        def emit(chunk):
            at = len(out)
            out.extend(chunk.bytes)
            for offset in range(len(chunk.bytes) - 3):
                if chunk.start + offset in markers:
                    struct.pack_into("<I", out, at + offset, at + offset)

        def emit_list(child_list):
            if child_list is None:
                out.extend(struct.pack("<I", 0))
                return
            out.extend(struct.pack("<3I", child_list.flag, child_list.kept, len(child_list.nodes)))
            if child_list.nodes:
                out.extend(struct.pack("<I", len(out)))
                for node in child_list.nodes:
                    emit(node.header)
                    emit(node.fields)
                    emit_list(node.children)

        for segment in self.segments:
            if isinstance(segment, Chunk):
                emit(segment)
            else:
                emit_list(segment)
        return bytes(out)


def tree_listing(parse):
    """Each object as (graph, its container's key, header bytes, field
    bytes), for comparing two parses."""
    rows = collections.Counter()
    for node in parse.nodes():
        owner = node.parent.owner
        rows[(node.graph, owner.key() if owner else None, node.header.bytes, node.fields.bytes)] += 1
    return rows


def identity_move(parse):
    """Cut the first character out of its list and put it back."""
    node = next(n for n in parse.nodes() if 0x30 <= n.family <= 0x3B)
    siblings = node.parent.nodes
    index = siblings.index(node)
    siblings.pop(index)
    siblings.insert(index, node)
    return parse.rebuild()


def summary(data):
    parse = Parse(data)
    nodes = list(parse.nodes())
    trees = sum(1 for s in parse.segments if isinstance(s, ChildList))
    unparsed = len(parse.scan.list_markers - parse.used)
    outside = len(set(parse.scan.headers) - {n.start for n in nodes})
    loose = len(parse.scan.markers - parse.used)
    return parse, (
        f"{len(data)} bytes, {trees} trees, {len(nodes)} objects, {unparsed} list markers unparsed, "
        f"{outside} headers outside the trees, {loose} other markers, {len(parse.tree_ends)} tree ends"
    )


def check(paths):
    ok = True
    for path in paths:
        data = Path(path).read_bytes()
        try:
            parse, line = summary(data)
        except SystemExit as failure:
            print(f"{Path(path).name}: {failure}")
            ok = False
            continue
        same = parse.rebuild() == data
        moved = identity_move(parse) == data
        ok &= same and moved
        print(
            f"{Path(path).name}: {'identical' if same else 'DIFFERS'}, identity move "
            f"{'identical' if moved else 'DIFFERS'}, {line}, sha256 {hashlib.sha256(data).hexdigest()[:16]}"
        )
    return ok


def relocate(node, target, index=None):
    """Take `node` out of its list and put it into `target`'s child list, at
    the end or at `index`; the caller rewrites any word that names the
    container."""
    node.parent.nodes.remove(node)
    if target.children is None:
        target.children = ChildList(None, 1, 1, [], target)
    target.children.nodes.insert(len(target.children.nodes) if index is None else index, node)
    node.parent = target.children


def jail(source, output):
    """Move a captive out of its system's child list into a jail abode in
    every graph, as `FUN_00549580`'s trip leaves it once the `0x387` timer
    has fired (`ghidra/notes/jail.md`, section 1): the abode's child, its
    master location word the abode's key."""
    data = Path(source).read_bytes()
    pin = (hashlib.sha256(data).hexdigest(), len(data))
    require(pin in JAIL_EDITS, f"{source} is not a pinned source")
    require(not Path(output).exists(), f"{output} exists")
    character, abode_key = JAIL_EDITS[pin]
    parse = Parse(data)
    require(parse.rebuild() == data, "the source does not rebuild")
    before = tree_listing(parse)
    copies = parse.find(character >> 24, character & 0xFFFFFF)
    abodes = {n.graph: n for n in parse.find(abode_key >> 24, abode_key & 0xFFFFFF)}
    require(copies, "no copy of the character")
    expected = collections.Counter()
    for node in copies:
        system = node.parent.owner
        require(system is not None and system.family in (0x90, 0x92), f"copy at {node.start:#x} is not in a system")
        abode = abodes.get(node.graph)
        require(abode is not None, f"no abode in graph {node.graph:#x}")
        # FUN_005775a0: a jail holds occupants of its own owner side.
        require((node.graph ^ abode.graph) & 0xC0 == 0, f"graph {node.graph:#x}: owner differs from the abode's")
        require(abode.children is None or not abode.children.nodes, f"abode in {node.graph:#x} already holds objects")
        old = (node.graph, system.key(), node.header.bytes, node.fields.bytes)
        if node.graph & 0x30 == 0:
            require(node.word(LOCATION_WORD) == system.key(), f"master copy: location {node.word(LOCATION_WORD):#x}")
            require(node.word(17) & 1, "master copy: no prisoner bit")
            node.set_word(LOCATION_WORD, abode_key)
        relocate(node, abode)
        expected[old] -= 1
        expected[(node.graph, abode_key, node.header.bytes, node.fields.bytes)] += 1
    out = parse.rebuild()
    reread = Parse(out)
    require(reread.rebuild() == out, "the output does not rebuild")
    after = tree_listing(reread)
    removed, added = before - after, after - before
    require(removed == collections.Counter({k: -v for k, v in expected.items() if v < 0}), "objects other than the captive changed")
    require(added == collections.Counter({k: v for k, v in expected.items() if v > 0}), "the captive did not move as intended")
    require(len(reread.scan.markers - reread.used) == len(parse.scan.markers - parse.used), "the section markers changed in number")
    Path(output).write_bytes(out)
    print(
        f"{output}: {len(copies)} copies of {character:#010x} moved into {abode_key:#010x}, "
        f"{len(out) - len(data):+d} bytes, sha256 {hashlib.sha256(out).hexdigest()}"
    )


def assassin(source, output):
    """Raise one free agent's effective combat so that an Assassination of a
    major rolls the top ASSNMSTB row (`ghidra/notes/decoy-roll.md`: the roll
    reads member combat minus target combat, slot `+0x1f0`, short `+0x86`)."""
    data = Path(source).read_bytes()
    pin = (hashlib.sha256(data).hexdigest(), len(data))
    require(pin in ASSASSIN_EDITS, f"{source} is not a pinned source")
    require(not Path(output).exists(), f"{output} exists")
    agent_key, agent_system, old, new, target_key, target_system = ASSASSIN_EDITS[pin]
    parse = Parse(data)
    require(parse.rebuild() == data, "the source does not rebuild")
    before = tree_listing(parse)

    def master(key):
        copies = [n for n in parse.find(key >> 24, key & 0xFFFFFF) if n.graph & 0x30 == 0]
        require(len(copies) == 1, f"{key:#010x}: {len(copies)} master copies")
        return copies[0]

    agent, target = master(agent_key), master(target_key)
    for node, system in ((agent, agent_system), (target, target_system)):
        container = node.parent.owner
        require(container is not None and container.key() == system, f"{node.key():#010x} is not in {system:#010x}")
        require(node.word(LOCATION_WORD) == system, f"{node.key():#010x}: location {node.word(LOCATION_WORD):#x}")
        require(node.word(MISSION_WORD) == 2, f"{node.key():#010x} is on a mission")
        require(not node.word(FLAGS_WORD) & 1, f"{node.key():#010x} is a captive")
        require(node.word(INJURY_WORD) & 0xFFFF == 0, f"{node.key():#010x} is injured")
    # Different sides: the owner bits of the two graphs differ.
    require((agent.graph ^ target.graph) & 0xC0 != 0, "agent and target share a side")
    require(agent.word(COMBAT_WORD) >> 16 == old, f"agent combat {agent.word(COMBAT_WORD) >> 16}, expected {old}")
    old_row = (agent.graph, agent_system, agent.header.bytes, agent.fields.bytes)
    agent.set_word(COMBAT_WORD, (new << 16) | (agent.word(COMBAT_WORD) & 0xFFFF))
    new_row = (agent.graph, agent_system, agent.header.bytes, agent.fields.bytes)
    out = parse.rebuild()
    require(len(out) == len(data), "the output changed length")
    differing = [i for i in range(len(data)) if data[i] != out[i]]
    require(len(differing) == 1 and out[differing[0]] == new, f"bytes differ at {differing}")
    reread = Parse(out)
    require(reread.rebuild() == out, "the output does not rebuild")
    after = tree_listing(reread)
    require(before - after == collections.Counter({old_row: 1}), "objects other than the agent changed")
    require(after - before == collections.Counter({new_row: 1}), "the agent did not change as intended")
    Path(output).write_bytes(out)
    print(
        f"{output}: {agent_key:#010x} combat {old} -> {new} at byte {differing[0]:#x}, target {target_key:#010x} "
        f"(combat {target.word(COMBAT_WORD) >> 16}) at {target_system:#010x}, sha256 {hashlib.sha256(out).hexdigest()}"
    )


def undefended(source, output):
    """Move a free character between two systems of its side in every graph
    that holds it: the destination's list, before the first object of a
    later family, and the master copy's location word."""
    data = Path(source).read_bytes()
    pin = (hashlib.sha256(data).hexdigest(), len(data))
    require(pin in UNDEFENDED_EDITS, f"{source} is not a pinned source")
    require(not Path(output).exists(), f"{output} exists")
    character, origin, destination = UNDEFENDED_EDITS[pin]
    parse = Parse(data)
    require(parse.rebuild() == data, "the source does not rebuild")
    before = tree_listing(parse)
    copies = parse.find(character >> 24, character & 0xFFFFFF)
    targets = {n.graph: n for n in parse.find(destination >> 24, destination & 0xFFFFFF)}
    require(copies, "no copy of the character")
    expected = collections.Counter()
    for node in copies:
        container = node.parent.owner
        require(container is not None and container.key() == origin, f"copy {node.graph:#x} is not in {origin:#010x}")
        target = targets.get(node.graph)
        require(target is not None and target.children is not None, f"no destination list in graph {node.graph:#x}")
        require((node.graph ^ target.graph) & 0xC0 == 0, f"graph {node.graph:#x}: owner differs from the destination's")
        for child in target.children.nodes:
            require(not (0x10 <= child.family <= 0x13 or child.family == 0x1C), f"graph {node.graph:#x}: the destination holds {child.key():#010x}")
            require(child.family != 0x08 or not (child.children and child.children.nodes), f"graph {node.graph:#x}: fleet {child.key():#010x} is not empty")
        old = (node.graph, origin, node.header.bytes, node.fields.bytes)
        if node.graph & 0x30 == 0:
            require(node.word(LOCATION_WORD) == origin, f"master copy: location {node.word(LOCATION_WORD):#x}")
            require(node.word(MISSION_WORD) == 2, "master copy: on a mission")
            require(not node.word(FLAGS_WORD) & 1, "master copy: a captive")
            node.set_word(LOCATION_WORD, destination)
        slot = next((i for i, c in enumerate(target.children.nodes) if c.family > node.family), len(target.children.nodes))
        relocate(node, target, slot)
        expected[old] -= 1
        expected[(node.graph, destination, node.header.bytes, node.fields.bytes)] += 1
    out = parse.rebuild()
    reread = Parse(out)
    require(reread.rebuild() == out, "the output does not rebuild")
    after = tree_listing(reread)
    require(before - after == collections.Counter({k: -v for k, v in expected.items() if v < 0}), "objects other than the character changed")
    require(after - before == collections.Counter({k: v for k, v in expected.items() if v > 0}), "the character did not move as intended")
    require(len(reread.scan.markers - reread.used) == len(parse.scan.markers - parse.used), "the section markers changed in number")
    Path(output).write_bytes(out)
    print(
        f"{output}: {len(copies)} copies of {character:#010x} moved from {origin:#010x} to {destination:#010x}, "
        f"{len(out) - len(data):+d} bytes, sha256 {hashlib.sha256(out).hexdigest()}"
    )


def regiment_slot(system):
    """Where a regiment goes in a system's list: the lists keep fleets first,
    then regiments, then facilities."""
    return max((i + 1 for i, c in enumerate(system.children.nodes) if c.family == 0x08), default=0)


def garrison(edit, source, output):
    """Move one regiment between two systems, as a landing or a regroup
    leaves it: the system's child list in each graph that holds both, the
    master copy's location word the new system's key. A copy in an
    unlocated bucket (`0xf2`, the other side's view that does not know where
    the regiment is) stays there."""
    data = Path(source).read_bytes()
    pin = (hashlib.sha256(data).hexdigest(), len(data))
    require(pin in GARRISON_EDITS and edit in GARRISON_EDITS[pin], f"{source} is not a pinned source for {edit}")
    require(not Path(output).exists(), f"{output} exists")
    regiment, origin, destination, view = GARRISON_EDITS[pin][edit]
    parse = Parse(data)
    require(parse.rebuild() == data, "the source does not rebuild")
    before = tree_listing(parse)
    copies = parse.find(regiment >> 24, regiment & 0xFFFFFF)
    targets = {n.graph: n for n in parse.find(destination >> 24, destination & 0xFFFFFF)}
    require(copies, "no copy of the regiment")
    expected = collections.Counter()
    moved = 0
    for node in copies:
        container = node.parent.owner
        if container is not None and container.family == 0xF2:
            if view is None:
                continue
            # an unlocated copy becomes the Alliance's view of the system
            target = targets.get(0x80000000 | view)
            require(target is not None and target.children is not None, f"no destination list in graph {view:#x}")
            require(node.graph == 0x80000092, f"bucket copy is in graph {node.graph:#x}")
            require((node.graph ^ target.graph) & 0xC0 == 0, f"graph {node.graph:#x}: owner differs from the destination's")
            old = (node.graph, container.key(), node.header.bytes, node.fields.bytes)
            # graph is header word 2; Node.graph reads it, so the expected row below carries the new graph
            raw = bytearray(node.header.bytes)
            struct.pack_into("<I", raw, 8, 0x80000000 | view)
            node.header.bytes = bytes(raw)
            relocate(node, target, regiment_slot(target))
            expected[old] -= 1
            expected[(node.graph, destination, node.header.bytes, node.fields.bytes)] += 1
            moved += 1
            continue
        require(container is not None and container.key() == origin, f"copy {node.graph:#x} is not in {origin:#010x}")
        target = targets.get(node.graph)
        require(target is not None and target.children is not None, f"no destination list in graph {node.graph:#x}")
        # the destination is held by the regiment's own side: the graph's owner bits (`+0x24` bits 6-7)
        require((node.graph ^ target.graph) & 0xC0 == 0, f"graph {node.graph:#x}: owner differs from the destination's")
        require(not any(0x10 <= c.family <= 0x13 for c in target.children.nodes), f"graph {node.graph:#x}: the destination holds a regiment")
        old = (node.graph, container.key(), node.header.bytes, node.fields.bytes)
        if node.graph & 0x30 == 0:
            require(node.word(LOCATION_WORD) == origin, f"master copy: location {node.word(LOCATION_WORD):#x}")
            require(node.word(0) == 0x30147, f"master copy: state {node.word(0):#x}")
            node.set_word(LOCATION_WORD, destination)
        relocate(node, target, regiment_slot(target))
        expected[old] -= 1
        expected[(node.graph, destination, node.header.bytes, node.fields.bytes)] += 1
        moved += 1
    require(moved == (2 if view is None else 3), f"{moved} copies moved")
    out = parse.rebuild()
    reread = Parse(out)
    require(reread.rebuild() == out, "the output does not rebuild")
    after = tree_listing(reread)
    require(before - after == collections.Counter({k: -v for k, v in expected.items() if v < 0}), "objects other than the regiment changed")
    require(after - before == collections.Counter({k: v for k, v in expected.items() if v > 0}), "the regiment did not move as intended")
    require(len(reread.scan.markers - reread.used) == len(parse.scan.markers - parse.used), "the section markers changed in number")
    Path(output).write_bytes(out)
    print(
        f"{output}: {moved} copies of {regiment:#010x} moved from {origin:#010x} to {destination:#010x}, "
        f"{len(out) - len(data):+d} bytes, sha256 {hashlib.sha256(out).hexdigest()}"
    )


def maintenance(side, delta, source, output):
    """Raise the Empire's maintenance capacity by `delta`: the capacity word
    of one mine and one refinery at Coruscant, and the side object's three
    capacity words, in the master copy and the Empire's own view."""
    require(side == "empire", f"{side}: only the Empire is supported")
    require(delta > 0, "the delta must be positive")
    data = Path(source).read_bytes()
    require(not Path(output).exists(), f"{output} exists")
    parse = Parse(data)
    require(parse.rebuild() == data, "the source does not rebuild")
    before = tree_listing(parse)
    own = 0x80000081  # the Empire's master copy; its own view is 0xa1

    def row(node):
        return (node.graph, node.parent.owner.key() if node.parent.owner else None, node.header.bytes, node.fields.bytes)

    def copies(family, ident):
        """The master copy and, when the save holds it, the Empire's view."""
        found = {n.graph: n for n in parse.find(family, ident) if n.graph in (own, own | 0x20)}
        require(own in found, f"{family:#x}:{ident:#x}: no master copy")
        return [("master", found[own])] + ([("view", found[own | 0x20])] if own | 0x20 in found else [])

    edits = []  # (node, word index, old value, new value, label)
    master = dict(copies(0xF3, EMPIRE_SIDE_ID))["master"]
    for kind, node in copies(0xF3, EMPIRE_SIDE_ID):
        capacity, mines, refineries = SIDE_CAPACITY_WORDS[kind]
        values = [node.word(i) for i in (capacity, mines, refineries)]
        require(values[0] == min(values[1], values[2]), f"side {kind}: capacity {values[0]} is not min({values[1]}, {values[2]})")
        for i, name in zip((capacity, mines, refineries), ("+0x58", "+0x60", "+0x68")):
            edits.append((node, i, node.word(i), node.word(i) + delta, f"side 0xf3000010 {kind} {name}"))
    # the master side sums must equal the master facilities' capacities
    totals = {0x2C: 0, 0x2D: 0}
    by_system = collections.defaultdict(lambda: {0x2C: [], 0x2D: []})
    for node in parse.nodes():
        if node.family not in totals or node.graph != own:
            continue
        completed = node.word(0) & 1 and node.word(0) & 0x40
        if completed and node.word(4) & 1 == 0:
            totals[node.family] += node.word(FACILITY_CAPACITY_WORD["master"])
            owner = node.parent.owner
            # a plain facility (flags 0x30047) at a system holding both kinds
            if owner is not None and node.word(0) == 0x30047:
                by_system[owner.key()][node.family].append(node)
    both = {k: v for k, v in by_system.items() if v[0x2C] and v[0x2D]}
    # Coruscant when it has both kinds, else the Empire system holding the
    # most of them, the lowest key on a tie
    site = CORUSCANT if CORUSCANT in both else min(both, key=lambda k: (-len(both[k][0x2C]) - len(both[k][0x2D]), k), default=None)
    require(site is not None, "no Empire system holds a plain completed mine and refinery")
    chosen = {family: min(both[site][family], key=lambda n: n.ident) for family in totals}
    require(totals[0x2C] == master.word(SIDE_CAPACITY_WORDS["master"][1]), "the mines' capacities do not sum to the side's word")
    require(totals[0x2D] == master.word(SIDE_CAPACITY_WORDS["master"][2]), "the refineries' capacities do not sum to the side's word")
    print(f"  facilities at {site:#010x}" + (" (Coruscant)" if site == CORUSCANT else " (Coruscant holds no mine and refinery pair)"))
    for family, name in ((0x2C, "mine"), (0x2D, "refinery")):
        for kind, node in copies(family, chosen[family].ident):
            word = FACILITY_CAPACITY_WORD[kind]
            require(node.word(word) == chosen[family].word(FACILITY_CAPACITY_WORD["master"]), f"{name} {node.key():#010x}: {kind} copy differs from the master")
            edits.append((node, word, node.word(word), node.word(word) + delta, f"{name} {node.key():#010x} {kind} capacity"))
    old_capacity = master.word(SIDE_CAPACITY_WORDS["master"][0])
    touched = {}
    for node, *_ in edits:
        touched.setdefault(id(node), (node, row(node)))
    offsets = []
    for node, index, old, new, label in edits:
        require(node.word(index) == old, f"word {index} of {node.key():#010x} changed under the edit")
        node.set_word(index, new)
        offsets.append(node.fields.start + 4 * index)
    expected = collections.Counter()
    for node, old_row in touched.values():
        expected[old_row] -= 1
        expected[row(node)] += 1
    out = parse.rebuild()
    require(len(out) == len(data), "the output changed length")
    differing = [i for i in range(len(data)) if data[i] != out[i]]
    require(set(differing) <= {o + b for o in offsets for b in range(4)}, f"bytes differ outside the edited words: {differing[:8]}")
    reread = Parse(out)
    require(reread.rebuild() == out, "the output does not rebuild")
    after = tree_listing(reread)
    require(before - after == collections.Counter({k: -v for k, v in expected.items() if v < 0}), "objects other than the edited ones changed")
    require(after - before == collections.Counter({k: v for k, v in expected.items() if v > 0}), "the edited objects did not change as intended")
    Path(output).write_bytes(out)
    for (node, index, old, new, label), offset in zip(edits, offsets):
        print(f"  {label}: {old} -> {new} at {offset:#x}")
    print(f"{output}: {side} capacity {old_capacity} -> {old_capacity + delta}, {len(offsets)} words, "
          f"sha256 {hashlib.sha256(out).hexdigest()}")


def main(argv):
    if len(argv) >= 3 and argv[1] == "check":
        sys.exit(0 if check(argv[2:]) else 1)
    if len(argv) == 4 and argv[1] == "jail":
        return jail(argv[2], argv[3])
    if len(argv) == 4 and argv[1] == "assassin":
        return assassin(argv[2], argv[3])
    if len(argv) == 5 and argv[1] == "garrison":
        return garrison(argv[2], argv[3], argv[4])
    if len(argv) == 4 and argv[1] == "undefended":
        return undefended(argv[2], argv[3])
    if len(argv) == 6 and argv[1] == "maintenance":
        return maintenance(argv[2], int(argv[3]), argv[4], argv[5])
    raise SystemExit(__doc__)


if __name__ == "__main__":
    main(sys.argv)
