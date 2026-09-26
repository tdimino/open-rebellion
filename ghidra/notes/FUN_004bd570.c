
int * __fastcall FUN_004bd570(int param_1)

{
  bool bVar1;
  void *pvVar2;
  uint *puVar3;
  void *pvVar4;
  undefined4 *puVar5;
  int iVar6;
  int *local_28;
  undefined1 local_20 [4];
  undefined1 local_1c [4];
  void *local_18;
  undefined4 local_14 [2];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00639d8c;
  pvStack_c = ExceptionList;
  bVar1 = false;
  local_28 = (int *)0x0;
  ExceptionList = &pvStack_c;
  FUN_004f4340(local_14);
  local_4._0_1_ = 0;
  local_4._1_3_ = 0;
  *(uint *)(param_1 + 0x20) = *(uint *)(param_1 + 0x20) & 0xfffffff7;
  for (pvVar2 = (void *)thunk_FUN_005f5060(param_1 + 0x3c); pvVar2 != (void *)0x0;
      pvVar2 = *(void **)((int)pvVar2 + 0x10)) {
    puVar3 = FUN_00403040(pvVar2,local_20);
    local_4._0_1_ = 1;
    pvVar4 = FUN_004f5940(*(void **)(param_1 + 100),puVar3);
    local_4._0_1_ = 0;
    FUN_00619730();
    if ((pvVar4 != (void *)0x0) && ((*(uint *)((int)pvVar4 + 0x30) & 0x74000f1) == 0)) {
      if (*(int *)((int)pvVar4 + 0x38) == *(int *)(param_1 + 0x38)) {
        *(uint *)(param_1 + 0x20) = *(uint *)(param_1 + 0x20) | 8;
      }
      else {
        pvVar4 = (void *)FUN_00618b70(0x20);
        local_4._0_1_ = 2;
        local_18 = pvVar4;
        if (pvVar4 == (void *)0x0) {
          pvVar4 = (void *)0x0;
        }
        else {
          puVar5 = FUN_00403040(pvVar2,local_1c);
          bVar1 = true;
          local_4 = CONCAT31(local_4._1_3_,3);
          pvVar4 = FUN_004f5b10(pvVar4,puVar5,0);
        }
        local_4 = 2;
        if (bVar1) {
          bVar1 = false;
          FUN_00619730();
        }
        local_4._0_1_ = 0;
        if (pvVar4 != (void *)0x0) {
          FUN_004f57b0(local_14,pvVar4);
        }
      }
    }
  }
  bVar1 = false;
  for (pvVar2 = (void *)thunk_FUN_005f5060(param_1 + 0x44); pvVar2 != (void *)0x0;
      pvVar2 = *(void **)((int)pvVar2 + 0x10)) {
    puVar3 = FUN_00403040(pvVar2,local_1c);
    local_4._0_1_ = 4;
    pvVar4 = FUN_004f5940(*(void **)(param_1 + 100),puVar3);
    local_4._0_1_ = 0;
    FUN_00619730();
    if (((pvVar4 != (void *)0x0) && ((*(uint *)((int)pvVar4 + 0x30) & 0x74000f1) == 0)) &&
       (*(int *)((int)pvVar4 + 0x38) != *(int *)(param_1 + 0x38))) {
      pvVar4 = (void *)FUN_00618b70(0x20);
      local_4._0_1_ = 5;
      local_18 = pvVar4;
      if (pvVar4 == (void *)0x0) {
        pvVar4 = (void *)0x0;
      }
      else {
        puVar5 = FUN_00403040(pvVar2,local_20);
        bVar1 = true;
        local_4 = CONCAT31(local_4._1_3_,6);
        pvVar4 = FUN_004f5b10(pvVar4,puVar5,0);
      }
      local_4 = 5;
      if (bVar1) {
        bVar1 = false;
        FUN_00619730();
      }
      local_4._0_1_ = 0;
      if (pvVar4 != (void *)0x0) {
        FUN_004f57b0(local_14,pvVar4);
      }
    }
  }
  iVar6 = thunk_FUN_005f5060((int)local_14);
  if ((iVar6 != 0) && (local_28 = (int *)FUN_004f5cd0(0x201), local_28 != (int *)0x0)) {
    local_28[8] = *(int *)(param_1 + 0x24);
    (**(code **)(*local_28 + 0x24))(local_14);
    (**(code **)(*local_28 + 0x2c))(param_1 + 0x38);
  }
  local_4 = 0xffffffff;
  FUN_004f4380(local_14);
  ExceptionList = pvStack_c;
  return local_28;
}

