
undefined4 __fastcall FUN_0049c0d0(void *param_1)

{
  int iVar1;
  int iVar2;
  int *piVar3;
  uint uVar4;
  int *piVar5;
  undefined4 uVar6;
  void *pvVar7;
  undefined4 *puVar8;
  int *piVar9;
  undefined4 uVar10;
  undefined4 local_ec [8];
  undefined4 local_cc [8];
  undefined4 local_ac [8];
  undefined4 local_8c [8];
  undefined4 local_6c [8];
  undefined4 local_4c [8];
  undefined4 local_2c [8];
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00636f84;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  iVar1 = FUN_004f3dd0(*(int *)((int)param_1 + 0x20),*(int *)((int)param_1 + 0x20));
  iVar2 = FUN_004f3dd0(*(int *)((int)param_1 + 0x20),2 - (*(int *)((int)param_1 + 0x20) != 1));
  piVar3 = FUN_004f3220(*(int *)((int)param_1 + 0x20),(uint *)((int)param_1 + 0x44));
  if (((iVar1 != 0) && (piVar3 != (int *)0x0)) && (iVar2 != 0)) {
    piVar9 = piVar3 + 0xc;
    FUN_0060b9d0((void *)((int)param_1 + 0x54),0x7020,piVar9,(undefined4 *)0x0,(undefined4 *)0x0,
                 (undefined4 *)0x0);
    uVar4 = (uint)piVar3[9] >> 6 & 3;
    *(uint *)((int)param_1 + 0x4c) = (uint)piVar3[0x1e] >> 6 & 3;
    piVar5 = piVar9;
    if (uVar4 == *(uint *)((int)param_1 + 0x48)) {
      if (((uint)piVar3[0x1e] >> 6 & 3) == uVar4) {
        puVar8 = (undefined4 *)(iVar1 + 0x30);
        uVar4 = 0x7022;
        piVar9 = (int *)(iVar2 + 0x30);
      }
      else {
        puVar8 = (undefined4 *)(iVar2 + 0x30);
        uVar4 = 0x7021;
        piVar9 = (int *)(iVar1 + 0x30);
      }
    }
    else if ((uVar4 == 0) || (uVar4 == 3)) {
      piVar5 = (int *)(iVar2 + 0x30);
      puVar8 = (undefined4 *)(iVar1 + 0x30);
      uVar4 = 0x7025;
    }
    else if (((uint)piVar3[0x1e] >> 6 & 3) == uVar4) {
      puVar8 = (undefined4 *)(iVar2 + 0x30);
      uVar4 = 0x7024;
      piVar5 = (int *)(iVar1 + 0x30);
    }
    else {
      puVar8 = (undefined4 *)(iVar1 + 0x30);
      uVar4 = 0x7023;
      piVar9 = (int *)(iVar2 + 0x30);
    }
    FUN_0060b9d0((void *)((int)param_1 + 0x60),uVar4,puVar8,piVar5,piVar9,(undefined4 *)0x0);
    FUN_004ffe70(local_cc,piVar3,1);
    local_4 = 0;
    FUN_005039d0(local_2c,piVar3,1);
    local_4._0_1_ = 1;
    FUN_004f25c0(local_6c,piVar3,1);
    local_4 = CONCAT31(local_4._1_3_,2);
    FUN_00513120((int)local_cc);
    iVar1 = FUN_0052bed0((int)local_cc);
    while (iVar1 != 0) {
      uVar10 = 1;
      uVar6 = FUN_0052bed0((int)local_cc);
      FUN_00502db0(local_ec,uVar6,uVar10);
      local_4 = CONCAT31(local_4._1_3_,3);
      pvVar7 = (void *)FUN_0052bed0((int)local_cc);
      FUN_0049c5d0(param_1,pvVar7);
      FUN_00513120((int)local_ec);
      iVar1 = FUN_0052bed0((int)local_ec);
      while (iVar1 != 0) {
        uVar10 = 1;
        uVar6 = FUN_0052bed0((int)local_ec);
        FUN_005039d0(local_8c,uVar6,uVar10);
        uVar10 = 1;
        local_4._0_1_ = 4;
        uVar6 = FUN_0052bed0((int)local_ec);
        FUN_004f25c0(local_4c,uVar6,uVar10);
        uVar10 = 1;
        local_4._0_1_ = 5;
        uVar6 = FUN_0052bed0((int)local_ec);
        FUN_00504c40(local_ac,uVar6,uVar10);
        local_4 = CONCAT31(local_4._1_3_,6);
        pvVar7 = (void *)FUN_0052bed0((int)local_ec);
        FUN_0049c5d0(param_1,pvVar7);
        FUN_00513120((int)local_8c);
        iVar1 = FUN_0052bed0((int)local_8c);
        while (iVar1 != 0) {
          pvVar7 = (void *)FUN_0052bed0((int)local_8c);
          FUN_0049c5d0(param_1,pvVar7);
          FUN_005130d0((int)local_8c);
          iVar1 = FUN_0052bed0((int)local_8c);
        }
        FUN_00513120((int)local_ac);
        iVar1 = FUN_0052bed0((int)local_ac);
        while (iVar1 != 0) {
          pvVar7 = (void *)FUN_0052bed0((int)local_ac);
          FUN_0049c5d0(param_1,pvVar7);
          FUN_005130d0((int)local_ac);
          iVar1 = FUN_0052bed0((int)local_ac);
        }
        FUN_00513120((int)local_4c);
        iVar1 = FUN_0052bed0((int)local_4c);
        while (iVar1 != 0) {
          pvVar7 = (void *)FUN_0052bed0((int)local_4c);
          FUN_0049c5d0(param_1,pvVar7);
          FUN_005130d0((int)local_4c);
          iVar1 = FUN_0052bed0((int)local_4c);
        }
        local_4._1_3_ = (uint3)((uint)local_4 >> 8);
        local_4._0_1_ = 5;
        FUN_00504d40(local_ac);
        local_4._0_1_ = 4;
        FUN_004f26c0(local_4c);
        local_4 = CONCAT31(local_4._1_3_,3);
        FUN_00503ad0(local_8c);
        FUN_005130d0((int)local_ec);
        iVar1 = FUN_0052bed0((int)local_ec);
      }
      local_4._1_3_ = (uint3)((uint)local_4 >> 8);
      local_4 = CONCAT31(local_4._1_3_,2);
      FUN_00502eb0(local_ec);
      FUN_005130d0((int)local_cc);
      iVar1 = FUN_0052bed0((int)local_cc);
    }
    FUN_00513120((int)local_2c);
    iVar1 = FUN_0052bed0((int)local_2c);
    while (iVar1 != 0) {
      pvVar7 = (void *)FUN_0052bed0((int)local_2c);
      FUN_0049c5d0(param_1,pvVar7);
      FUN_005130d0((int)local_2c);
      iVar1 = FUN_0052bed0((int)local_2c);
    }
    FUN_00513120((int)local_6c);
    iVar1 = FUN_0052bed0((int)local_6c);
    while (iVar1 != 0) {
      pvVar7 = (void *)FUN_0052bed0((int)local_6c);
      FUN_0049c5d0(param_1,pvVar7);
      FUN_005130d0((int)local_6c);
      iVar1 = FUN_0052bed0((int)local_6c);
    }
    local_4._1_3_ = (uint3)((uint)local_4 >> 8);
    local_4._0_1_ = 1;
    FUN_004f26c0(local_6c);
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_00503ad0(local_2c);
    local_4 = 0xffffffff;
    FUN_004fff70(local_cc);
  }
  ExceptionList = local_c;
  return 1;
}

