
int * __fastcall FUN_004d9b00(int param_1)

{
  bool bVar1;
  bool bVar2;
  uint uVar3;
  int *piVar4;
  void *pvVar5;
  void *pvVar6;
  uint *puVar7;
  int iVar8;
  uint local_84;
  int *local_80;
  void *local_7c;
  undefined4 local_78;
  undefined4 local_74 [2];
  undefined4 local_6c [7];
  int local_50;
  undefined4 local_4c [7];
  int local_30;
  undefined4 local_2c [7];
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063ced8;
  pvStack_c = ExceptionList;
  bVar1 = false;
  bVar2 = false;
  local_80 = (int *)0x0;
  ExceptionList = &pvStack_c;
  FUN_004f4340(local_74);
  uVar3 = *(uint *)(param_1 + 0x54) >> 0x18;
  local_4 = 0;
  local_7c = (void *)0x90;
  local_78 = 0x98;
  if ((0x8f < uVar3) && (uVar3 < 0x98)) {
    bVar1 = true;
  }
  FUN_00619730();
  if ((bVar1) &&
     (piVar4 = FUN_004f3220(*(int *)(param_1 + 0x30),(uint *)(param_1 + 0x54)), piVar4 != (int *)0x0
     )) {
    FUN_0052cab0(local_2c,piVar4,1);
    local_4 = CONCAT31(local_4._1_3_,1);
    FUN_00513120((int)local_2c);
    while (local_10 != 0) {
      pvVar5 = (void *)FUN_00618b70(0x20);
      local_4._0_1_ = 2;
      local_7c = pvVar5;
      if (pvVar5 == (void *)0x0) {
        pvVar5 = (void *)0x0;
      }
      else {
        puVar7 = &local_84;
        pvVar6 = (void *)FUN_0052bed0((int)local_2c);
        puVar7 = FUN_004025b0(pvVar6,puVar7);
        bVar2 = true;
        local_4 = CONCAT31(local_4._1_3_,3);
        pvVar5 = FUN_004f5b10(pvVar5,puVar7,0);
      }
      local_4 = 2;
      if (bVar2) {
        bVar2 = false;
        FUN_00619730();
      }
      local_4 = CONCAT31(local_4._1_3_,1);
      if (pvVar5 != (void *)0x0) {
        FUN_004f57b0(local_74,pvVar5);
      }
      FUN_005130d0((int)local_2c);
    }
    bVar1 = false;
    FUN_0052c740(local_4c,piVar4,1);
    local_4 = CONCAT31(local_4._1_3_,4);
    FUN_00513120((int)local_4c);
    while (bVar2 = false, local_30 != 0) {
      pvVar5 = (void *)FUN_00618b70(0x20);
      local_4._0_1_ = 5;
      local_7c = pvVar5;
      if (pvVar5 == (void *)0x0) {
        pvVar5 = (void *)0x0;
      }
      else {
        puVar7 = &local_84;
        pvVar6 = (void *)FUN_0052bed0((int)local_4c);
        puVar7 = FUN_004025b0(pvVar6,puVar7);
        bVar1 = true;
        local_4 = CONCAT31(local_4._1_3_,6);
        pvVar5 = FUN_004f5b10(pvVar5,puVar7,0);
      }
      local_4 = 5;
      if (bVar1) {
        bVar1 = false;
        FUN_00619730();
      }
      local_4 = CONCAT31(local_4._1_3_,4);
      if (pvVar5 != (void *)0x0) {
        FUN_004f57b0(local_74,pvVar5);
      }
      FUN_005130d0((int)local_4c);
    }
    FUN_0052c420(local_6c,piVar4,1);
    local_4 = CONCAT31(local_4._1_3_,7);
    FUN_00513120((int)local_6c);
    while (local_50 != 0) {
      pvVar5 = (void *)FUN_00618b70(0x20);
      local_4._0_1_ = 8;
      local_7c = pvVar5;
      if (pvVar5 == (void *)0x0) {
        pvVar5 = (void *)0x0;
      }
      else {
        puVar7 = &local_84;
        pvVar6 = (void *)FUN_0052bed0((int)local_6c);
        puVar7 = FUN_004025b0(pvVar6,puVar7);
        bVar2 = true;
        local_4 = CONCAT31(local_4._1_3_,9);
        pvVar5 = FUN_004f5b10(pvVar5,puVar7,0);
      }
      local_4 = 8;
      if (bVar2) {
        bVar2 = false;
        FUN_00619730();
      }
      local_4 = CONCAT31(local_4._1_3_,7);
      if (pvVar5 != (void *)0x0) {
        FUN_004f57b0(local_74,pvVar5);
      }
      FUN_005130d0((int)local_6c);
    }
    local_4._1_3_ = (uint3)((uint)local_4 >> 8);
    local_4._0_1_ = 4;
    FUN_0052c520(local_6c);
    local_4._0_1_ = 1;
    FUN_0052c840(local_4c);
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_0052cbb0(local_2c);
  }
  iVar8 = thunk_FUN_005f5060((int)local_74);
  piVar4 = local_80;
  if ((iVar8 != 0) && (piVar4 = (int *)FUN_004f5cd0(0x200), piVar4 != (int *)0x0)) {
    piVar4[8] = *(int *)(param_1 + 0x30);
    (**(code **)(*piVar4 + 0x24))(local_74);
  }
  local_4 = 0xffffffff;
  FUN_004f4380(local_74);
  ExceptionList = pvStack_c;
  return piVar4;
}

