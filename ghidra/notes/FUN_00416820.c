
void FUN_00416820(void)

{
  LPCSTR pCVar1;
  LPBYTE pBVar2;
  uint uVar3;
  char *pcVar4;
  undefined4 local_3c [3];
  undefined4 local_30 [3];
  undefined4 local_24 [3];
  undefined4 local_18 [3];
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062ad50;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f2f50(local_3c);
  local_4 = 0;
  FUN_005f35b0(local_18,s_Star_Wars_Rebellion_006a7c50);
  local_4._0_1_ = 1;
  FUN_005f35b0(local_24,&DAT_006a7c4c);
  local_4._0_1_ = 2;
  FUN_005f35b0(local_30,s_SOFTWARE_LucasArts_Entertainment_006a7c1c);
  local_4._0_1_ = 3;
  FUN_005f30d0(local_3c,(int)local_30);
  FUN_005f30d0(local_3c,(int)local_24);
  FUN_005f30d0(local_3c,(int)local_18);
  local_4._0_1_ = 2;
  FUN_005f2ff0(local_30);
  local_4._0_1_ = 1;
  FUN_005f2ff0(local_24);
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_005f2ff0(local_18);
  pcVar4 = s_InstalledLocation_006a7c84;
  pCVar1 = (LPCSTR)FUN_00583c40((int)local_3c);
  pBVar2 = FUN_005f3fc0(pCVar1,pcVar4);
  if (pBVar2 != (LPBYTE)0x0) {
    uVar3 = FUN_00619bf0((int)(char)*pBVar2);
    if (1 < uVar3 - 0x40) {
      FUN_00619b70(uVar3 - 0x40);
    }
    if (pBVar2 + 2 != (LPBYTE)0x0) {
      FUN_00619ad0((LPCSTR)(pBVar2 + 2));
    }
    FUN_00618b60(pBVar2);
  }
  local_4 = 0xffffffff;
  FUN_005f2ff0(local_3c);
  ExceptionList = local_c;
  return;
}

