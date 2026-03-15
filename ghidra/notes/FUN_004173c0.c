
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_004173c0(void)

{
  LPBYTE pBVar1;
  int iVar2;
  uint uVar3;
  undefined4 local_18 [3];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062ae48;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f3c20(&DAT_006b1498);
  pBVar1 = FUN_00401660();
  if (pBVar1 != (LPBYTE)0x0) {
    FUN_005f35b0(local_18,(char *)pBVar1);
    local_4 = 0;
    FUN_005f30d0(&DAT_006b1498,(int)local_18);
    local_4 = 0xffffffff;
    FUN_005f2ff0(local_18);
    FUN_005f35b0(local_18,&DAT_006a7c4c);
    local_4 = 1;
    FUN_005f30d0(&DAT_006b1498,(int)local_18);
    local_4 = 0xffffffff;
    FUN_005f2ff0(local_18);
    FUN_00618b60(pBVar1);
  }
  FUN_005f35b0(local_18,s_MDATA__006a8590);
  local_4 = 2;
  FUN_005f30d0(&DAT_006b1498,(int)local_18);
  local_4 = 0xffffffff;
  FUN_005f2ff0(local_18);
  FUN_00610c20(&LAB_00417780);
  iVar2 = FUN_005f41b0(s_SOFTWARE_LucasArts_Entertainment_006a81fc,s_MusicVolume_006a8584);
  DAT_006b14a8 = -iVar2;
  iVar2 = FUN_005f41b0(s_SOFTWARE_LucasArts_Entertainment_006a81fc,s_MusicSwitch_006a8578);
  DAT_006a8470 = (uint)(iVar2 == 0);
  if (iVar2 != 0) {
    _DAT_006b1490 = _DAT_006b1490 | 1;
  }
  uVar3 = FUN_004019e0();
  if (DAT_006a8470 == 0) {
    uVar3 = uVar3 & 0xfffffeff;
  }
  else {
    uVar3 = uVar3 | 0x100;
  }
  FUN_00401a20(uVar3);
  ExceptionList = local_c;
  return;
}

