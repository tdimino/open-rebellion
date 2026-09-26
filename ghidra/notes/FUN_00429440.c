
undefined4 __thiscall FUN_00429440(void *this,uint *param_1)

{
  bool bVar1;
  byte bVar2;
  bool bVar3;
  uint *puVar4;
  uint uVar5;
  uint *puVar6;
  uint *puVar7;
  int iVar8;
  void *pvVar9;
  undefined4 uVar10;
  int *piVar11;
  uint local_28;
  int local_24;
  void *local_20;
  uint local_1c [4];
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062c440;
  local_c = ExceptionList;
  local_24 = 0;
  ExceptionList = &local_c;
  local_20 = this;
  FUN_004ece30(&local_28);
  local_4 = 0;
  puVar4 = (uint *)FUN_004f2d10(*(int *)((int)this + 0x9c),param_1);
  if (puVar4 == (uint *)0x0) goto LAB_00429c5d;
  local_1c[0] = 0x20;
  local_1c[1] = 0x30;
  if ((*param_1 >> 0x18 < 0x20) || (0x2f < *param_1 >> 0x18)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_00619730();
  if (bVar1) {
    puVar7 = (uint *)puVar4[7];
    puVar4 = FUN_004025b0((void *)puVar7[7],local_1c);
    local_4._0_1_ = 1;
    FUN_004f26d0(&local_28,puVar4);
    local_4._0_1_ = 0;
    FUN_00619730();
    local_1c[0] = 0x22;
    local_1c[1] = 0x28;
    if ((*param_1 >> 0x18 < 0x22) || (0x27 < *param_1 >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (bVar1) {
      local_24 = 10;
    }
    else {
      local_1c[0] = 0x28;
      local_1c[1] = 0x30;
      if ((*param_1 >> 0x18 < 0x28) || (0x2f < *param_1 >> 0x18)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (bVar1) {
        local_24 = 9;
      }
    }
LAB_00429c3c:
    iVar8 = FUN_004ece60(&local_28);
    if (iVar8 != 0) {
      pvVar9 = FUN_00429ce0(local_20,&local_28);
      iVar8 = local_24;
      if (pvVar9 == (void *)0x0) goto LAB_00429c5d;
      if (local_24 != 0) {
        puVar4 = FUN_004025b0(puVar7,local_1c);
        local_4._0_1_ = 0x15;
        piVar11 = FUN_0045c8e0(pvVar9,(ushort *)puVar4,iVar8);
        local_4 = (uint)local_4._1_3_ << 8;
        FUN_00619730();
        (**(code **)(*piVar11 + 0x6c))(param_1);
      }
    }
    local_4 = 0xffffffff;
    FUN_00619730();
    uVar10 = 1;
  }
  else {
    local_1c[0] = 0x14;
    local_1c[1] = 0x1c;
    if ((*param_1 >> 0x18 < 0x14) || (0x1b < *param_1 >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    puVar7 = param_1;
    if (bVar1) {
      pvVar9 = (void *)puVar4[7];
      puVar4 = FUN_004025b0(pvVar9,local_1c);
      local_1c[2] = 8;
      local_1c[3] = 0x10;
      local_4._0_1_ = 2;
      if ((*puVar4 >> 0x18 < 8) || (0xf < *puVar4 >> 0x18)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      local_4._0_1_ = 0;
      FUN_00619730();
      if (bVar1) {
        puVar7 = *(uint **)((int)pvVar9 + 0x1c);
        puVar4 = FUN_004025b0(puVar7,local_1c);
        local_1c[2] = 0xf2;
        local_1c[3] = 0xf3;
        local_4._0_1_ = 3;
        if ((*puVar4 >> 0x18 < 0xf2) || (0xf2 < *puVar4 >> 0x18)) {
          bVar1 = false;
        }
        else {
          bVar1 = true;
        }
        FUN_00619730();
        local_4._0_1_ = 0;
        FUN_00619730();
        if (bVar1) goto LAB_00429c5d;
        puVar4 = FUN_004025b0((void *)puVar7[7],local_1c);
        local_4._0_1_ = 4;
        FUN_004f26d0(&local_28,puVar4);
        local_4._0_1_ = 0;
        FUN_00619730();
        local_24 = 4;
      }
      goto LAB_00429c3c;
    }
    local_1c[0] = 0x1c;
    local_1c[1] = 0x20;
    uVar5 = *param_1 >> 0x18;
    local_4 = CONCAT31(local_4._1_3_,5);
    if ((uVar5 < 0x1c) || (0x1f < uVar5)) {
      local_1c[2] = 0x10;
      local_1c[3] = 0x14;
      if ((uVar5 < 0x10) || (0x13 < uVar5)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      bVar3 = false;
      if (bVar1) goto LAB_004296fa;
    }
    else {
LAB_004296fa:
      bVar3 = true;
    }
    local_4._0_1_ = 0;
    FUN_00619730();
    if (bVar3) {
      puVar4 = (uint *)puVar4[7];
      puVar6 = FUN_004025b0(puVar4,local_1c);
      local_1c[2] = 0x14;
      local_1c[3] = 0x1c;
      local_4._0_1_ = 6;
      if ((*puVar6 >> 0x18 < 0x14) || (0x1b < *puVar6 >> 0x18)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      local_4._0_1_ = 0;
      FUN_00619730();
      if (bVar1) {
        puVar7 = *(uint **)(puVar4[7] + 0x1c);
        puVar4 = FUN_004025b0(puVar7,local_1c);
        local_1c[2] = 0xf2;
        local_1c[3] = 0xf3;
        local_4._0_1_ = 7;
        if ((*puVar4 >> 0x18 < 0xf2) || (0xf2 < *puVar4 >> 0x18)) {
          bVar1 = false;
        }
        else {
          bVar1 = true;
        }
        FUN_00619730();
        local_4._0_1_ = 0;
        FUN_00619730();
        if (bVar1) goto LAB_00429c5d;
        puVar4 = FUN_004025b0((void *)puVar7[7],local_1c);
        local_4._0_1_ = 8;
        FUN_004f26d0(&local_28,puVar4);
        local_4._0_1_ = 0;
        FUN_00619730();
        local_24 = 4;
      }
      else {
        puVar6 = FUN_004025b0(puVar4,local_1c);
        local_1c[2] = 0x90;
        local_1c[3] = 0x98;
        local_4._0_1_ = 9;
        if ((*puVar6 >> 0x18 < 0x90) || (0x97 < *puVar6 >> 0x18)) {
          bVar1 = false;
        }
        else {
          bVar1 = true;
        }
        FUN_00619730();
        local_4._0_1_ = 0;
        FUN_00619730();
        if (bVar1) {
          puVar7 = FUN_004025b0((void *)puVar4[7],local_1c);
          local_4._0_1_ = 10;
          FUN_004f26d0(&local_28,puVar7);
          local_4._0_1_ = 0;
          FUN_00619730();
          local_24 = 10;
          puVar7 = puVar4;
        }
      }
      goto LAB_00429c3c;
    }
    local_1c[2] = 0x30;
    local_1c[3] = 0x40;
    if ((*param_1 >> 0x18 < 0x30) || (0x3f < *param_1 >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (!bVar1) {
      local_1c[2] = 0x90;
      local_1c[3] = 0x98;
      if ((*param_1 >> 0x18 < 0x90) || (0x97 < *param_1 >> 0x18)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (bVar1) {
        puVar7 = FUN_004025b0((void *)puVar4[7],local_1c);
        local_4._0_1_ = 0x12;
        FUN_004f26d0(&local_28,puVar7);
        local_4._0_1_ = 0;
        FUN_00619730();
        puVar7 = puVar4;
      }
      else {
        local_1c[2] = 8;
        local_1c[3] = 0x10;
        if ((*param_1 >> 0x18 < 8) || (0xf < *param_1 >> 0x18)) {
          bVar1 = false;
        }
        else {
          bVar1 = true;
        }
        FUN_00619730();
        if (bVar1) {
          puVar7 = (uint *)puVar4[7];
          puVar4 = FUN_004025b0(puVar7,local_1c);
          local_1c[2] = 0xf2;
          local_1c[3] = 0xf3;
          local_4._0_1_ = 0x13;
          if ((*puVar4 >> 0x18 < 0xf2) || (0xf2 < *puVar4 >> 0x18)) {
            bVar1 = false;
          }
          else {
            bVar1 = true;
          }
          FUN_00619730();
          local_4._0_1_ = 0;
          FUN_00619730();
          if (bVar1) goto LAB_00429c5d;
          puVar4 = FUN_004025b0((void *)puVar7[7],local_1c);
          local_4._0_1_ = 0x14;
          FUN_004f26d0(&local_28,puVar4);
          local_4._0_1_ = 0;
          FUN_00619730();
          local_24 = 4;
        }
      }
      goto LAB_00429c3c;
    }
    puVar7 = (uint *)puVar4[7];
    puVar6 = FUN_004025b0(puVar7,local_1c);
    local_1c[2] = 0x14;
    local_1c[3] = 0x1c;
    local_4._0_1_ = 0xb;
    if ((*puVar6 >> 0x18 < 0x14) || (0x1b < *puVar6 >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    local_4._0_1_ = 0;
    FUN_00619730();
    if (!bVar1) {
      puVar6 = FUN_004025b0(puVar7,local_1c);
      local_1c[2] = 0x90;
      local_1c[3] = 0x98;
      local_4._0_1_ = 0xe;
      if ((*puVar6 >> 0x18 < 0x90) || (0x97 < *puVar6 >> 0x18)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      local_4._0_1_ = 0;
      FUN_00619730();
      if (bVar1) {
        puVar6 = FUN_004025b0((void *)puVar7[7],local_1c);
        local_4._0_1_ = 0xf;
        FUN_004f26d0(&local_28,puVar6);
        local_4._0_1_ = 0;
        FUN_00619730();
        bVar2 = 0;
        puVar4 = FUN_0042d170(puVar4,local_1c);
        local_4._0_1_ = 0x10;
        piVar11 = FUN_004f3000(*(int *)((int)local_20 + 0x9c),puVar4);
        local_4._0_1_ = 0;
        FUN_00619730();
        if ((piVar11 != (int *)0x0) && (iVar8 = FUN_00520b70((int)piVar11), iVar8 == 0)) {
          bVar2 = 1;
        }
        local_24 = bVar2 + 10;
      }
      else {
        puVar4 = FUN_004025b0(puVar7,local_1c);
        local_1c[2] = 0x98;
        local_1c[3] = 0xa0;
        local_4._0_1_ = 0x11;
        if ((*puVar4 >> 0x18 < 0x98) || (0x9f < *puVar4 >> 0x18)) {
          bVar1 = false;
        }
        else {
          bVar1 = true;
        }
        FUN_00619730();
        local_4._0_1_ = 0;
        FUN_00619730();
        puVar7 = param_1;
        if (bVar1) goto LAB_00429c5d;
      }
      goto LAB_00429c3c;
    }
    puVar4 = FUN_004025b0((void *)puVar7[7],local_1c);
    local_1c[2] = 0xf2;
    local_1c[3] = 0xf3;
    local_4._0_1_ = 0xc;
    if ((*puVar4 >> 0x18 < 0xf2) || (0xf2 < *puVar4 >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    local_4._0_1_ = 0;
    FUN_00619730();
    if (!bVar1) {
      puVar7 = *(uint **)(puVar7[7] + 0x1c);
      puVar4 = FUN_004025b0((void *)puVar7[7],local_1c);
      local_4._0_1_ = 0xd;
      FUN_004f26d0(&local_28,puVar4);
      local_4._0_1_ = 0;
      FUN_00619730();
      local_24 = 4;
      goto LAB_00429c3c;
    }
LAB_00429c5d:
    local_4 = 0xffffffff;
    FUN_00619730();
    uVar10 = 0;
  }
  ExceptionList = local_c;
  return uVar10;
}

