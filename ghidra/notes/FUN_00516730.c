
uint __thiscall
FUN_00516730(void *this,int param_1,uint *param_2,undefined4 *param_3,int *param_4,
            undefined4 *param_5)

{
  bool bVar1;
  int iVar2;
  undefined4 uVar3;
  int *piVar4;
  int *piVar5;
  uint *puVar6;
  void *pvVar7;
  uint *puVar8;
  void *this_00;
  short *psVar9;
  short *psVar10;
  undefined4 uVar11;
  uint local_90;
  uint local_8c;
  uint local_88;
  uint local_84;
  int local_80;
  undefined4 uStack_7c;
  int local_78;
  int local_74;
  uint *local_70;
  undefined4 local_6c [7];
  int local_50;
  undefined4 local_4c [7];
  int local_30;
  undefined4 local_2c [7];
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064280f;
  pvStack_c = ExceptionList;
  local_70 = (uint *)((int)this + 0x8c);
  ExceptionList = &pvStack_c;
  *param_2 = 0;
  *param_3 = 0;
  *param_4 = 0;
  *param_5 = 0;
  local_88 = FUN_00505f60(local_70);
  local_90 = (uint)(local_88 != 0);
  if ((local_90 != 0) && ((param_1 == 2 || (local_90 = 0, param_1 == 1)))) {
    local_90 = 1;
  }
  if (local_90 != 0) {
    if (param_1 == 3) {
      iVar2 = 3;
    }
    else {
      iVar2 = 2 - (uint)(param_1 != 1);
    }
    FUN_004ffef0(local_6c,local_88,1,iVar2);
    local_4 = 0;
    FUN_00513120((int)local_6c);
    while ((local_50 != 0 && (*param_2 == 0))) {
      if (param_1 == 3) {
        iVar2 = 3;
      }
      else {
        iVar2 = 2 - (uint)(param_1 != 1);
      }
      uVar11 = 1;
      uVar3 = FUN_0052bed0((int)local_6c);
      FUN_00502e30(local_4c,uVar3,uVar11,iVar2);
      local_4 = CONCAT31(local_4._1_3_,1);
      FUN_00513120((int)local_4c);
      while ((local_30 != 0 && (*param_2 == 0))) {
        piVar4 = (int *)FUN_0052bed0((int)local_4c);
        local_8c = (**(code **)(*piVar4 + 0x280))();
        local_80 = (**(code **)(*piVar4 + 0x254))();
        local_80 = local_80 - local_8c;
        *param_2 = (uint)(local_80 != 0);
        FUN_005130d0((int)local_4c);
      }
      local_4 = local_4 & 0xffffff00;
      FUN_00502eb0(local_4c);
      FUN_005130d0((int)local_6c);
    }
    local_4 = 0xffffffff;
    FUN_004fff70(local_6c);
  }
  piVar4 = (int *)0x0;
  FUN_004ece30(&local_84);
  local_4 = 2;
  if (local_90 != 0) {
    FUN_004ffef0(local_4c,local_88,1,param_1);
    local_4 = CONCAT31(local_4._1_3_,3);
    FUN_00513120((int)local_4c);
    while ((local_30 != 0 && (piVar4 == (int *)0x0))) {
      uVar11 = 1;
      iVar2 = param_1;
      uVar3 = FUN_0052bed0((int)local_4c);
      FUN_00502e30(local_2c,uVar3,uVar11,iVar2);
      local_4 = CONCAT31(local_4._1_3_,4);
      FUN_00513120((int)local_2c);
      while ((local_10 != 0 && (piVar4 == (int *)0x0))) {
        piVar5 = (int *)FUN_0052bed0((int)local_2c);
        iVar2 = (**(code **)(*piVar5 + 0x34))(0);
        if (iVar2 != 0) {
          piVar4 = (int *)FUN_0052bed0((int)local_4c);
        }
        FUN_005130d0((int)local_2c);
      }
      local_4._1_3_ = (undefined3)(local_4 >> 8);
      local_4 = CONCAT31(local_4._1_3_,3);
      FUN_00502eb0(local_2c);
      FUN_005130d0((int)local_4c);
    }
    FUN_00503a50(local_6c,local_88,1,param_1);
    local_4 = CONCAT31(local_4._1_3_,5);
    FUN_00513120((int)local_6c);
    while ((local_50 != 0 && (piVar4 == (int *)0x0))) {
      piVar5 = (int *)FUN_0052bed0((int)local_6c);
      iVar2 = (**(code **)(*piVar5 + 0x34))(0);
      if (iVar2 != 0) {
        piVar4 = (int *)FUN_0052bed0((int)local_6c);
      }
      FUN_005130d0((int)local_6c);
    }
    local_4._1_3_ = (undefined3)(local_4 >> 8);
    local_4._0_1_ = 3;
    FUN_00503ad0(local_6c);
    local_4 = CONCAT31(local_4._1_3_,2);
    FUN_004fff70(local_4c);
  }
  if (piVar4 != (int *)0x0) {
    *param_3 = 1;
    if (local_90 != 0) {
      puVar6 = (uint *)FUN_0050c640(piVar4);
      local_78 = 0x90;
      local_74 = 0x98;
      local_4._0_1_ = 6;
      if ((*puVar6 >> 0x18 < 0x90) || (local_90 = 1, 0x97 < *puVar6 >> 0x18)) {
        local_90 = 0;
      }
      FUN_00619730();
      local_4 = CONCAT31(local_4._1_3_,2);
      FUN_00619730();
    }
    FUN_004ece30(&local_88);
    local_4._0_1_ = 7;
    if ((local_90 != 0) && (pvVar7 = (void *)thunk_FUN_00506e60(), local_90 != 0)) {
      FUN_00520570(&local_78);
      local_4._0_1_ = 8;
      puVar6 = (uint *)FUN_004ece30(&local_80);
      local_4._0_1_ = 9;
      puVar8 = (uint *)FUN_0050c640(piVar4);
      local_4._0_1_ = 10;
      local_90 = FUN_005545e0(pvVar7,piVar4,puVar8,puVar6,(uint *)0x0,0,&local_84,&local_78);
      local_4._0_1_ = 9;
      FUN_00619730();
      local_4._0_1_ = 8;
      FUN_00619730();
      if ((local_90 != 0) &&
         ((local_74 == -1 && (piVar4 = (int *)FUN_00504dc0(&local_84), piVar4 != (int *)0x0)))) {
        FUN_004ece30(&local_8c);
        local_4 = CONCAT31(local_4._1_3_,0xb);
        (**(code **)(*piVar4 + 0xc))(&local_8c);
        local_80 = 0x90;
        uStack_7c = 0x98;
        if ((local_8c >> 0x18 < 0x90) || (0x97 < local_8c >> 0x18)) {
          bVar1 = false;
        }
        else {
          bVar1 = true;
        }
        FUN_00619730();
        if (bVar1) {
          *param_4 = 1;
          FUN_004f26d0(&local_88,&local_8c);
        }
        local_4._0_1_ = 8;
        FUN_00619730();
      }
      local_4._0_1_ = 7;
      FUN_00619730();
    }
    if ((local_90 != 0) && (*param_4 != 0)) {
      pvVar7 = (void *)FUN_00505f60(local_70);
      this_00 = (void *)FUN_00505f60(&local_88);
      if ((pvVar7 == (void *)0x0) || (local_90 = 1, this_00 == (void *)0x0)) {
        local_90 = 0;
      }
      if (local_90 != 0) {
        psVar9 = (short *)FUN_00509620(this_00,&local_70);
        psVar10 = (short *)FUN_00509620(pvVar7,&local_80);
        uVar3 = FUN_0055d8a0(psVar10,psVar9);
        *param_5 = uVar3;
      }
    }
    local_4 = CONCAT31(local_4._1_3_,2);
    FUN_00619730();
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return local_90;
}

