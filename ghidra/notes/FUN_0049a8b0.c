
void __fastcall FUN_0049a8b0(int param_1)

{
  int iVar1;
  void *pvVar2;
  uint *puVar3;
  int *this;
  void *pvVar4;
  void *this_00;
  char *pcVar5;
  char *pcVar6;
  int iVar7;
  int local_94 [2];
  undefined1 local_8c [4];
  int local_88;
  undefined4 local_84 [3];
  undefined4 local_78 [2];
  int local_70;
  int aiStack_6c [2];
  undefined1 local_64 [8];
  undefined4 auStack_5c [8];
  int iStack_3c;
  _RTL_CRITICAL_SECTION _Stack_38;
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00636b3c;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  local_88 = param_1;
  FUN_00615db0(&local_70,1);
  pvVar2 = *(void **)(param_1 + 0x5c);
  local_4 = 0;
  FUN_004f4340(local_94);
  local_4._0_1_ = 1;
  FUN_00520570(local_78);
  local_4._0_1_ = 2;
  FUN_0053c2e0(pvVar2,local_78,local_94);
  for (pvVar2 = (void *)thunk_FUN_005f5060((int)local_94); pvVar2 != (void *)0x0;
      pvVar2 = *(void **)((int)pvVar2 + 0x10)) {
    puVar3 = FUN_00403040(pvVar2,local_8c);
    local_4._0_1_ = 3;
    this = FUN_004f2d10(*(int *)(param_1 + 0x20),puVar3);
    local_4._0_1_ = 2;
    FUN_00619730();
    if (this != (int *)0x0) {
      iVar7 = *(int *)((int)pvVar2 + 0x1c);
      pvVar4 = FUN_004f6270(this,local_84);
      local_4._0_1_ = 4;
      this_00 = FUN_00615f00(local_64,10);
      pcVar6 = &DAT_006a8798;
      pcVar5 = (char *)FUN_00583c40((int)pvVar4);
      pvVar4 = FUN_00616110(this_00,pcVar5);
      pvVar4 = FUN_00616110(pvVar4,pcVar6);
      FUN_00616e10(pvVar4,iVar7);
      local_4._0_1_ = 2;
      FUN_005f2ff0(local_84);
      param_1 = local_88;
    }
  }
  FUN_00615f00(local_64,0);
  pcVar5 = (char *)FUN_00615680(*(int *)((int)aiStack_6c + *(int *)(local_70 + 4)));
  FUN_005f35b0(local_84,pcVar5);
  local_4._0_1_ = 5;
  FUN_005f30d0((void *)(param_1 + 0x50),(int)local_84);
  local_4 = CONCAT31(local_4._1_3_,2);
  FUN_005f2ff0(local_84);
  FUN_00615660(*(void **)((int)aiStack_6c + *(int *)(local_70 + 4)),0);
  iVar7 = *(int *)(local_70 + 4);
  if (*(int *)((int)&iStack_3c + iVar7) < 0) {
    FUN_00615ee0((LPCRITICAL_SECTION)((int)&_Stack_38.DebugInfo + iVar7));
  }
  iVar1 = *(int *)((int)&iStack_3c + iVar7);
  *(undefined4 *)((int)aiStack_6c + iVar7 + 4) = 0;
  if (iVar1 < 0) {
    FUN_00615ef0((LPCRITICAL_SECTION)((int)&_Stack_38.DebugInfo + iVar7));
  }
  local_4._0_1_ = 1;
  FUN_00619730();
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_004f4380(local_94);
  local_4 = 0xffffffff;
  FUN_00615ea0((int)auStack_5c);
  FUN_00615440(auStack_5c);
  ExceptionList = pvStack_c;
  return;
}

