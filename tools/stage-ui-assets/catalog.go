package main

type dllTarget struct {
	Filename        string
	Directory       string
	Expected        int
	ExpectedType302 int
	// ExpectedRCData counts the droid action scripts (RT_RCDATA) that
	// FUN_0042b1d0 and the command classes read from a sprite DLL.
	ExpectedRCData int
	// ActionTable is the side's GData advice table (FUN_004c2c70,
	// FUN_004c0ba0), staged beside its droid's resources.
	ActionTable string
	// ExpectedWaves counts the droid voices and sounds (WAVE) that command
	// 4 plays through FUN_00403f70.
	ExpectedWaves int
}

var uiDLLTargets = []dllTarget{
	{Filename: "COMMON.DLL", Directory: "common-dll", Expected: 321},
	{Filename: "GOKRES.DLL", Directory: "gokres-dll", Expected: 580},
	{Filename: "STRATEGY.DLL", Directory: "strategy-dll", Expected: 1042},
	{Filename: "TACTICAL.DLL", Directory: "tactical-dll", Expected: 288},
	{Filename: "ALSPRITE.DLL", Directory: "alsprite-dll", Expected: 38, ExpectedType302: 1640, ExpectedRCData: 752, ActionTable: "C3POACT.SPT", ExpectedWaves: 213},
	{Filename: "EMSPRITE.DLL", Directory: "emsprite-dll", Expected: 34, ExpectedType302: 2348, ExpectedRCData: 753, ActionTable: "IMP22ACT.SPT", ExpectedWaves: 216},
	// The opening briefing tour's clips (FUN_005fefd0 module 13).
	{Filename: "ALBRIEF.DLL", Directory: "albrief-dll", Expected: 20, ExpectedType302: 2684, ExpectedRCData: 366, ExpectedWaves: 17},
	{Filename: "EMBRIEF.DLL", Directory: "embrief-dll", Expected: 18, ExpectedType302: 2738, ExpectedRCData: 471, ExpectedWaves: 22},
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
