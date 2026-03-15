
void __fastcall FUN_004e4a20(int param_1)

{
  int iVar1;
  bool bVar2;
  void *pvVar3;
  undefined4 *puVar4;
  void *pvVar5;
  uint *puVar6;
  void *pvVar7;
  uint local_3c;
  int local_38;
  int local_34;
  undefined1 local_30 [4];
  undefined1 local_2c [4];
  undefined1 local_28 [4];
  undefined1 local_24 [4];
  undefined1 *local_20;
  undefined4 local_1c [2];
  undefined4 local_14;
  undefined4 local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063e253;
  pvStack_c = ExceptionList;
  local_34 = 0;
  ExceptionList = &pvStack_c;
  FUN_004f5510(local_1c);
  local_4 = 0;
  FUN_004ece30(&local_3c);
  local_4._0_1_ = 1;
  local_38 = 1;
  do {
    FUN_004ece80(&local_3c);
    for (pvVar3 = (void *)thunk_FUN_005f5060(param_1 + 0x60); pvVar3 != (void *)0x0;
        pvVar3 = *(void **)((int)pvVar3 + 0x10)) {
      puVar4 = (undefined4 *)FUN_00403040(pvVar3,local_30);
      iVar1 = *(int *)(param_1 + 0x2c);
      local_20 = &stack0xffffffb0;
      local_4._0_1_ = 2;
      FUN_004f26d0(&stack0xffffffb0,puVar4);
      pvVar5 = FUN_00403d30((void *)(iVar1 + 0x2c));
      local_4._0_1_ = 1;
      FUN_00619730();
      if ((pvVar5 != (void *)0x0) && (local_34 < *(int *)((int)pvVar5 + 0x60))) {
        if ((*(uint *)((int)pvVar5 + 0x30) & 0x10000000) != 0) {
          puVar6 = (uint *)FUN_00403040(pvVar3,local_2c);
          local_4._0_1_ = 3;
          pvVar7 = FUN_004f5940((void *)(param_1 + 0x68),puVar6);
          local_4._0_1_ = 1;
          FUN_00619730();
          if (pvVar7 == (void *)0x0) goto LAB_004e4b7d;
        }
        puVar6 = (uint *)FUN_00403040(pvVar3,local_28);
        local_4._0_1_ = 4;
        pvVar7 = FUN_004f5940(local_1c,puVar6);
        local_4._0_1_ = 1;
        FUN_00619730();
        if ((pvVar7 == (void *)0x0) && (0 < *(int *)((int)pvVar5 + 0x90))) {
          local_34 = *(int *)((int)pvVar5 + 0x60);
          puVar4 = (undefined4 *)FUN_00403040(pvVar3,local_24);
          local_4._0_1_ = 5;
          FUN_004f26d0(&local_3c,puVar4);
          local_4._0_1_ = 1;
          FUN_00619730();
        }
      }
LAB_004e4b7d:
    }
    local_14 = 0x90;
    local_10 = 0x98;
    if ((local_3c >> 0x18 < 0x90) || (0x97 < local_3c >> 0x18)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    FUN_00619730();
    if (bVar2) {
      local_20 = (undefined1 *)FUN_00618b70(0x1c);
      local_4._0_1_ = 6;
      if (local_20 == (undefined1 *)0x0) {
        pvVar3 = (void *)0x0;
      }
      else {
        pvVar3 = FUN_004ecc70(local_20,&local_3c);
      }
      local_4._0_1_ = 1;
      if (pvVar3 != (void *)0x0) {
        FUN_004f57b0(local_1c,pvVar3);
        local_38 = local_38 + -1;
      }
    }
    else {
      local_38 = 0;
    }
    if (local_38 < 1) {
      for (pvVar3 = (void *)thunk_FUN_005f5060(param_1 + 0x68); pvVar3 != (void *)0x0;
          pvVar3 = *(void **)((int)pvVar3 + 0x10)) {
        puVar4 = (undefined4 *)FUN_00403040(pvVar3,local_24);
        iVar1 = *(int *)(param_1 + 0x2c);
        local_20 = &stack0xffffffb0;
        local_4._0_1_ = 7;
        FUN_004f26d0(&stack0xffffffb0,puVar4);
        pvVar5 = FUN_00403d30((void *)(iVar1 + 0x2c));
        local_4._0_1_ = 1;
        FUN_00619730();
        if (pvVar5 != (void *)0x0) {
          *(uint *)((int)pvVar5 + 0x30) = *(uint *)((int)pvVar5 + 0x30) & 0xefffffff;
        }
      }
      (**(code **)(*(int *)(param_1 + 0x68) + 4))();
      pvVar3 = (void *)thunk_FUN_005f5060((int)local_1c);
      while (pvVar3 != (void *)0x0) {
        puVar4 = (undefined4 *)FUN_00403040(pvVar3,local_24);
        iVar1 = *(int *)(param_1 + 0x2c);
        local_20 = &stack0xffffffb0;
        local_4._0_1_ = 8;
        FUN_004f26d0(&stack0xffffffb0,puVar4);
        pvVar5 = FUN_00403d30((void *)(iVar1 + 0x2c));
        local_4._0_1_ = 1;
        FUN_00619730();
        if (pvVar5 != (void *)0x0) {
          *(uint *)((int)pvVar5 + 0x30) = *(uint *)((int)pvVar5 + 0x30) | 0x10000000;
        }
        pvVar5 = *(void **)((int)pvVar3 + 0x10);
        puVar6 = (uint *)FUN_00403040(pvVar3,local_28);
        local_4._0_1_ = 9;
        FUN_004f5910(local_1c,puVar6);
        local_4._0_1_ = 1;
        FUN_00619730();
        FUN_004f57b0((void *)(param_1 + 0x68),pvVar3);
        pvVar3 = pvVar5;
      }
      local_4 = (uint)local_4._1_3_ << 8;
      FUN_00619730();
      local_4 = 0xffffffff;
      FUN_004f55c0(local_1c);
      ExceptionList = pvStack_c;
      return;
    }
  } while( true );
}

