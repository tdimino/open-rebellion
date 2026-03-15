
void __thiscall FUN_004ebe40(void *this,int param_1)

{
  bool bVar1;
  uint uVar2;
  uint *puVar3;
  int *this_00;
  uint uVar4;
  undefined4 uVar5;
  void *pvVar6;
  int iVar7;
  int iVar8;
  undefined4 *puVar9;
  undefined1 *puVar10;
  undefined1 local_13c [4];
  int *local_138;
  uint local_134;
  undefined4 local_130;
  undefined4 local_12c [7];
  int local_110;
  undefined4 local_10c [7];
  int local_f0;
  undefined4 local_ec [7];
  int local_d0;
  undefined4 local_cc [7];
  int local_b0;
  undefined4 local_ac [7];
  int local_90;
  undefined4 local_8c [7];
  int local_70;
  undefined4 local_6c [7];
  int local_50;
  undefined4 local_4c [7];
  int local_30;
  undefined4 local_2c [7];
  int local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063eae8;
  local_c = ExceptionList;
  if (*(int *)((int)this + 0x1c) != 0) {
    ExceptionList = &local_c;
    *(undefined4 *)((int)this + 0x1c) = 0;
    puVar9 = (undefined4 *)((int)this + 0x40);
    for (iVar7 = 0x16; iVar7 != 0; iVar7 = iVar7 + -1) {
      *puVar9 = 0;
      puVar9 = puVar9 + 1;
    }
    puVar3 = (uint *)FUN_00403040(this,local_13c);
    local_4 = 0;
    this_00 = FUN_004f3220(param_1,puVar3);
    local_4 = 0xffffffff;
    local_138 = this_00;
    FUN_00619730();
    puVar3 = (uint *)((int)this + 0x24);
    FUN_004fcdd0(puVar3);
    if (param_1 == 1) {
      iVar7 = 1;
    }
    else if (param_1 == 2) {
      iVar7 = 2;
    }
    else {
      iVar7 = 0;
    }
    if (param_1 != 1) {
      iVar8 = 1;
    }
    else if (param_1 != 1) {
      iVar8 = 0;
    }
    else {
      iVar8 = 2;
    }
    bVar1 = false;
    *(uint *)((int)this + 0x20) = (uint)this_00[9] >> 6 & 3;
    *(undefined4 *)((int)this + 0x98) = 0;
    if ((*(byte *)(this_00 + 0x14) & 8) != 0) {
      *(uint *)((int)this + 0x28) = *(uint *)((int)this + 0x28) | 8;
    }
    if (*(int *)((int)this + 0x20) == iVar7) {
      uVar4 = *puVar3 | 4;
    }
    else if (*(int *)((int)this + 0x20) == iVar8) {
      uVar4 = *puVar3 | 8;
    }
    else {
      uVar4 = *puVar3 | 0x10;
    }
    *puVar3 = uVar4;
    uVar4 = *(uint *)((int)this + 0x18) >> 0x18;
    local_134 = 0x90;
    local_130 = 0x92;
    if ((0x8f < uVar4) && (uVar4 < 0x92)) {
      bVar1 = true;
    }
    FUN_00619730();
    if (bVar1) {
      *(uint *)((int)this + 0x28) = *(uint *)((int)this + 0x28) | 4;
    }
    if ((*(byte *)(this_00 + 0x22) & 2) != 0) {
      *puVar3 = *puVar3 | 0x20000000;
      *(int *)((int)this + 0x44) = this_00[0x17];
      *(int *)((int)this + 0x48) = this_00[0x19];
      iVar7 = this_00[0x17];
      iVar8 = this_00[0x18];
      *(int *)((int)this + 0x70) = iVar7 - iVar8;
      if (0 < iVar7 - iVar8) {
        *puVar3 = *puVar3 | 0x20;
      }
      iVar7 = this_00[0x19];
      iVar8 = this_00[0x1a];
      *(int *)((int)this + 0x6c) = iVar7 - iVar8;
      if ((0 < iVar7 - iVar8) && ((*(byte *)puVar3 & 0x20) != 0)) {
        *puVar3 = *puVar3 | 0x40;
      }
      if (((*(byte *)(this_00 + 0x22) & 0x20) != 0) &&
         (*puVar3 = *puVar3 | 2, (*(byte *)puVar3 & 0x18) != 0)) {
        uVar5 = thunk_FUN_004fcf00();
        *(undefined4 *)((int)this + 0x3c) = uVar5;
      }
      if ((*(byte *)puVar3 & 4) != 0) {
        *(undefined4 *)((int)this + 0x40) = 100;
      }
      uVar4 = 0;
      local_134 = 0;
      FUN_00504c40(local_6c,this_00,3);
      local_4 = 1;
      FUN_00513120((int)local_6c);
      uVar2 = local_134;
      while (local_50 != 0) {
        puVar3 = &local_134;
        pvVar6 = (void *)FUN_0052bed0((int)local_6c);
        puVar3 = (uint *)FUN_00402d80(pvVar6,puVar3);
        local_4._0_1_ = 2;
        iVar7 = FUN_0042e190(*(void **)((int)this + 0x34),puVar3);
        local_4 = CONCAT31(local_4._1_3_,1);
        FUN_00619730();
        iVar8 = FUN_0052bed0((int)local_6c);
        uVar4 = uVar4 | *(uint *)(iVar8 + 0x50) & 1;
        *(int *)((int)this + 0x74) = *(int *)((int)this + 0x74) + 1;
        *(int *)((int)this + 0x7c) = *(int *)((int)this + 0x7c) + *(int *)(iVar7 + 0x20);
        FUN_005130d0((int)local_6c);
        uVar2 = uVar4;
      }
      local_134 = uVar2;
      if ((*(byte *)(this_00 + 0x22) & 1) != 0) {
        if ((*(uint *)((int)this + 0x18) & 0xffffff) == 0x109) {
          *(uint *)((int)this + 0x28) = *(uint *)((int)this + 0x28) | 0x20000000;
        }
        iVar7 = FUN_00526a60(this_00,*(undefined4 *)((int)this + 0x20),1);
        if (iVar7 != 0) {
          *(uint *)((int)this + 0x24) = *(uint *)((int)this + 0x24) | 0x80000000;
        }
        if ((*(byte *)(this_00 + 0x22) & 4) != 0) {
          *(uint *)((int)this + 0x24) = *(uint *)((int)this + 0x24) | 1;
        }
        *(uint *)((int)this + 0x24) = *(uint *)((int)this + 0x24) | 0x40000000;
        iVar7 = FUN_00507270(this_00,param_1);
        *(int *)((int)this + 0x40) = iVar7;
        iVar7 = FUN_0052d370(this_00,*(undefined4 *)((int)this + 0x20),3);
        *(int *)((int)this + 0x4c) = iVar7;
        iVar7 = FUN_0052cf40(this_00,*(undefined4 *)((int)this + 0x20),3);
        *(int *)((int)this + 0x50) = iVar7;
        uVar4 = 0;
        FUN_0052cab0(local_ac,this_00,3);
        local_4 = CONCAT31(local_4._1_3_,3);
        FUN_00513120((int)local_ac);
        while (local_90 != 0) {
          puVar10 = local_13c;
          pvVar6 = (void *)FUN_0052bed0((int)local_ac);
          puVar3 = (uint *)FUN_00402d80(pvVar6,puVar10);
          local_4._0_1_ = 4;
          iVar7 = FUN_0042e190(*(void **)((int)this + 0x34),puVar3);
          local_4 = CONCAT31(local_4._1_3_,3);
          FUN_00619730();
          iVar8 = FUN_0052bed0((int)local_ac);
          uVar4 = uVar4 | *(uint *)(iVar8 + 0x50) & 1;
          *(int *)((int)this + 0x54) = *(int *)((int)this + 0x54) + 1;
          *(int *)((int)this + 0x84) = *(int *)((int)this + 0x84) + *(int *)(iVar7 + 0x20);
          FUN_005130d0((int)local_ac);
        }
        if ((((0 < *(int *)((int)this + 0x54)) && (uVar4 != 0)) &&
            ((*(byte *)((int)this + 0x24) & 3) == 0)) &&
           (iVar7 = FUN_00509670(this_00,0), iVar7 != 0)) {
          if (*(int *)(iVar7 + 0x70) != 0) {
            *(uint *)((int)this + 0x24) = *(uint *)((int)this + 0x24) | 0x400;
          }
          if (*(int *)(iVar7 + 0x58) == 0) {
            uVar4 = *(uint *)((int)this + 0x24);
            *(uint *)((int)this + 0x24) = uVar4 | 0x80;
            if (1 < *(int *)((int)this + 0x54)) {
              *(uint *)((int)this + 0x24) = uVar4 | 0x480;
            }
          }
        }
        uVar4 = 0;
        FUN_0052c740(local_ec,this_00,3);
        local_4 = CONCAT31(local_4._1_3_,5);
        FUN_00513120((int)local_ec);
        while (local_d0 != 0) {
          puVar10 = local_13c;
          pvVar6 = (void *)FUN_0052bed0((int)local_ec);
          puVar3 = (uint *)FUN_00402d80(pvVar6,puVar10);
          local_4._0_1_ = 6;
          iVar7 = FUN_0042e190(*(void **)((int)this + 0x34),puVar3);
          local_4 = CONCAT31(local_4._1_3_,5);
          FUN_00619730();
          iVar8 = FUN_0052bed0((int)local_ec);
          uVar4 = uVar4 | *(uint *)(iVar8 + 0x50) & 1;
          *(int *)((int)this + 0x58) = *(int *)((int)this + 0x58) + 1;
          *(int *)((int)this + 0x84) = *(int *)((int)this + 0x84) + *(int *)(iVar7 + 0x20);
          FUN_005130d0((int)local_ec);
          this_00 = local_138;
        }
        if (((0 < *(int *)((int)this + 0x58)) && (uVar4 != 0)) &&
           (((*(byte *)((int)this + 0x24) & 3) == 0 && (iVar7 = FUN_00509670(this_00,2), iVar7 != 0)
            ))) {
          if (*(int *)(iVar7 + 0x70) != 0) {
            *(uint *)((int)this + 0x24) = *(uint *)((int)this + 0x24) | 0x800;
          }
          if (*(int *)(iVar7 + 0x58) == 0) {
            uVar4 = *(uint *)((int)this + 0x24);
            *(uint *)((int)this + 0x24) = uVar4 | 0x100;
            if (1 < *(int *)((int)this + 0x58)) {
              *(uint *)((int)this + 0x24) = uVar4 | 0x500;
            }
          }
        }
        uVar4 = 0;
        FUN_0052c420(local_10c,this_00,3);
        local_4 = CONCAT31(local_4._1_3_,7);
        FUN_00513120((int)local_10c);
        while (local_f0 != 0) {
          puVar10 = local_13c;
          pvVar6 = (void *)FUN_0052bed0((int)local_10c);
          puVar3 = (uint *)FUN_00402d80(pvVar6,puVar10);
          local_4._0_1_ = 8;
          iVar7 = FUN_0042e190(*(void **)((int)this + 0x34),puVar3);
          local_4 = CONCAT31(local_4._1_3_,7);
          FUN_00619730();
          iVar8 = FUN_0052bed0((int)local_10c);
          uVar4 = uVar4 | *(uint *)(iVar8 + 0x50) & 1;
          *(int *)((int)this + 0x5c) = *(int *)((int)this + 0x5c) + 1;
          *(int *)((int)this + 0x84) = *(int *)((int)this + 0x84) + *(int *)(iVar7 + 0x20);
          FUN_005130d0((int)local_10c);
        }
        if (((0 < *(int *)((int)this + 0x5c)) && (uVar4 != 0)) &&
           (((*(byte *)((int)this + 0x24) & 3) == 0 && (iVar7 = FUN_00509670(this_00,1), iVar7 != 0)
            ))) {
          if (*(int *)(iVar7 + 0x70) != 0) {
            *(uint *)((int)this + 0x24) = *(uint *)((int)this + 0x24) | 0x1000;
          }
          if (*(int *)(iVar7 + 0x58) == 0) {
            *(uint *)((int)this + 0x24) = *(uint *)((int)this + 0x24) | 0x200;
          }
        }
        uVar4 = 0;
        FUN_005278c0(local_4c,this_00,3);
        local_4 = CONCAT31(local_4._1_3_,9);
        FUN_00513120((int)local_4c);
        while (local_30 != 0) {
          puVar10 = local_13c;
          pvVar6 = (void *)FUN_0052bed0((int)local_4c);
          puVar3 = (uint *)FUN_00402d80(pvVar6,puVar10);
          local_4._0_1_ = 10;
          iVar7 = FUN_0042e190(*(void **)((int)this + 0x34),puVar3);
          local_4 = CONCAT31(local_4._1_3_,9);
          FUN_00619730();
          iVar8 = FUN_0052bed0((int)local_4c);
          uVar4 = uVar4 | *(uint *)(iVar8 + 0x50) & 1;
          *(int *)((int)this + 0x60) = *(int *)((int)this + 0x60) + 1;
          *(int *)((int)this + 0x80) = *(int *)((int)this + 0x80) + *(int *)(iVar7 + 0x20);
          FUN_005130d0((int)local_4c);
        }
        if ((uVar4 != 0) && (1 < *(int *)((int)this + 0x60))) {
          *(uint *)((int)this + 0x24) = *(uint *)((int)this + 0x24) | 0x400;
        }
        FUN_0051c270(local_2c,local_138,3);
        local_4 = CONCAT31(local_4._1_3_,0xb);
        FUN_00513120((int)local_2c);
        while (local_10 != 0) {
          puVar10 = local_13c;
          pvVar6 = (void *)FUN_0052bed0((int)local_2c);
          puVar3 = (uint *)FUN_00402d80(pvVar6,puVar10);
          local_4._0_1_ = 0xc;
          iVar7 = FUN_0042e190(*(void **)((int)this + 0x34),puVar3);
          local_4 = CONCAT31(local_4._1_3_,0xb);
          FUN_00619730();
          *(int *)((int)this + 100) = *(int *)((int)this + 100) + 1;
          *(int *)((int)this + 0x80) = *(int *)((int)this + 0x80) + *(int *)(iVar7 + 0x20);
          FUN_005130d0((int)local_2c);
        }
        uVar4 = 0;
        FUN_005272e0(local_12c,local_138,3);
        local_4 = CONCAT31(local_4._1_3_,0xd);
        FUN_00513120((int)local_12c);
        while (local_110 != 0) {
          puVar10 = local_13c;
          pvVar6 = (void *)FUN_0052bed0((int)local_12c);
          puVar3 = (uint *)FUN_00402d80(pvVar6,puVar10);
          local_4._0_1_ = 0xe;
          iVar7 = FUN_0042e190(*(void **)((int)this + 0x34),puVar3);
          local_4 = CONCAT31(local_4._1_3_,0xd);
          FUN_00619730();
          iVar8 = FUN_0052bed0((int)local_12c);
          uVar4 = uVar4 | *(uint *)(iVar8 + 0x50) & 1;
          *(int *)((int)this + 0x68) = *(int *)((int)this + 0x68) + 1;
          *(int *)((int)this + 0x80) = *(int *)((int)this + 0x80) + *(int *)(iVar7 + 0x20);
          FUN_005130d0((int)local_12c);
        }
        FUN_00527550(local_8c,local_138,3);
        local_4 = CONCAT31(local_4._1_3_,0xf);
        FUN_00513120((int)local_8c);
        while (local_70 != 0) {
          puVar10 = local_13c;
          pvVar6 = (void *)FUN_0052bed0((int)local_8c);
          puVar3 = (uint *)FUN_00402d80(pvVar6,puVar10);
          local_4._0_1_ = 0x10;
          iVar7 = FUN_0042e190(*(void **)((int)this + 0x34),puVar3);
          local_4 = CONCAT31(local_4._1_3_,0xf);
          FUN_00619730();
          iVar8 = FUN_0052bed0((int)local_8c);
          uVar4 = uVar4 | *(uint *)(iVar8 + 0x50) & 1;
          *(int *)((int)this + 0x68) = *(int *)((int)this + 0x68) + 1;
          *(int *)((int)this + 0x80) = *(int *)((int)this + 0x80) + *(int *)(iVar7 + 0x20);
          FUN_005130d0((int)local_8c);
        }
        if ((uVar4 != 0) && (1 < *(int *)((int)this + 0x68))) {
          *(uint *)((int)this + 0x24) = *(uint *)((int)this + 0x24) | 0x400;
        }
        uVar4 = 0;
        FUN_005039d0(local_cc,local_138,3);
        local_4 = CONCAT31(local_4._1_3_,0x11);
        FUN_00513120((int)local_cc);
        while (local_b0 != 0) {
          puVar10 = local_13c;
          pvVar6 = (void *)FUN_0052bed0((int)local_cc);
          puVar3 = (uint *)FUN_00402d80(pvVar6,puVar10);
          local_4._0_1_ = 0x12;
          iVar7 = FUN_0042e190(*(void **)((int)this + 0x34),puVar3);
          local_4 = CONCAT31(local_4._1_3_,0x11);
          FUN_00619730();
          iVar8 = FUN_0052bed0((int)local_cc);
          uVar4 = uVar4 | *(uint *)(iVar8 + 0x50) & 1;
          *(int *)((int)this + 0x78) = *(int *)((int)this + 0x78) + 1;
          *(int *)((int)this + 0x88) = *(int *)((int)this + 0x88) + *(int *)(iVar7 + 0x20);
          FUN_005130d0((int)local_cc);
        }
        if ((uVar4 != 0) && (0 < *(int *)((int)this + 0x78))) {
          *(uint *)((int)this + 0x24) = *(uint *)((int)this + 0x24) | 0x400;
        }
        if ((*(uint *)((int)this + 0x24) & 4) != 0) {
          *(undefined4 *)((int)this + 0x8c) = 1;
          if (0 < -*(int *)((int)this + 0x40) + 0x3c) {
            *(int *)((int)this + 0x8c) = (-*(int *)((int)this + 0x40) + 0x45) / 10 + 1;
          }
          if ((*(uint *)((int)this + 0x24) & 1) != 0) {
            *(int *)((int)this + 0x8c) = *(int *)((int)this + 0x8c) << 1;
          }
          if ((local_134 != 0) && (*(int *)((int)this + 0x8c) < *(int *)((int)this + 0x74))) {
            *(uint *)((int)this + 0x24) = *(uint *)((int)this + 0x24) | 0x400;
          }
          iVar7 = (*(int *)((int)this + 0x50) + *(int *)((int)this + 0x4c)) / 2 +
                  *(int *)((int)this + 0x5c) + *(int *)((int)this + 0x54) +
                  *(int *)((int)this + 0x58);
          if (iVar7 < *(int *)((int)this + 0x8c)) {
            iVar7 = *(int *)((int)this + 0x8c);
          }
          iVar7 = *(int *)((int)this + 0x74) - iVar7;
          *(int *)((int)this + 0x90) = iVar7;
          if (iVar7 < 0) {
            *(uint *)((int)this + 0x24) = *(uint *)((int)this + 0x24) | 0x10000000;
          }
          *(int *)((int)this + 0x94) =
               *(int *)((int)this + 0x88) + *(int *)((int)this + 0x84) + *(int *)((int)this + 0x80)
               + *(int *)((int)this + 0x7c);
        }
        local_4._1_3_ = (undefined3)((uint)local_4 >> 8);
        local_4._0_1_ = 0xf;
        FUN_00503ad0(local_cc);
        local_4._0_1_ = 0xd;
        FUN_00527650(local_8c);
        local_4._0_1_ = 0xb;
        FUN_00527360(local_12c);
        local_4._0_1_ = 9;
        FUN_0051c2f0(local_2c);
        local_4._0_1_ = 7;
        FUN_005279c0(local_4c);
        local_4._0_1_ = 5;
        FUN_0052c520(local_10c);
        local_4._0_1_ = 3;
        FUN_0052c840(local_ec);
        local_4 = CONCAT31(local_4._1_3_,1);
        FUN_0052cbb0(local_ac);
      }
      local_4 = 0xffffffff;
      FUN_00504d40(local_6c);
    }
  }
  ExceptionList = local_c;
  return;
}

