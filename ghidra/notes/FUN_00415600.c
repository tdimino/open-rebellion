
uint __fastcall FUN_00415600(void *param_1)

{
  int iVar1;
  int iVar2;
  void *pvVar3;
  char *pcVar4;
  HCURSOR pHVar5;
  uint local_b4;
  undefined4 local_b0;
  int local_ac;
  undefined4 local_a4 [3];
  undefined4 local_98 [3];
  undefined4 auStack_8c [4];
  undefined4 uStack_7c;
  undefined4 uStack_78;
  undefined4 uStack_74;
  int iStack_70;
  undefined4 uStack_6c;
  undefined4 uStack_68;
  int local_64 [2];
  undefined4 auStack_5c [20];
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062ab5f;
  pvStack_c = ExceptionList;
  iVar1 = *(int *)((int)param_1 + 0xcc);
  if ((*(int *)((int)param_1 + 0x104) == 7) && (*(int *)((int)param_1 + 0xe4) == 2)) {
    iVar2 = 1;
  }
  else {
    iVar2 = 0;
  }
  ExceptionList = &pvStack_c;
  FUN_00415ad0(param_1,local_98,iVar1,iVar2);
  local_4 = 0;
  FUN_005f2f90(&local_b0,(int)param_1 + 0xd0);
  local_4._0_1_ = 1;
  iVar2 = FUN_005f2f50(local_a4);
  local_4._0_1_ = 2;
  FUN_005f3090((void *)((int)param_1 + 0xd0),iVar2);
  local_4._0_1_ = 1;
  FUN_005f2ff0(local_a4);
  if (local_ac == 0) {
    if ((*(int *)((int)param_1 + 0x104) == 7) && (*(int *)((int)param_1 + 0xe4) == 2)) {
      iVar2 = 1;
    }
    else {
      iVar2 = 0;
    }
    pvVar3 = FUN_00415a00(param_1,local_a4,iVar1,iVar2);
    local_4._0_1_ = 3;
    FUN_005f3090(&local_b0,(int)pvVar3);
    local_4._0_1_ = 1;
    FUN_005f2ff0(local_a4);
    if (local_ac == 0) {
      FUN_005f32b0(&local_b0,1);
    }
  }
  *(undefined4 *)((int)param_1 + 0xcc) = 0;
  FUN_006164b0(local_64,1);
  local_4._0_1_ = 4;
  pcVar4 = (char *)FUN_00583c40((int)local_98);
  FUN_005f4b10(local_64,pcVar4);
  iVar2 = FUN_005f49b0();
  if (iVar2 == 0) {
    pHVar5 = LoadCursorA((HINSTANCE)0x0,(LPCSTR)0x7f02);
    pHVar5 = SetCursor(pHVar5);
    FUN_00411730(auStack_8c);
    uStack_74 = *(undefined4 *)((int)param_1 + 0xe4);
    local_4._0_1_ = 5;
    uStack_7c = 1;
    uStack_78 = 0;
    iStack_70 = iVar1;
    FUN_005f2f90(&stack0xffffff30,(int)&local_b0);
    FUN_00415890((int)auStack_8c);
    uStack_6c = *(undefined4 *)((int)param_1 + 0x108);
    uStack_68 = *(undefined4 *)((int)param_1 + 0x13c);
    FUN_00411970(auStack_8c,local_64);
    FUN_00401bb0(local_64);
    SetCursor(pHVar5);
    local_4._0_1_ = 4;
    FUN_004118f0(auStack_8c);
  }
  local_b4 = (uint)(iVar2 == 0);
  FUN_005f4bc0(local_64);
  if (((*(int *)((int)param_1 + 0xdc) == 0xb) || (*(int *)((int)param_1 + 0xdc) == 0xc)) &&
     (*(void **)((int)param_1 + 0xc0) != (void *)0x0)) {
    FUN_00408240(*(void **)((int)param_1 + 0xc0),local_b4,iVar1);
  }
  local_4._0_1_ = 1;
  FUN_00616590((int)auStack_5c);
  FUN_00615440(auStack_5c);
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_005f2ff0(&local_b0);
  local_4 = 0xffffffff;
  FUN_005f2ff0(local_98);
  ExceptionList = pvStack_c;
  return local_b4;
}

