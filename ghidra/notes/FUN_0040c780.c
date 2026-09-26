
uint __thiscall FUN_0040c780(void *this,int *param_1)

{
  int iVar1;
  int *piVar2;
  uint uVar3;
  void *pvVar4;
  undefined4 uVar5;
  uint uVar6;
  int *piVar7;
  bool bVar8;
  undefined4 uVar9;
  undefined4 *puVar10;
  uint local_10c;
  int local_108 [6];
  int local_f0;
  int local_ec;
  undefined4 local_e8 [7];
  int local_cc;
  undefined4 local_c8 [7];
  int local_ac;
  undefined4 local_a8 [7];
  int iStack_8c;
  undefined4 local_88 [7];
  int local_6c;
  undefined4 local_68 [7];
  undefined4 auStack_4c [7];
  int iStack_30;
  undefined4 local_2c [8];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062a23a;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = 0;
  *(undefined4 *)((int)this + 8) = 0;
  FUN_004ece30(&local_10c);
  local_4 = 0;
  FUN_0041e1d0(&local_10c);
  iVar1 = FUN_00505f60(&local_10c);
  *(int *)((int)this + 8) = iVar1;
  uVar6 = (uint)(iVar1 != 0);
  local_4 = 0xffffffff;
  FUN_00619730();
  if ((*(int *)((int)this + 4) == 0) || (*(int *)((int)this + 8) == 0)) goto LAB_0040cf9a;
  FUN_004fd400(local_68);
  local_4 = 1;
  local_10c = 0;
  piVar2 = (int *)FUN_005969d0(local_108);
  piVar7 = local_108;
  for (iVar1 = 7; iVar1 != 0; iVar1 = iVar1 + -1) {
    *piVar7 = *piVar2;
    piVar2 = piVar2 + 1;
    piVar7 = piVar7 + 1;
  }
  if ((local_108[1] == 0) || (local_108[3] == 0)) {
    if (local_108[1] == 0) {
      if (local_108[3] != 0) {
        local_10c = 2;
      }
    }
    else {
      local_10c = 1;
    }
  }
  else {
    uVar3 = *(uint *)(*(int *)((int)this + 8) + 0x78) >> 6 & 3;
    if (uVar3 == 3) {
      local_10c = 3;
    }
    else {
      local_10c = 2 - (uVar3 != 1);
    }
  }
  if (((*(int *)((int)this + 0xc) == 0) && (local_f0 != 0)) && (local_10c == 0)) {
    *param_1 = 1;
  }
  if (*param_1 == 0) {
    if (local_10c == 0) {
      if ((local_108[0] == 0) || (local_108[2] == 0)) {
        if (local_108[0] == 0) {
          if (local_108[2] != 0) {
            local_10c = 2;
          }
        }
        else {
          local_10c = 1;
        }
      }
      else {
        uVar3 = *(uint *)(*(int *)((int)this + 8) + 0x78) >> 6 & 3;
        if (uVar3 == 3) {
          local_10c = 3;
        }
        else {
          local_10c = 2 - (uVar3 != 1);
        }
      }
    }
    FUN_0050cfd0(*(void **)((int)this + 8),1);
    FUN_0050ceb0(*(void **)((int)this + 8),local_10c);
  }
  iVar1 = FUN_00506e10();
  if (iVar1 == 0) {
LAB_0040c97d:
    uVar6 = 0;
  }
  else {
    pvVar4 = (void *)FUN_00516ee0(iVar1);
    FUN_004fd490(local_68,pvVar4);
    if (uVar6 == 0) goto LAB_0040c97d;
    uVar6 = 1;
  }
  FUN_004ffe70(local_e8,*(undefined4 *)((int)this + 8),3);
  local_4 = CONCAT31(local_4._1_3_,2);
  FUN_00513120((int)local_e8);
  while (local_cc != 0) {
    uVar9 = 1;
    uVar5 = FUN_0052bed0((int)local_e8);
    FUN_00502db0(local_108,uVar5,uVar9);
    local_4 = CONCAT31(local_4._1_3_,3);
    FUN_00513120((int)local_108);
    while (local_ec != 0) {
      uVar9 = 1;
      uVar5 = FUN_0052bed0((int)local_108);
      FUN_005039d0(local_c8,uVar5,uVar9);
      local_4 = CONCAT31(local_4._1_3_,4);
      FUN_00513120((int)local_c8);
      while (local_ac != 0) {
        puVar10 = local_68;
        piVar2 = (int *)FUN_0052bed0((int)local_c8);
        uVar6 = FUN_0040d260(this,piVar2,puVar10);
        FUN_005130d0((int)local_c8);
      }
      uVar9 = 1;
      uVar5 = FUN_0052bed0((int)local_108);
      FUN_004f25c0(local_88,uVar5,uVar9);
      local_4 = CONCAT31(local_4._1_3_,5);
      FUN_00513120((int)local_88);
      while (local_6c != 0) {
        pvVar4 = (void *)FUN_0052bed0((int)local_88);
        bVar8 = false;
        if (*(int *)((int)this + 4) != 0) {
          bVar8 = *(int *)((int)this + 8) != 0;
        }
        uVar6 = 0;
        if (bVar8) {
          uVar6 = FUN_004f6df0(pvVar4,&local_10c);
        }
        if (uVar6 != 0) {
          FUN_00596930(local_2c,local_10c);
        }
        FUN_005130d0((int)local_88);
      }
      local_4._1_3_ = (undefined3)((uint)local_4 >> 8);
      local_4._0_1_ = 4;
      FUN_004f26c0(local_88);
      local_4 = CONCAT31(local_4._1_3_,3);
      FUN_00503ad0(local_c8);
      FUN_005130d0((int)local_108);
    }
    local_4._1_3_ = (undefined3)((uint)local_4 >> 8);
    local_4 = CONCAT31(local_4._1_3_,2);
    FUN_00502eb0(local_108);
    FUN_005130d0((int)local_e8);
  }
  FUN_005039d0(local_a8,*(undefined4 *)((int)this + 8),1);
  local_4 = CONCAT31(local_4._1_3_,6);
  FUN_00513120((int)local_a8);
  while (iStack_8c != 0) {
    puVar10 = local_68;
    piVar2 = (int *)FUN_0052bed0((int)local_a8);
    uVar6 = FUN_0040d260(this,piVar2,puVar10);
    FUN_005130d0((int)local_a8);
  }
  local_4._1_3_ = (undefined3)((uint)local_4 >> 8);
  local_4._0_1_ = 2;
  FUN_00503ad0(local_a8);
  local_4._0_1_ = 1;
  FUN_004fff70(local_e8);
  FUN_004ffe70(local_88,*(undefined4 *)((int)this + 8),3);
  local_4 = CONCAT31(local_4._1_3_,7);
  FUN_00513120((int)local_88);
  while (local_6c != 0) {
    uVar9 = 1;
    uVar5 = FUN_0052bed0((int)local_88);
    FUN_00502db0(local_108,uVar5,uVar9);
    local_4 = CONCAT31(local_4._1_3_,8);
    FUN_00513120((int)local_108);
    while (local_ec != 0) {
      uVar9 = 1;
      uVar5 = FUN_0052bed0((int)local_108);
      FUN_005039d0(local_a8,uVar5,uVar9);
      local_4 = CONCAT31(local_4._1_3_,9);
      FUN_00513120((int)local_a8);
      while (iStack_8c != 0) {
        piVar2 = (int *)FUN_0052bed0((int)local_a8);
        FUN_005130d0((int)local_a8);
        uVar6 = FUN_0040d320(this,piVar2,local_68);
      }
      uVar9 = 1;
      uVar5 = FUN_0052bed0((int)local_108);
      FUN_004f25c0(local_e8,uVar5,uVar9);
      local_4 = CONCAT31(local_4._1_3_,10);
      FUN_00513120((int)local_e8);
      while (local_cc != 0) {
        pvVar4 = (void *)FUN_0052bed0((int)local_e8);
        FUN_005130d0((int)local_e8);
        uVar6 = FUN_004f6df0(pvVar4,&local_10c);
        if (uVar6 != 0) {
          FUN_00596930(local_2c,local_10c);
        }
      }
      local_4._1_3_ = (undefined3)((uint)local_4 >> 8);
      local_4._0_1_ = 9;
      FUN_004f26c0(local_e8);
      local_4 = CONCAT31(local_4._1_3_,8);
      FUN_00503ad0(local_a8);
      FUN_005130d0((int)local_108);
    }
    local_4._1_3_ = (undefined3)((uint)local_4 >> 8);
    local_4 = CONCAT31(local_4._1_3_,7);
    FUN_00502eb0(local_108);
    FUN_005130d0((int)local_88);
  }
  FUN_005039d0(local_c8,*(undefined4 *)((int)this + 8),1);
  local_4 = CONCAT31(local_4._1_3_,0xb);
  FUN_00513120((int)local_c8);
  while (local_ac != 0) {
    piVar2 = (int *)FUN_0052bed0((int)local_c8);
    FUN_005130d0((int)local_c8);
    uVar6 = FUN_0040d320(this,piVar2,local_68);
  }
  local_4._1_3_ = (undefined3)((uint)local_4 >> 8);
  local_4._0_1_ = 7;
  FUN_00503ad0(local_c8);
  local_4._0_1_ = 1;
  FUN_004fff70(local_88);
  FUN_004ffe70(auStack_4c,*(undefined4 *)((int)this + 8),3);
  local_4 = CONCAT31(local_4._1_3_,0xc);
  FUN_00513120((int)auStack_4c);
  while (iStack_30 != 0) {
    uVar9 = 1;
    uVar5 = FUN_0052bed0((int)auStack_4c);
    FUN_005001b0(local_c8,uVar5,uVar9);
    local_4 = CONCAT31(local_4._1_3_,0xd);
    FUN_00513120((int)local_c8);
    while (local_ac != 0) {
      puVar10 = local_68;
      piVar2 = (int *)FUN_0052bed0((int)local_c8);
      uVar6 = FUN_0040cfc0(this,piVar2,puVar10);
      FUN_005130d0((int)local_c8);
    }
    uVar9 = 1;
    uVar5 = FUN_0052bed0((int)auStack_4c);
    FUN_005003d0(local_a8,uVar5,uVar9);
    local_4 = CONCAT31(local_4._1_3_,0xe);
    FUN_00513120((int)local_a8);
    while (iStack_8c != 0) {
      puVar10 = local_68;
      piVar2 = (int *)FUN_0052bed0((int)local_a8);
      uVar6 = FUN_0040d170(this,piVar2,puVar10);
      FUN_005130d0((int)local_a8);
    }
    uVar9 = 1;
    uVar5 = FUN_0052bed0((int)auStack_4c);
    FUN_00502db0(local_e8,uVar5,uVar9);
    local_4 = CONCAT31(local_4._1_3_,0xf);
    FUN_00513120((int)local_e8);
    while (local_cc != 0) {
      puVar10 = local_68;
      piVar2 = (int *)FUN_0052bed0((int)local_e8);
      uVar6 = FUN_0040d510(this,piVar2,puVar10);
      FUN_005130d0((int)local_e8);
    }
    local_4._1_3_ = (undefined3)((uint)local_4 >> 8);
    local_4._0_1_ = 0xe;
    FUN_00502eb0(local_e8);
    local_4._0_1_ = 0xd;
    FUN_005004d0(local_a8);
    local_4 = CONCAT31(local_4._1_3_,0xc);
    FUN_00500230(local_c8);
    FUN_005130d0((int)auStack_4c);
  }
  local_4._1_3_ = (undefined3)((uint)local_4 >> 8);
  local_4 = CONCAT31(local_4._1_3_,1);
  FUN_004fff70(auStack_4c);
  local_4 = 0xffffffff;
  FUN_004fd4d0(local_68);
LAB_0040cf9a:
  thunk_FUN_005975a0();
  ExceptionList = pvStack_c;
  return uVar6;
}

