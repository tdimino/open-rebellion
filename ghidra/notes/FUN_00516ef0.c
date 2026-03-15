
int __fastcall FUN_00516ef0(int param_1)

{
  void *pvVar1;
  bool bVar2;
  bool bVar3;
  bool bVar4;
  bool bVar5;
  undefined3 extraout_var;
  int iVar6;
  void *pvVar7;
  int iVar8;
  void *pvVar9;
  undefined3 extraout_var_00;
  undefined3 extraout_var_01;
  undefined3 extraout_var_02;
  undefined3 extraout_var_03;
  uint uVar10;
  int iVar11;
  int *piVar12;
  void *this;
  int local_60;
  undefined4 auStack_4c [7];
  int iStack_30;
  undefined4 local_2c [7];
  int iStack_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00642898;
  local_c = ExceptionList;
  pvVar1 = (void *)(param_1 + 0x90);
  ExceptionList = &local_c;
  bVar2 = FUN_005197f0(param_1);
  if (CONCAT31(extraout_var,bVar2) == 0) {
    ExceptionList = local_c;
    return 0;
  }
  iVar6 = FUN_004ece60((uint *)(param_1 + 0x8c));
  pvVar7 = (void *)FUN_00505f60((uint *)(param_1 + 0x8c));
  if ((pvVar7 == (void *)0x0) || (iVar6 == 0)) {
    iVar6 = 0;
  }
  else {
    iVar6 = 1;
  }
  if (pvVar7 == (void *)0x0) {
    ExceptionList = local_c;
    return iVar6;
  }
  iVar8 = FUN_0050a580(pvVar7,0,pvVar1);
  if ((iVar8 == 0) || (iVar6 == 0)) {
    iVar6 = 0;
  }
  else {
    iVar6 = 1;
  }
  local_60 = 0;
  iVar8 = FUN_00509ad0((int)pvVar7);
  pvVar9 = FUN_004ffef0(local_2c,pvVar7,1,1);
  local_4 = 0;
  bVar2 = FUN_005131b0((int)pvVar9);
  local_4 = 0xffffffff;
  FUN_004fff70(local_2c);
  pvVar9 = FUN_00503a50(local_2c,pvVar7,1,1);
  local_4 = 1;
  bVar3 = FUN_005131b0((int)pvVar9);
  local_4 = 0xffffffff;
  FUN_00503ad0(local_2c);
  pvVar9 = FUN_004ffef0(local_2c,pvVar7,1,2);
  local_4 = 2;
  bVar4 = FUN_005131b0((int)pvVar9);
  local_4 = 0xffffffff;
  FUN_004fff70(local_2c);
  pvVar9 = FUN_00503a50(local_2c,pvVar7,1,2);
  local_4 = 3;
  bVar5 = FUN_005131b0((int)pvVar9);
  local_4 = 0xffffffff;
  FUN_00503ad0(local_2c);
  if ((CONCAT31(extraout_var_00,bVar2) == 0) && (CONCAT31(extraout_var_01,bVar3) == 0)) {
    bVar2 = false;
  }
  else {
    bVar2 = true;
  }
  if ((CONCAT31(extraout_var_02,bVar4) == 0) && (CONCAT31(extraout_var_03,bVar5) == 0)) {
    bVar3 = false;
  }
  else {
    bVar3 = true;
  }
  if (iVar8 == 0) {
    if ((bVar2) && (bVar3)) {
      uVar10 = *(uint *)((int)pvVar7 + 0x78) >> 6 & 3;
      if (uVar10 == 3) {
        iVar8 = 3;
      }
      else {
        iVar8 = 2 - (uint)(uVar10 != 1);
      }
    }
    if (iVar8 == 0) goto LAB_005170c9;
  }
  if (iVar8 == 3) {
    local_60 = 3;
  }
  else {
    local_60 = 2 - (uint)(iVar8 != 1);
  }
LAB_005170c9:
  if (iVar8 != 0) {
    iVar11 = thunk_FUN_00506e60();
    if ((iVar11 == 0) || (iVar6 == 0)) {
      iVar6 = 0;
    }
    else {
      iVar6 = 1;
    }
    if (iVar11 != 0) {
      FUN_00503a50(auStack_4c,pvVar7,1,iVar8);
      local_4 = 4;
      FUN_00513120((int)auStack_4c);
      while (iStack_30 != 0) {
        pvVar9 = pvVar1;
        piVar12 = (int *)FUN_0052bed0((int)auStack_4c);
        uVar10 = FUN_00556710(piVar12,pvVar9);
        if ((uVar10 == 0) || (iVar6 == 0)) {
          iVar6 = 0;
        }
        else {
          iVar6 = 1;
        }
        FUN_005130d0((int)auStack_4c);
      }
      local_4 = 0xffffffff;
      FUN_00503ad0(auStack_4c);
    }
  }
  if (local_60 != 0) {
    FUN_004ffef0(local_2c,pvVar7,1,local_60);
    local_4 = 5;
    FUN_00513120((int)local_2c);
    while (iStack_10 != 0) {
      iVar8 = 1;
      pvVar9 = pvVar1;
      this = (void *)FUN_0052bed0((int)local_2c);
      iVar8 = FUN_004f78e0(this,iVar8,pvVar9);
      if ((iVar8 == 0) || (iVar6 == 0)) {
        iVar6 = 0;
      }
      else {
        iVar6 = 1;
      }
      FUN_005130d0((int)local_2c);
    }
    FUN_00503a50(auStack_4c,pvVar7,1,local_60);
    local_4 = CONCAT31(local_4._1_3_,6);
    FUN_00513120((int)auStack_4c);
    while (iStack_30 != 0) {
      iVar8 = 1;
      pvVar7 = pvVar1;
      pvVar9 = (void *)FUN_0052bed0((int)auStack_4c);
      iVar8 = FUN_004f78e0(pvVar9,iVar8,pvVar7);
      if ((iVar8 == 0) || (iVar6 == 0)) {
        iVar6 = 0;
      }
      else {
        iVar6 = 1;
      }
      FUN_005130d0((int)auStack_4c);
    }
    local_4._1_3_ = (undefined3)((uint)local_4 >> 8);
    local_4 = CONCAT31(local_4._1_3_,5);
    FUN_00503ad0(auStack_4c);
    local_4 = 0xffffffff;
    FUN_004fff70(local_2c);
  }
  ExceptionList = local_c;
  return iVar6;
}

