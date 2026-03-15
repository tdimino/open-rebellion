
uint __thiscall FUN_00537180(void *this,int *param_1)

{
  bool bVar1;
  undefined1 uVar2;
  undefined1 uVar3;
  bool bVar4;
  void *pvVar5;
  int iVar6;
  undefined *puVar7;
  void *this_00;
  uint *puVar8;
  int *piVar9;
  undefined4 *puVar10;
  undefined3 extraout_var;
  uint uVar11;
  uint local_ec;
  int *local_e8;
  uint local_e4;
  uint local_e0;
  void *local_dc;
  uint local_d8;
  uint local_d4;
  undefined4 local_d0;
  undefined4 local_cc;
  uint local_c8;
  int local_c4;
  uint uStack_c0;
  uint uStack_bc;
  uint uStack_b8;
  uint uStack_b4;
  uint uStack_b0;
  uint uStack_ac;
  uint uStack_a8;
  uint uStack_a4;
  undefined4 local_a0;
  undefined4 local_9c;
  uint local_98;
  uint local_94;
  int local_90 [2];
  uint local_88 [2];
  uint local_80;
  undefined4 local_7c;
  undefined4 uStack_78;
  undefined4 uStack_74;
  undefined1 local_70 [4];
  uint uStack_6c;
  undefined4 uStack_68;
  undefined4 uStack_64;
  undefined1 local_60 [4];
  undefined4 uStack_5c;
  undefined4 uStack_58;
  undefined4 uStack_54;
  undefined4 uStack_50;
  undefined4 local_4c [9];
  undefined4 local_28 [7];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00645d0b;
  pvStack_c = ExceptionList;
  local_e4 = 1;
  local_e0 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &pvStack_c;
  local_dc = this;
  FUN_00520580(param_1,&local_e4);
  local_4 = 0xffffffff;
  FUN_00619730();
  FUN_00541e70(local_90);
  local_4 = 1;
  FUN_0051fcb0(this,local_90);
  local_ec = FUN_005202d0(this,param_1);
  local_c8 = 0;
  if ((local_ec != 0) && (param_1[1] == -1)) {
    local_ec = FUN_00553b80(*(int *)((int)this + 0x40),&local_c8);
  }
  local_c4 = 0;
  local_a0 = 1;
  local_9c = 0xffffffff;
  local_4._0_1_ = 2;
  if (local_ec != 0) {
    if (param_1[1] == -1) {
      local_e4 = 1;
      local_e0 = 0x22;
      local_4._0_1_ = 3;
      local_ec = FUN_00504e60(local_88,&local_c4,&local_e4,&local_a0);
      local_4._0_1_ = 2;
      FUN_00619730();
      if (local_ec == 0) goto LAB_00537421;
      if (local_c4 == 0) {
        FUN_00520580(param_1,&local_a0);
      }
    }
    if ((local_ec != 0) && (param_1[1] == -1)) {
      bVar4 = true;
      pvVar5 = (void *)FUN_0041c210(*(int *)((int)this + 0x40));
      while (((pvVar5 != (void *)0x0 && (param_1[1] == -1)) && (bVar4))) {
        FUN_004fa150(pvVar5,&local_d8);
        local_4 = CONCAT31(local_4._1_3_,4);
        iVar6 = FUN_00505750(&local_d8);
        if (iVar6 != 0) {
          puVar7 = FUN_00528820(iVar6);
          this_00 = (void *)thunk_FUN_005f5060((int)puVar7);
          while (((param_1[1] == -1 && (this_00 != (void *)0x0)) && (bVar4))) {
            puVar8 = (uint *)FUN_00403040(this_00,&local_d4);
            local_d0 = 0x14;
            local_cc = 0x1c;
            local_4._0_1_ = 5;
            if ((*puVar8 >> 0x18 < 0x14) || (0x1b < *puVar8 >> 0x18)) {
              bVar1 = false;
            }
            else {
              bVar1 = true;
            }
            FUN_00619730();
            local_4 = CONCAT31(local_4._1_3_,4);
            FUN_00619730();
            if (!bVar1) {
              bVar4 = false;
            }
            this_00 = *(void **)((int)this_00 + 0x10);
          }
        }
        local_4._0_1_ = 2;
        FUN_00619730();
        pvVar5 = *(void **)((int)pvVar5 + 0x10);
      }
      if (((*(uint *)(local_c4 + 0x24) >> 6 & 3) != local_c8) && (!bVar4)) {
        local_e4 = 1;
        local_e0 = 0x28;
        local_4._0_1_ = 6;
        FUN_00520580(param_1,&local_e4);
        local_4._0_1_ = 2;
        FUN_00619730();
      }
    }
  }
LAB_00537421:
  FUN_00551060(local_4c);
  local_4._0_1_ = 7;
  if (local_ec != 0) {
    if (param_1[1] == -1) {
      local_d4 = 3;
      local_ec = FUN_00551190(local_4c,local_90,local_c8,&local_d4);
    }
    if (local_ec != 0) {
      if (param_1[1] == -1) {
        local_ec = FUN_005513a0(local_4c,local_88);
      }
      if ((local_ec != 0) && (param_1[1] == -1)) {
        pvVar5 = (void *)FUN_0041c210(*(int *)((int)local_dc + 0x40));
        while (((local_dc = pvVar5, pvVar5 != (void *)0x0 && (local_ec != 0)) && (param_1[1] == -1))
              ) {
          FUN_004fa150(pvVar5,&local_d4);
          local_4._0_1_ = 8;
          piVar9 = (int *)FUN_00505750(&local_d4);
          if (piVar9[0x16] == 0) {
            local_e8 = (int *)0x0;
            local_80 = 1;
            local_7c = 0x22;
            local_4._0_1_ = 0xd;
            local_ec = FUN_00504e60(local_88,&local_e8,&local_80,param_1);
            local_4._0_1_ = 8;
            FUN_00619730();
            if ((local_ec == 0) || (param_1[1] != -1)) goto LAB_00537a99;
            local_98 = 0xa0;
            local_94 = 0xa2;
            local_4._0_1_ = 0xe;
            uVar11 = (**(code **)(*piVar9 + 4))();
            if ((uVar11 < local_98) || (local_94 <= uVar11)) {
              bVar4 = false;
            }
            else {
              bVar4 = true;
            }
            local_4._0_1_ = 8;
            FUN_00619730();
            if (bVar4) {
              if (local_90[0] == 1) {
                uVar11 = 1;
              }
              else if (local_90[0] == 2) {
                uVar11 = 2;
              }
              else {
                uVar11 = 0;
              }
              if (param_1[1] == -1) {
                local_ec = FUN_00553410((int)local_e8,uVar11,local_c8,1,1,param_1);
              }
              if (local_ec == 0) goto LAB_00537ae4;
              if (param_1[1] == -1) {
                uStack_a8 = 0x90;
                uStack_a4 = 0x98;
                local_4._0_1_ = 0xf;
                uVar11 = (**(code **)(*local_e8 + 4))();
                if ((uVar11 < uStack_a8) || (uStack_a4 <= uVar11)) {
                  bVar4 = false;
                }
                else {
                  bVar4 = true;
                }
                local_4._0_1_ = 8;
                FUN_00619730();
                if (!bVar4) {
                  uStack_5c = 1;
                  uStack_58 = 0x25;
                  local_4._0_1_ = 0x10;
                  FUN_00520580(param_1,&uStack_5c);
                  goto LAB_00537a94;
                }
              }
            }
            else {
              if (param_1[1] == -1) {
                uVar11 = local_e8[0x14];
                if ((uVar11 & 4) == 0) {
                  uStack_68 = 1;
                  uStack_64 = 0x20;
                  local_4._0_1_ = 0x11;
                  FUN_00520580(param_1,&uStack_68);
                  local_4._0_1_ = 8;
                  FUN_00619730();
                }
                else if ((uVar11 & 0x10) == 0) {
                  if ((uVar11 & 8) != 0) {
                    uStack_c0 = 0x90;
                    uStack_bc = 0x98;
                    local_4._0_1_ = 0x13;
                    uVar11 = (**(code **)(*local_e8 + 4))();
                    if ((uVar11 < uStack_c0) || (uStack_bc <= uVar11)) {
                      bVar4 = false;
                    }
                    else {
                      bVar4 = true;
                    }
                    local_4._0_1_ = 8;
                    FUN_00619730();
                    if (!bVar4) {
                      uStack_78 = 1;
                      uStack_74 = 0x22;
                      local_4._0_1_ = 0x14;
                      FUN_00520580(param_1,&uStack_78);
                      local_4._0_1_ = 8;
                      FUN_00619730();
                    }
                  }
                }
                else {
                  uStack_54 = 1;
                  uStack_50 = 0x21;
                  local_4._0_1_ = 0x12;
                  FUN_00520580(param_1,&uStack_54);
                  local_4._0_1_ = 8;
                  FUN_00619730();
                }
              }
              if (param_1[1] != -1) goto LAB_00537a99;
              local_e4 = 0x90;
              local_e0 = 0x98;
              local_4._0_1_ = 0x15;
              uVar11 = (**(code **)(*local_e8 + 4))();
              if ((uVar11 < local_e4) || (local_e0 <= uVar11)) {
                uStack_b8 = 8;
                uStack_b4 = 0x10;
                local_4._0_1_ = 0x16;
                uVar11 = (**(code **)(*local_e8 + 4))();
                if ((uVar11 < uStack_b8) || (uStack_b4 <= uVar11)) {
                  bVar4 = false;
                }
                else {
                  bVar4 = true;
                }
                local_4._0_1_ = 0x15;
                FUN_00619730();
                if (bVar4) goto LAB_00537a48;
                uStack_b0 = 0x14;
                uStack_ac = 0x1c;
                local_4._0_1_ = 0x17;
                uVar11 = (**(code **)(*local_e8 + 4))();
                if ((uVar11 < uStack_b0) || (uStack_ac <= uVar11)) {
                  bVar4 = false;
                }
                else {
                  bVar4 = true;
                }
                local_4._0_1_ = 0x15;
                FUN_00619730();
                if (bVar4) goto LAB_00537a48;
                bVar4 = true;
              }
              else {
LAB_00537a48:
                bVar4 = false;
              }
              local_4._0_1_ = 8;
              FUN_00619730();
              if (bVar4) {
                local_d0 = 1;
                local_cc = 0x25;
                local_4._0_1_ = 0x18;
                FUN_00520580(param_1,&local_d0);
LAB_00537a94:
                local_4._0_1_ = 8;
                FUN_00619730();
              }
            }
LAB_00537a99:
            if ((local_ec != 0) && (param_1[1] == -1)) {
              puVar8 = FUN_004025b0(local_e8,&uStack_6c);
              local_4._0_1_ = 0x19;
              FUN_004f26d0((void *)((int)pvVar5 + 0x48),puVar8);
              local_4._0_1_ = 8;
              FUN_00619730();
            }
          }
          else {
            local_ec = FUN_00552210(local_4c,piVar9,param_1);
            puVar7 = FUN_00528820((int)piVar9);
            for (pvVar5 = (void *)thunk_FUN_005f5060((int)puVar7);
                ((local_ec != 0 && (param_1[1] == -1)) && (pvVar5 != (void *)0x0));
                pvVar5 = *(void **)((int)pvVar5 + 0x10)) {
              FUN_004ece30(&local_d8);
              local_4._0_1_ = 9;
              FUN_0054fa40(local_28);
              local_4._0_1_ = 10;
              uVar2 = (undefined1)local_4;
              local_4._0_1_ = 10;
              uVar3 = (undefined1)local_4;
              if ((local_ec != 0) && (uVar3 = uVar2, param_1[1] == -1)) {
                puVar8 = (uint *)FUN_00403040(pvVar5,local_60);
                local_4._0_1_ = 0xb;
                local_ec = FUN_00551900(local_4c,puVar8,local_88,param_1,local_28,&local_d8);
                local_4._0_1_ = 10;
                FUN_00619730();
                uVar3 = (undefined1)local_4;
              }
              local_4._0_1_ = uVar3;
              if ((local_ec != 0) && (param_1[1] == -1)) {
                puVar10 = (undefined4 *)FUN_00403040(pvVar5,local_70);
                local_4._0_1_ = 0xc;
                bVar4 = FUN_00583f50((void *)((int)local_dc + 0x50),&local_d8,puVar10);
                local_ec = CONCAT31(extraout_var,bVar4);
                local_4._0_1_ = 10;
                FUN_00619730();
              }
              local_4._0_1_ = 9;
              FUN_0054fa90(local_28);
              local_4._0_1_ = 8;
              FUN_00619730();
            }
          }
LAB_00537ae4:
          local_4._0_1_ = 7;
          FUN_00619730();
          pvVar5 = *(void **)((int)local_dc + 0x10);
        }
      }
    }
  }
  if (local_ec == 0) {
    local_d0 = 1;
    local_cc = 1;
    local_4._0_1_ = 0x1a;
    FUN_00520580(param_1,&local_d0);
    local_4._0_1_ = 7;
    FUN_00619730();
  }
  local_4._0_1_ = 2;
  FUN_005510b0(local_4c);
  local_4 = CONCAT31(local_4._1_3_,1);
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_00541ea0();
  ExceptionList = pvStack_c;
  return local_ec;
}

