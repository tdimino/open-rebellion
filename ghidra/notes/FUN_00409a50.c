
/* WARNING: Type propagation algorithm not settling */

bool FUN_00409a50(void *param_1)

{
  char *pcVar1;
  int iVar2;
  HCURSOR pHVar3;
  undefined4 local_7c [3];
  undefined4 local_70 [3];
  int local_64;
  undefined4 uStack_60;
  undefined1 auStack_5c [80];
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00629c50;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_005f2f50(local_7c);
  local_4 = 0;
  FUN_00409d20(local_7c);
  pcVar1 = FUN_005f49e0(0x610);
  FUN_005f35b0(local_70,pcVar1);
  local_4._0_1_ = 1;
  FUN_005f30d0(local_7c,(int)local_70);
  local_4._0_1_ = 0;
  FUN_005f2ff0(local_70);
  pcVar1 = FUN_005f49e0(0x613);
  FUN_005f35b0(local_70,pcVar1);
  local_4._0_1_ = 2;
  FUN_005f30d0(local_7c,(int)local_70);
  local_4._0_1_ = 0;
  FUN_005f2ff0(local_70);
  FUN_006164b0(&local_64,1);
  local_4 = CONCAT31(local_4._1_3_,3);
  pcVar1 = (char *)FUN_00583c40((int)local_7c);
  FUN_005f4b10(&local_64,pcVar1);
  iVar2 = FUN_005f49b0();
  if (iVar2 == 0) {
    pHVar3 = LoadCursorA((HINSTANCE)0x0,(LPCSTR)0x7f02);
    pHVar3 = SetCursor(pHVar3);
    FUN_00616b70(param_1,*(int **)((int)&uStack_60 + *(int *)(local_64 + 4)));
    SetCursor(pHVar3);
  }
  FUN_005f4bc0(&local_64);
  local_4 = local_4 & 0xffffff00;
  FUN_00616590((int)auStack_5c);
  FUN_00615440((undefined4 *)auStack_5c);
  local_4 = 0xffffffff;
  FUN_005f2ff0(local_7c);
  ExceptionList = pvStack_c;
  return iVar2 == 0;
}

