package main

type dllTarget struct {
	Filename        string
	Directory       string
	Expected        int
	ExpectedType302 int
}

var uiDLLTargets = []dllTarget{
	{Filename: "COMMON.DLL", Directory: "common-dll", Expected: 321},
	{Filename: "GOKRES.DLL", Directory: "gokres-dll", Expected: 580},
	{Filename: "STRATEGY.DLL", Directory: "strategy-dll", Expected: 1042},
	{Filename: "TACTICAL.DLL", Directory: "tactical-dll", Expected: 288},
	{Filename: "ALSPRITE.DLL", Directory: "alsprite-dll", Expected: 38, ExpectedType302: 1640},
	{Filename: "EMSPRITE.DLL", Directory: "emsprite-dll", Expected: 34, ExpectedType302: 2348},
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
