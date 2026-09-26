
undefined4 FUN_00409700(void)

{
  char *pcVar1;
  int iVar2;
  HCURSOR pHVar3;
  undefined4 uVar4;
  undefined4 local_80 [3];
  undefined4 local_74 [3];
  int local_68 [3];
  undefined4 auStack_5c [20];
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00629bf0;
  pvStack_c = ExceptionList;
  uVar4 = 0;
  ExceptionList = &pvStack_c;
  FUN_005f2f50(local_80);
  local_4 = 0;
  FUN_00409d20(local_80);
  pcVar1 = FUN_005f49e0(0x610);
  FUN_005f35b0(local_74,pcVar1);
  local_4._0_1_ = 1;
  FUN_005f30d0(local_80,(int)local_74);
  local_4._0_1_ = 0;
  FUN_005f2ff0(local_74);
  pcVar1 = FUN_005f49e0(0x613);
  FUN_005f35b0(local_74,pcVar1);
  local_4._0_1_ = 2;
  FUN_005f30d0(local_80,(int)local_74);
  local_4._0_1_ = 0;
  FUN_005f2ff0(local_74);
  FUN_00616620(local_68,1);
  local_4 = CONCAT31(local_4._1_3_,3);
  pcVar1 = (char *)FUN_00583c40((int)local_80);
  FUN_005f4a60(local_68,pcVar1);
  iVar2 = FUN_005f49b0();
  if (iVar2 == 0) {
    pHVar3 = LoadCursorA((HINSTANCE)0x0,(LPCSTR)0x7f02);
    pHVar3 = SetCursor(pHVar3);
    FUN_005f4d40(local_68);
    if (DAT_006b1218 != (void *)0x0) {
      FUN_0041dd60(DAT_006b1218,local_68);
    }
    FUN_005f4d40(local_68);
    FUN_00401270(local_68);
    FUN_005f4d40(local_68);
    uVar4 = 1;
    SetCursor(pHVar3);
  }
  FUN_005f4bc0(local_68);
  local_4 = local_4 & 0xffffff00;
  FUN_00616810((int)auStack_5c);
  FUN_00615440(auStack_5c);
  local_4 = 0xffffffff;
  FUN_005f2ff0(local_80);
  ExceptionList = pvStack_c;
  return uVar4;
}

