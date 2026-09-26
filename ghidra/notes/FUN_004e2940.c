
int * __fastcall FUN_004e2940(int param_1)

{
  bool bVar1;
  uint uVar2;
  void *pvVar3;
  int *piVar4;
  void *pvVar5;
  uint *puVar6;
  int iVar7;
  undefined1 **ppuVar8;
  int *local_68;
  undefined1 *local_60;
  void *local_5c;
  undefined4 local_58;
  undefined4 local_54 [2];
  undefined4 local_4c [7];
  int local_30;
  undefined4 local_2c [7];
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063de66;
  pvStack_c = ExceptionList;
  bVar1 = false;
  local_68 = (int *)0x0;
  ExceptionList = &pvStack_c;
  FUN_004f4340(local_54);
  uVar2 = *(uint *)(param_1 + 0x48) >> 0x18;
  local_4 = 0;
  local_5c = (void *)0x90;
  local_58 = 0x98;
  if ((0x8f < uVar2) && (uVar2 < 0x98)) {
    bVar1 = true;
  }
  FUN_00619730();
  if (!bVar1) goto LAB_004e2c05;
  iVar7 = *(int *)(param_1 + 0x2c);
  local_60 = &stack0xffffff80;
  FUN_004f26d0(&stack0xffffff80,(uint *)(param_1 + 0x48));
  pvVar3 = FUN_00403d30((void *)(iVar7 + 0x2c));
  piVar4 = FUN_004f3220(*(int *)(param_1 + 0x30),(uint *)(param_1 + 0x48));
  if ((piVar4 == (int *)0x0) || (pvVar3 == (void *)0x0)) goto LAB_004e2c05;
  FUN_0052cab0(local_2c,piVar4,1);
  local_4._0_1_ = 1;
  FUN_00513120((int)local_2c);
  pvVar3 = local_5c;
  if (local_10 == 0) {
LAB_004e2a94:
    if (pvVar3 == (void *)0x0) goto LAB_004e2a9c;
  }
  else {
    pvVar5 = (void *)FUN_00618b70(0x20);
    local_4._0_1_ = 2;
    local_5c = pvVar5;
    if (pvVar5 == (void *)0x0) {
      pvVar3 = (void *)0x0;
    }
    else {
      ppuVar8 = &local_60;
      pvVar3 = (void *)FUN_0052bed0((int)local_2c);
      puVar6 = FUN_004025b0(pvVar3,(uint *)ppuVar8);
      local_4 = CONCAT31(local_4._1_3_,3);
      pvVar3 = FUN_004f5b10(pvVar5,puVar6,0);
    }
    local_4 = 2;
    if (pvVar5 != (void *)0x0) {
      FUN_00619730();
    }
    local_4._0_1_ = 1;
    if (pvVar3 != (void *)0x0) {
      FUN_004f57b0(local_54,pvVar3);
      goto LAB_004e2a94;
    }
LAB_004e2a9c:
    pvVar3 = (void *)0x0;
    FUN_0052c740(local_4c,piVar4,1);
    local_4._0_1_ = 4;
    FUN_00513120((int)local_4c);
    if (local_30 != 0) {
      pvVar5 = (void *)FUN_00618b70(0x20);
      local_4._0_1_ = 5;
      local_5c = pvVar5;
      if (pvVar5 == (void *)0x0) {
        pvVar3 = (void *)0x0;
      }
      else {
        ppuVar8 = &local_60;
        pvVar3 = (void *)FUN_0052bed0((int)local_4c);
        puVar6 = FUN_004025b0(pvVar3,(uint *)ppuVar8);
        local_4 = CONCAT31(local_4._1_3_,6);
        pvVar3 = FUN_004f5b10(pvVar5,puVar6,0);
      }
      local_4 = 5;
      if (pvVar5 != (void *)0x0) {
        FUN_00619730();
      }
      local_4._0_1_ = 4;
      if (pvVar3 != (void *)0x0) {
        FUN_004f57b0(local_54,pvVar3);
      }
    }
    local_4._0_1_ = 1;
    FUN_0052c840(local_4c);
    if (pvVar3 == (void *)0x0) {
      FUN_0052c420(local_4c,piVar4,1);
      local_4._0_1_ = 7;
      FUN_00513120((int)local_4c);
      if (local_30 != 0) {
        pvVar3 = (void *)FUN_00618b70(0x20);
        local_4._0_1_ = 8;
        local_5c = pvVar3;
        if (pvVar3 == (void *)0x0) {
          pvVar5 = (void *)0x0;
        }
        else {
          ppuVar8 = &local_60;
          pvVar5 = (void *)FUN_0052bed0((int)local_4c);
          puVar6 = FUN_004025b0(pvVar5,(uint *)ppuVar8);
          local_4 = CONCAT31(local_4._1_3_,9);
          pvVar5 = FUN_004f5b10(pvVar3,puVar6,0);
        }
        local_4 = 8;
        if (pvVar3 != (void *)0x0) {
          FUN_00619730();
        }
        local_4._0_1_ = 7;
        if (pvVar5 != (void *)0x0) {
          FUN_004f57b0(local_54,pvVar5);
        }
      }
      local_4._0_1_ = 1;
      FUN_0052c520(local_4c);
    }
  }
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_0052cbb0(local_2c);
LAB_004e2c05:
  iVar7 = thunk_FUN_005f5060((int)local_54);
  if ((iVar7 != 0) && (local_68 = (int *)FUN_004f5cd0(0x200), local_68 != (int *)0x0)) {
    local_68[8] = *(int *)(param_1 + 0x30);
    (**(code **)(*local_68 + 0x24))();
  }
  local_4 = 0xffffffff;
  FUN_004f4380(local_54);
  ExceptionList = pvStack_c;
  return local_68;
}

