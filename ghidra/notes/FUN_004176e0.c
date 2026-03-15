
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl FUN_004176e0(int param_1)

{
  uint uVar1;
  
  DAT_006a8470 = param_1;
  if (param_1 == 0) {
    if ((_DAT_006b1490 & 1) == 0) {
      _DAT_006b1490 = _DAT_006b1490 | 1;
      FUN_00417640();
    }
  }
  else if ((_DAT_006b1490 & 1) != 0) {
    _DAT_006b1490 = _DAT_006b1490 & 0xfffffffe;
    uVar1 = FUN_0041d3b0();
    FUN_00417520(uVar1);
  }
  FUN_005f4250((HKEY)s_HKEY_LOCAL_MACHINE_006a844c,s_SOFTWARE_LucasArts_Entertainment_006a81fc,
               s_MusicSwitch_006a8578);
  uVar1 = FUN_004019e0();
  if (DAT_006a8470 != 0) {
    FUN_00401a20(uVar1 | 0x100);
    return;
  }
  FUN_00401a20(uVar1 & 0xfffffeff);
  return;
}

