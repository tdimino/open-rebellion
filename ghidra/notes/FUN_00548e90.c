
bool FUN_00548e90(uint *param_1,void *param_2)

{
  bool bVar1;
  bool bVar2;
  int *piVar3;
  int iVar4;
  void *pvVar5;
  uint *puVar6;
  undefined4 uVar7;
  uint uVar8;
  undefined3 extraout_var;
  undefined4 uVar9;
  uint local_88;
  int *local_84;
  void *local_80;
  uint local_7c;
  uint uStack_78;
  undefined4 local_74 [2];
  undefined4 local_6c [7];
  int local_50;
  undefined4 auStack_4c [7];
  int iStack_30;
  undefined4 auStack_2c [7];
  int iStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00647bd1;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  piVar3 = (int *)FUN_00504dc0(param_1);
  local_84 = piVar3;
  FUN_004f4340(local_74);
  local_4 = 0;
  if (piVar3 != (int *)0x0) {
    FUN_00536da0(local_6c,piVar3,3);
    local_4 = CONCAT31(local_4._1_3_,1);
    FUN_00513120((int)local_6c);
    while (local_50 != 0) {
      iVar4 = FUN_0052bed0((int)local_6c);
      local_7c = *(uint *)(iVar4 + 0x78) >> 6 & 1;
      if (local_7c != 0) {
        puVar6 = &local_88;
        pvVar5 = (void *)FUN_0052bed0((int)local_6c);
        puVar6 = FUN_004025b0(pvVar5,puVar6);
        local_4._0_1_ = 2;
        FUN_004f44b0(local_74,puVar6,0);
        local_4 = CONCAT31(local_4._1_3_,1);
        FUN_00619730();
      }
      FUN_005130d0((int)local_6c);
    }
    local_4 = local_4 & 0xffffff00;
    FUN_00536ea0(local_6c);
    if (piVar3 != (int *)0x0) {
      FUN_004ffe70(auStack_2c,piVar3,4);
      local_4 = CONCAT31(local_4._1_3_,3);
      FUN_00513120((int)auStack_2c);
      while (iStack_10 != 0) {
        uVar9 = 4;
        uVar7 = FUN_0052bed0((int)auStack_2c);
        FUN_00502db0(auStack_4c,uVar7,uVar9);
        local_4 = CONCAT31(local_4._1_3_,4);
        FUN_00513120((int)auStack_4c);
        while (iStack_30 != 0) {
          uVar9 = 3;
          uVar7 = FUN_0052bed0((int)auStack_4c);
          FUN_00536da0(local_6c,uVar7,uVar9);
          local_4 = CONCAT31(local_4._1_3_,5);
          FUN_00513120((int)local_6c);
          while (local_50 != 0) {
            iVar4 = FUN_0052bed0((int)local_6c);
            local_7c = *(uint *)(iVar4 + 0x78) >> 6 & 1;
            if (local_7c != 0) {
              puVar6 = &local_88;
              pvVar5 = (void *)FUN_0052bed0((int)local_6c);
              puVar6 = FUN_004025b0(pvVar5,puVar6);
              local_4._0_1_ = 6;
              FUN_004f44b0(local_74,puVar6,0);
              local_4 = CONCAT31(local_4._1_3_,5);
              FUN_00619730();
            }
            FUN_005130d0((int)local_6c);
          }
          local_4._1_3_ = (uint3)(local_4 >> 8);
          local_4 = CONCAT31(local_4._1_3_,4);
          FUN_00536ea0(local_6c);
          FUN_005130d0((int)auStack_4c);
        }
        local_4._1_3_ = (uint3)(local_4 >> 8);
        local_4 = CONCAT31(local_4._1_3_,3);
        FUN_00502eb0(auStack_4c);
        FUN_005130d0((int)auStack_2c);
      }
      local_4 = local_4 & 0xffffff00;
      FUN_004fff70(auStack_2c);
    }
  }
  pvVar5 = (void *)thunk_FUN_005f5060((int)local_74);
  bVar1 = piVar3 != (int *)0x0;
  piVar3 = local_84;
  for (; local_84 = piVar3, pvVar5 != (void *)0x0; pvVar5 = *(void **)((int)pvVar5 + 0x10)) {
    puVar6 = (uint *)FUN_00403040(pvVar5,&local_88);
    local_4._0_1_ = 7;
    piVar3 = (int *)FUN_00505d40(puVar6);
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_00619730();
    bVar2 = bVar1;
    if ((piVar3 != (int *)0x0) &&
       ((iVar4 = (**(code **)(*piVar3 + 0xa8))(param_1,param_2), iVar4 == 0 ||
        (bVar2 = true, !bVar1)))) {
      bVar2 = false;
    }
    bVar1 = bVar2;
    piVar3 = local_84;
  }
  if (piVar3 != (int *)0x0) {
    local_7c = 0x90;
    uStack_78 = 0x98;
    local_4._1_3_ = (uint3)(local_4 >> 8);
    local_4._0_1_ = 8;
    uVar8 = (**(code **)(*piVar3 + 4))();
    if (((uVar8 < local_7c) || (uStack_78 <= uVar8)) || ((piVar3[0x22] & 0x1000U) == 0)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_00619730();
    if ((piVar3 != (int *)0x0) && (!bVar2)) {
      local_88 = 0;
      do {
        bVar2 = FUN_00549230(local_80,(int)piVar3,&local_88,param_2);
        if ((CONCAT31(extraout_var,bVar2) == 0) || (bVar2 = !bVar1, bVar1 = true, bVar2)) {
          bVar1 = false;
        }
      } while (local_88 != 0);
    }
  }
  local_4 = 0xffffffff;
  FUN_004f4380(local_74);
  ExceptionList = pvStack_c;
  return bVar1;
}

