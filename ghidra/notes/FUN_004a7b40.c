
void __thiscall FUN_004a7b40(void *this,uint *param_1,int *param_2)

{
  uint uVar1;
  bool bVar2;
  bool bVar3;
  bool bVar4;
  int *this_00;
  void *pvVar5;
  uint *puVar6;
  uint *puVar7;
  uint uVar8;
  int iVar9;
  undefined4 *puVar10;
  bool bVar11;
  undefined4 local_24;
  void *local_20;
  undefined4 local_1c;
  uint local_18;
  undefined4 local_14;
  void *local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00637ff3;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  this_00 = FUN_004a25c0((int *)this);
  if (this_00 == (int *)0x0) {
    ExceptionList = local_c;
    return;
  }
  FUN_004ece30(&local_24);
  local_10 = (void *)((int)this + 0x114);
  local_4 = 0;
  pvVar5 = FUN_004f5940(local_10,param_1);
  bVar11 = pvVar5 == (void *)0x0;
  if (param_2 == (int *)0x0) {
    param_2 = FUN_004f2d10(*(int *)(*(int *)((int)this + 0x14c) + 0x9c),param_1);
  }
  if (param_2 == (int *)0x0) goto LAB_004a7edb;
  bVar3 = false;
  bVar4 = false;
  puVar6 = FUN_004025b0(this_00,&local_18);
  local_4._0_1_ = 1;
  puVar7 = FUN_004025b0((void *)param_2[7],(uint *)&local_20);
  uVar8 = *puVar7;
  uVar1 = *puVar6;
  FUN_00619730();
  local_4._0_1_ = 0;
  FUN_00619730();
  if (uVar8 == uVar1) {
    local_20 = (void *)0x22;
    local_1c = 0x23;
    uVar8 = *param_1 >> 0x18;
    local_4 = CONCAT31(local_4._1_3_,2);
    if ((uVar8 < 0x22) || (0x22 < uVar8)) {
      local_18 = 0x23;
      local_14 = 0x24;
      if ((uVar8 < 0x23) || (0x23 < uVar8)) {
        bVar2 = false;
      }
      else {
        bVar2 = true;
      }
      FUN_00619730();
      if (bVar2) goto LAB_004a7c75;
      bVar2 = false;
    }
    else {
LAB_004a7c75:
      bVar2 = true;
    }
    local_4._0_1_ = 0;
    FUN_00619730();
    if (bVar2) {
      bVar4 = true;
      if ((*(int *)((int)this + 0x158) == 5) && (bVar11)) {
        bVar3 = true;
      }
    }
    else {
      local_20 = (void *)0x24;
      local_1c = 0x25;
      uVar8 = *param_1 >> 0x18;
      local_4 = CONCAT31(local_4._1_3_,3);
      if ((uVar8 < 0x24) || (0x24 < uVar8)) {
        local_18 = 0x25;
        local_14 = 0x26;
        if ((uVar8 < 0x25) || (0x25 < uVar8)) {
          bVar2 = false;
        }
        else {
          bVar2 = true;
        }
        FUN_00619730();
        if (bVar2) goto LAB_004a7d12;
        bVar2 = false;
      }
      else {
LAB_004a7d12:
        bVar2 = true;
      }
      local_4._0_1_ = 0;
      FUN_00619730();
      if (bVar2) {
        bVar4 = true;
        if (*(int *)((int)this + 0x158) == 4) {
joined_r0x004a7de9:
          bVar4 = true;
          if (bVar11) {
LAB_004a7e7a:
            bVar4 = true;
            bVar3 = true;
          }
        }
      }
      else {
        local_18 = 0x1c;
        local_14 = 0x20;
        if ((*param_1 >> 0x18 < 0x1c) || (0x1f < *param_1 >> 0x18)) {
          bVar2 = false;
        }
        else {
          bVar2 = true;
        }
        FUN_00619730();
        if (bVar2) {
          bVar4 = true;
          if (*(int *)((int)this + 0x158) == 3) goto joined_r0x004a7de9;
        }
        else {
          local_18 = 0x10;
          local_14 = 0x14;
          if ((*param_1 >> 0x18 < 0x10) || (0x13 < *param_1 >> 0x18)) {
            bVar2 = false;
          }
          else {
            bVar2 = true;
          }
          FUN_00619730();
          if (bVar2) {
            bVar4 = true;
            if (*(int *)((int)this + 0x158) == 2) goto joined_r0x004a7de9;
          }
          else {
            local_18 = 0x30;
            local_14 = 0x40;
            if ((*param_1 >> 0x18 < 0x30) || (0x3f < *param_1 >> 0x18)) {
              bVar2 = false;
            }
            else {
              bVar2 = true;
            }
            FUN_00619730();
            if (bVar2) {
              puVar6 = (uint *)FUN_0042d170(param_2,&local_18);
              local_4._0_1_ = 4;
              iVar9 = FUN_004ece60(puVar6);
              if ((iVar9 == 0) || ((param_2[0x1e] & 0x100U) != 0)) {
                bVar2 = true;
              }
              else {
                bVar2 = false;
              }
              local_4._0_1_ = 0;
              FUN_00619730();
              if (((bVar2) && (bVar11)) && (bVar4 = true, *(int *)((int)this + 0x158) == 1))
              goto LAB_004a7e7a;
            }
          }
        }
      }
    }
  }
  if (bVar3) {
    local_20 = (void *)FUN_00618b70(0x28);
    local_4._0_1_ = 5;
    if (local_20 == (void *)0x0) {
      puVar10 = (undefined4 *)0x0;
    }
    else {
      puVar10 = (undefined4 *)FUN_004acd80(local_20,param_1,0);
    }
    local_4._0_1_ = 0;
    FUN_004acba0(local_10,puVar10,0);
  }
  if (bVar4) {
                    /* WARNING: Load size is inaccurate */
    (**(code **)(*this + 100))(param_1,param_2);
  }
LAB_004a7edb:
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return;
}

