package main

type dllTarget struct {
	Filename        string
	Directory       string
	Expected        int
	ExpectedType302 int
	RequiredBMPs    []uint32
}

// requiredEncyclopediaChromeResourceIDs is the complete source-proven bitmap
// set for the original encyclopedia shell and controls. The inner overlays are
// mode-specific and shared by both factions; shells and rails are faction-
// specific. Selected/disabled mode tabs reuse pressed resources, while the
// navigation controls have distinct disabled resources. STRATEGY text 0x1842,
// baked index text 0x1843, and font 0x299d are deliberately not BMP IDs.
var requiredEncyclopediaChromeResourceIDs = []uint32{
	0x285f, 0x2860, 0x2861, 0x2862, 0x2959, 0x295d,
	0x2882, 0x2883, 0x2888, 0x2889,
	0x288e, 0x288f, 0x2890, 0x2891, 0x2892, 0x2893,
	0x2886, 0x2887, 0x288c, 0x288d,
	0x2884, 0x2885, 0x288a, 0x288b,
	0x2864, 0x2863, 0x286e, 0x286d,
	0x286c, 0x286b, 0x2878, 0x2877,
	0x2868, 0x2867, 0x2874, 0x2873,
	0x2d60, 0x2d5f, 0x2d62, 0x2d61,
	0x2870, 0x286f, 0x287a, 0x2879,
	0x286a, 0x2869, 0x2876, 0x2875,
}

var uiDLLTargets = []dllTarget{
	{Filename: "COMMON.DLL", Directory: "common-dll", Expected: 321},
	{Filename: "GOKRES.DLL", Directory: "gokres-dll", Expected: 580},
	{Filename: "STRATEGY.DLL", Directory: "strategy-dll", Expected: 1042, RequiredBMPs: requiredEncyclopediaChromeResourceIDs},
	{Filename: "TACTICAL.DLL", Directory: "tactical-dll", Expected: 288},
	{Filename: "ALSPRITE.DLL", Directory: "alsprite-dll", Expected: 38, ExpectedType302: 1640},
	{Filename: "EMSPRITE.DLL", Directory: "emsprite-dll", Expected: 34, ExpectedType302: 2348},
	{Filename: "REBDLOG.DLL", Directory: "rebdlog-dll", Expected: 23},
}

// unloadedNamedBitmaps lists named bitmaps that REBEXE.EXE never loads by
// name. They are skipped instead of given an invented numeric ID.
var unloadedNamedBitmaps = map[string]bool{
	"DLG_CORNER_GRAB_FRAME": true,
}

var namedBitmapIDs = map[string]uint32{
	"COCKPIT_BUTTON_GAMESCALE_HUGE_UP":    15856,
	"COCKPIT_BUTTON_GAMESCALE_LARGE_UP":   15922,
	"COCKPIT_BUTTON_GAMESCALE_STD_UP":     15990,
	"DATA_BUTTON_UP_FIGHTERGROUP_RECOVER": 40720,
	"DATA_BUTTON_DN_FIGHTERGROUP_RECOVER": 40792,
	"DATA_BUTTON_UP_FIGHTERGROUP_TACTICS": 40864,
	"DATA_BUTTON_DN_FIGHTERGROUP_TACTICS": 40936,
}
