
undefined4 __fastcall FUN_00415440(void *param_1)

{
  int iVar1;
  char *pcVar2;
  HCURSOR pHVar3;
  LPBYTE pBVar4;
  undefined4 uVar5;
  undefined4 local_9c [3];
  undefined4 auStack_90 [4];
  undefined4 uStack_80;
  undefined4 uStack_7c;
  undefined4 uStack_78;
  undefined4 uStack_70;
  int local_68 [3];
  undefined4 auStack_5c [20];
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062ab0e;
  pvStack_c = ExceptionList;
  uVar5 = 0;
  ExceptionList = &pvStack_c;
  FUN_0041dae0(0);
  if ((*(int *)((int)param_1 + 0x104) == 7) && (*(int *)((int)param_1 + 0xe4) == 2)) {
    iVar1 = 1;
  }
  else {
    iVar1 = 0;
  }
  FUN_00415ad0(param_1,local_9c,*(int *)((int)param_1 + 200),iVar1);
  local_4 = 0;
  *(undefined4 *)((int)param_1 + 200) = 0;
  FUN_00616620(local_68,1);
  local_4._0_1_ = 1;
  pcVar2 = (char *)FUN_00583c40((int)local_9c);
  FUN_005f4a60(local_68,pcVar2);
  iVar1 = FUN_005f49b0();
  if (iVar1 == 0) {
    pHVar3 = LoadCursorA((HINSTANCE)0x0,(LPCSTR)0x7f02);
    pHVar3 = SetCursor(pHVar3);
    FUN_00411730(auStack_90);
    local_4 = CONCAT31(local_4._1_3_,2);
    FUN_00411900(auStack_90,local_68);
    *(undefined4 *)((int)param_1 + 0x130) = uStack_80;
    *(undefined4 *)((int)param_1 + 0x108) = uStack_70;
    *(undefined4 *)((int)param_1 + 0x134) = uStack_7c;
    *(undefined4 *)((int)param_1 + 0xe4) = uStack_78;
    *(undefined4 *)((int)param_1 + 0x13c) = 0;
    iVar1 = FUN_005fba20(2);
    if (iVar1 == 0) {
      pBVar4 = FUN_00401440();
      FUN_005fba30((char *)pBVar4,s_textstra_dll_006a7ecc,2);
      if (pBVar4 != (LPBYTE)0x0) {
        FUN_00618b60(pBVar4);
      }
    }
    thunk_FUN_00409280();
    FUN_00401b70(local_68);
    uVar5 = 1;
    SetCursor(pHVar3);
    local_4._0_1_ = 1;
    FUN_004118f0(auStack_90);
  }
  FUN_005f4bc0(local_68);
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00616810((int)auStack_5c);
  FUN_00615440(auStack_5c);
  local_4 = 0xffffffff;
  FUN_005f2ff0(local_9c);
  ExceptionList = pvStack_c;
  return uVar5;
}

