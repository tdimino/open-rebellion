
void __fastcall FUN_00433e40(void *param_1)

{
  int iVar1;
  bool bVar2;
  uint *puVar3;
  int *piVar4;
  uint uVar5;
  undefined4 uVar6;
  void *pvVar7;
  int iVar8;
  int iVar9;
  int iVar10;
  uint uVar11;
  undefined4 *puVar12;
  int iVar13;
  undefined1 **ppuVar14;
  uint local_18c;
  int *local_188;
  uint local_184;
  undefined1 *local_180;
  int local_17c;
  uint local_178;
  undefined4 local_174;
  uint local_170;
  undefined4 local_16c [7];
  int local_150;
  undefined4 local_14c [7];
  int local_130;
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
  puStack_8 = &LAB_0062d3aa;
  local_c = ExceptionList;
  uVar11 = 0;
  if (*(int *)((int)param_1 + 0x24) == 0) {
    return;
  }
  ExceptionList = &local_c;
  *(undefined4 *)((int)param_1 + 0x24) = 0;
  iVar10 = *(int *)(*(int *)((int)param_1 + 0x3c) + 0x44);
  puVar3 = (uint *)FUN_00403040(param_1,&local_17c);
  local_4 = 0;
  piVar4 = FUN_004f3220(*(int *)((int)param_1 + 0x1c),puVar3);
  local_4 = 0xffffffff;
  local_188 = piVar4;
  FUN_00619730();
  puVar12 = (undefined4 *)((int)param_1 + 0x48);
  for (iVar9 = 0x3c; iVar9 != 0; iVar9 = iVar9 + -1) {
    *puVar12 = 0;
    puVar12 = puVar12 + 1;
  }
  puVar3 = (uint *)((int)param_1 + 0x28);
  *puVar3 = *(uint *)((int)param_1 + 0x28) & 0x3e00000;
  *(uint *)((int)param_1 + 0x30) = *(uint *)((int)param_1 + 0x30) & 0x35ff0000;
  *(uint *)((int)param_1 + 0x2c) = *(uint *)((int)param_1 + 0x2c) & 0x60000000;
  iVar9 = *(int *)((int)param_1 + 0x1c);
  if (iVar9 == 1) {
    uVar11 = 1;
  }
  else if (iVar9 == 2) {
    uVar11 = 2;
  }
  if (iVar9 != 1) {
    local_184 = 1;
  }
  else if (iVar9 != 1) {
    local_184 = 0;
  }
  else {
    local_184 = 2;
  }
  *(uint *)((int)param_1 + 0x20) = (uint)piVar4[9] >> 6 & 3;
  if ((*(byte *)(piVar4 + 0x14) & 8) != 0) {
    *(uint *)((int)param_1 + 0x30) = *(uint *)((int)param_1 + 0x30) | 0x10;
    FUN_004fcdd0(puVar3);
  }
  local_178 = 0x92;
  uVar5 = *(uint *)((int)param_1 + 0x18) >> 0x18;
  local_174 = 0x94;
  if ((uVar5 < 0x92) || (0x93 < uVar5)) {
    bVar2 = false;
  }
  else {
    bVar2 = true;
  }
  FUN_00619730();
  if (bVar2) {
    *(uint *)((int)param_1 + 0x30) = *(uint *)((int)param_1 + 0x30) | 0x40;
  }
  if (*(uint *)((int)param_1 + 0x20) == uVar11) {
    uVar11 = *(uint *)((int)param_1 + 0x30);
    *(uint *)((int)param_1 + 0x30) = uVar11 | 1;
    uVar11 = uVar11 | 0x21;
  }
  else if (*(uint *)((int)param_1 + 0x20) == local_184) {
    uVar11 = *(uint *)((int)param_1 + 0x30);
    *(uint *)((int)param_1 + 0x30) = uVar11 | 4;
    uVar11 = uVar11 | 0x24;
  }
  else {
    uVar11 = *(uint *)((int)param_1 + 0x30) | 2;
  }
  *(uint *)((int)param_1 + 0x30) = uVar11;
  if ((*(byte *)(local_188 + 0x22) & 2) == 0) {
    *(uint *)((int)param_1 + 0x2c) = *(uint *)((int)param_1 + 0x2c) | 0x80;
    ExceptionList = local_c;
    return;
  }
  *(uint *)((int)param_1 + 0x2c) = *(uint *)((int)param_1 + 0x2c) | 0x8000000;
  *(int *)((int)param_1 + 0x4c) = local_188[0x17];
  *(int *)((int)param_1 + 0x84) = local_188[0x17] - local_188[0x18];
  *(int *)((int)param_1 + 0x80) = local_188[0x19] - local_188[0x1a];
  if (((*(byte *)(local_188 + 0x22) & 0x20) != 0) &&
     (*puVar3 = *puVar3 | 2, (*(byte *)((int)param_1 + 0x30) & 6) != 0)) {
    uVar6 = thunk_FUN_004fcf00();
    *(undefined4 *)((int)param_1 + 0x44) = uVar6;
  }
  piVar4 = local_188;
  if ((*(uint *)((int)param_1 + 0x18) & 0xffffff) == 0x109) {
    *puVar3 = *puVar3 | 0x80000000;
    *(uint *)((int)param_1 + 0x30) = *(uint *)((int)param_1 + 0x30) | 0x40000000;
  }
  if ((*(uint *)((int)param_1 + 0x18) & 0xffffff) == 0x121) {
    *(uint *)((int)param_1 + 0x30) = *(uint *)((int)param_1 + 0x30) | 0x8000000;
  }
  iVar9 = FUN_00526a60(local_188,*(undefined4 *)((int)param_1 + 0x20),3);
  if (iVar9 != 0) {
    *puVar3 = *puVar3 | 0x80000000;
    *(uint *)((int)param_1 + 0x30) = *(uint *)((int)param_1 + 0x30) | 0x80000000;
    *(uint *)((int)param_1 + 0x2c) = *(uint *)((int)param_1 + 0x2c) | 0x2000000;
  }
  if ((*(byte *)((int)param_1 + 0x30) & 1) != 0) {
    *(undefined4 *)((int)param_1 + 0x48) = 100;
  }
  FUN_00504c40(local_2c,piVar4,3);
  local_4 = 1;
  FUN_00513120((int)local_2c);
  piVar4 = local_188;
  while (local_188 = piVar4, local_10 != 0) {
    puVar3 = &local_18c;
    pvVar7 = (void *)FUN_0052bed0((int)local_2c);
    puVar3 = (uint *)FUN_00402d80(pvVar7,puVar3);
    local_4._0_1_ = 2;
    iVar9 = FUN_0042e190(*(void **)((int)param_1 + 0x40),puVar3);
    local_4 = CONCAT31(local_4._1_3_,1);
    FUN_00619730();
    if ((*(byte *)((int)param_1 + 0x30) & 1) != 0) {
      if ((*(uint *)((int)param_1 + 0x28) & 0x83e00000) == 0) {
        if ((*(uint *)(iVar9 + 0x18) & 0x1000) != 0) {
          *(uint *)((int)param_1 + 0x30) = *(uint *)((int)param_1 + 0x30) | 0x4000;
          uVar11 = *(uint *)(iVar9 + 0x18);
          goto joined_r0x0043419f;
        }
        *(int *)((int)param_1 + 0x8c) = *(int *)((int)param_1 + 0x8c) + 1;
        *(uint *)((int)param_1 + 0x28) = *(uint *)((int)param_1 + 0x28) | 0x20000000;
      }
      else if (*(int *)(iVar9 + 0x34) < *(int *)(*(int *)((int)param_1 + 0x40) + 0x24)) {
        if ((*(uint *)(iVar9 + 0x18) & 0x1000) != 0) {
          *(uint *)((int)param_1 + 0x30) = *(uint *)((int)param_1 + 0x30) | 0x4000;
        }
        uVar11 = *(uint *)(iVar9 + 0x18);
joined_r0x0043419f:
        if ((uVar11 & 0x2000) != 0) {
          *(uint *)((int)param_1 + 0x30) = *(uint *)((int)param_1 + 0x30) | 0x8000;
        }
      }
      else {
        *(int *)((int)param_1 + 0x8c) = *(int *)((int)param_1 + 0x8c) + 1;
        if ((*(uint *)(iVar9 + 0x18) & 0x1000) != 0) {
          *(uint *)((int)param_1 + 0x28) = *(uint *)((int)param_1 + 0x28) | 0x10000000;
        }
        if ((*(uint *)(iVar9 + 0x18) & 0x2000) != 0) {
          *(uint *)((int)param_1 + 0x28) = *(uint *)((int)param_1 + 0x28) | 0x20000000;
        }
      }
    }
    *(int *)((int)param_1 + 0x88) = *(int *)((int)param_1 + 0x88) + 1;
    *(int *)((int)param_1 + 0xc0) = *(int *)((int)param_1 + 0xc0) + *(int *)(iVar9 + 0x20);
    iVar9 = FUN_0052bed0((int)local_2c);
    local_180 = (undefined1 *)(*(uint *)(iVar9 + 0x50) & 1);
    if (local_180 != (undefined1 *)0x0) {
      *(int *)((int)param_1 + 0x90) = *(int *)((int)param_1 + 0x90) + 1;
    }
    FUN_005130d0((int)local_2c);
    piVar4 = local_188;
  }
  FUN_004ffe70(local_6c,piVar4,3);
  local_4._0_1_ = 3;
  FUN_00513120((int)local_6c);
  if (local_50 != 0) {
    FUN_005f4950(&local_184,0);
    local_4._0_1_ = 4;
    if (local_50 != 0) {
      do {
        puVar3 = &local_18c;
        pvVar7 = (void *)FUN_0052bed0((int)local_6c);
        puVar3 = FUN_004025b0(pvVar7,puVar3);
        local_4._0_1_ = 5;
        local_180 = &stack0xfffffe5c;
        FUN_004f26d0(&stack0xfffffe5c,puVar3);
        pvVar7 = FUN_00419e40((void *)(iVar10 + 0x78));
        local_4._0_1_ = 4;
        FUN_00619730();
        if (pvVar7 != (void *)0x0) {
          local_184 = *(uint *)((int)pvVar7 + 0x38);
          if (((local_184 & 1) == 0) && ((*(uint *)((int)pvVar7 + 0x38) & 0x4000) != 0)) {
            *(int *)((int)param_1 + 0xe4) =
                 *(int *)((int)param_1 + 0xe4) + *(int *)((int)pvVar7 + 0xb4);
            *(int *)((int)param_1 + 0xe8) =
                 *(int *)((int)param_1 + 0xe8) + *(int *)((int)pvVar7 + 0xb8);
            *(int *)((int)param_1 + 0xec) =
                 *(int *)((int)param_1 + 0xec) + *(int *)((int)pvVar7 + 0x60);
            *(int *)((int)param_1 + 0xf4) =
                 *(int *)((int)param_1 + 0xf4) + *(int *)((int)pvVar7 + 0x70);
            *(int *)((int)param_1 + 0xbc) =
                 *(int *)((int)param_1 + 0xbc) + *(int *)((int)pvVar7 + 0xa4);
            *(uint *)((int)param_1 + 0x30) = *(uint *)((int)param_1 + 0x30) | 0x200;
            *(uint *)((int)param_1 + 0x2c) = *(uint *)((int)param_1 + 0x2c) | 0x800000;
            if (((local_184 & 0x10) != 0) &&
               (*(uint *)((int)param_1 + 0x30) = *(uint *)((int)param_1 + 0x30) | 0x400,
               *(int *)((int)param_1 + 0x1c) == 1)) {
              *(uint *)((int)param_1 + 0x2c) = *(uint *)((int)param_1 + 0x2c) | 0x1000000;
            }
          }
          if ((local_184 & 0x200000) != 0) {
            uVar11 = *(uint *)((int)param_1 + 0x2c);
            *(uint *)((int)param_1 + 0x2c) = uVar11 | 0x800;
            *(uint *)((int)param_1 + 0x2c) = uVar11 | 0x2800;
          }
          if ((local_184 & 0x400000) != 0) {
            *(uint *)((int)param_1 + 0x2c) = *(uint *)((int)param_1 + 0x2c) | 0x800;
          }
          if ((local_184 & 0x2000000) != 0) {
            *(uint *)((int)param_1 + 0x30) = *(uint *)((int)param_1 + 0x30) | 0x1000;
          }
          if ((local_184 & 0x1000000) != 0) {
            *(uint *)((int)param_1 + 0x2c) = *(uint *)((int)param_1 + 0x2c) | 0x8000;
            *(uint *)((int)param_1 + 0x2c) = *(uint *)((int)param_1 + 0x2c) | 0x8000;
          }
        }
        FUN_005130d0((int)local_6c);
        piVar4 = local_188;
      } while (local_50 != 0);
    }
    local_4._0_1_ = 3;
    FUN_00619730();
  }
  FUN_00536da0(local_ac,piVar4,3);
  local_4._0_1_ = 6;
  FUN_00513120((int)local_ac);
  if (local_90 != 0) {
    FUN_005f4950(&local_184,0);
    local_4._0_1_ = 7;
    if (local_90 != 0) {
      do {
        puVar3 = &local_18c;
        pvVar7 = (void *)FUN_0052bed0((int)local_ac);
        puVar3 = FUN_004025b0(pvVar7,puVar3);
        local_4._0_1_ = 8;
        pvVar7 = FUN_004f5940((void *)(iVar10 + 0x8c),puVar3);
        local_4._0_1_ = 7;
        FUN_00619730();
        if (pvVar7 != (void *)0x0) {
          local_184 = *(uint *)((int)pvVar7 + 0x30);
          if ((local_184 & 4) == 0) {
            if ((local_184 & 0x20000000) != 0) {
              if ((local_184 & 0x40) != 0) {
                uVar11 = *(uint *)((int)param_1 + 0x2c) | 0x4000;
                goto LAB_0043452b;
              }
              if ((local_184 & 8) == 0) {
                *(int *)((int)param_1 + 0xfc) = *(int *)((int)param_1 + 0xfc) + 1;
              }
              else {
                *(int *)((int)param_1 + 0xf8) = *(int *)((int)param_1 + 0xf8) + 1;
              }
            }
          }
          else if ((local_184 & 0x20000000) != 0) {
            if ((local_184 & 0x40) == 0) {
              if ((local_184 & 0x20) == 0) {
                *(uint *)((int)param_1 + 0x2c) = *(uint *)((int)param_1 + 0x2c) | 0x400;
              }
              if ((local_184 & 0x7000000) != 0) {
                uVar11 = *(uint *)((int)param_1 + 0x30);
                *(uint *)((int)param_1 + 0x30) = uVar11 | 0x1000;
                if ((local_184 & 0x2000000) != 0) {
                  *(uint *)((int)param_1 + 0x30) = uVar11 | 0x3000;
                }
              }
              if ((local_184 & 8) == 0) {
                *(int *)((int)param_1 + 0x108) = *(int *)((int)param_1 + 0x108) + 1;
              }
              else {
                *(uint *)((int)param_1 + 0x2c) = *(uint *)((int)param_1 + 0x2c) | 0x1000;
                *(int *)((int)param_1 + 0x104) = *(int *)((int)param_1 + 0x104) + 1;
                if ((local_184 & 0x40000000) != 0) {
                  *(int *)((int)param_1 + 0x110) = *(int *)((int)param_1 + 0x110) + 1;
                }
              }
            }
            else {
              uVar11 = *(uint *)((int)param_1 + 0x2c) | 0x4000000;
LAB_0043452b:
              *(uint *)((int)param_1 + 0x2c) = uVar11;
              *(int *)((int)param_1 + 0x100) = *(int *)((int)param_1 + 0x100) + 1;
            }
          }
        }
        FUN_005130d0((int)local_ac);
      } while (local_90 != 0);
    }
    local_4._0_1_ = 6;
    FUN_00619730();
    piVar4 = local_188;
  }
  if (((*(byte *)((int)param_1 + 0x30) & 1) != 0) && (0 < *(int *)((int)param_1 + 0x84))) {
    uVar11 = *(uint *)((int)param_1 + 0x28);
    *(uint *)((int)param_1 + 0x28) = uVar11 | 0x20;
    if (0 < *(int *)((int)param_1 + 0x80)) {
      *(uint *)((int)param_1 + 0x28) = uVar11 | 0x60;
    }
  }
  iVar9 = FUN_0052d370(piVar4,*(undefined4 *)((int)param_1 + 0x20),3);
  *(int *)((int)param_1 + 0x50) = iVar9;
  iVar9 = FUN_0052cf40(piVar4,*(undefined4 *)((int)param_1 + 0x20),3);
  uVar11 = 0;
  *(int *)((int)param_1 + 0x54) = iVar9;
  local_184 = 0;
  FUN_0052cab0(local_ec,piVar4,3);
  local_4 = CONCAT31(local_4._1_3_,9);
  FUN_00513120((int)local_ec);
  uVar5 = local_184;
  while (local_d0 != 0) {
    puVar3 = &local_18c;
    pvVar7 = (void *)FUN_0052bed0((int)local_ec);
    puVar3 = (uint *)FUN_00402d80(pvVar7,puVar3);
    local_4._0_1_ = 10;
    iVar9 = FUN_0042e190(*(void **)((int)param_1 + 0x40),puVar3);
    local_4 = CONCAT31(local_4._1_3_,9);
    FUN_00619730();
    iVar8 = FUN_0052bed0((int)local_ec);
    uVar11 = uVar11 | *(uint *)(iVar8 + 0x50) & 1;
    *(int *)((int)param_1 + 0x58) = *(int *)((int)param_1 + 0x58) + 1;
    *(int *)((int)param_1 + 0xcc) = *(int *)((int)param_1 + 0xcc) + *(int *)(iVar9 + 0x20);
    *(int *)((int)param_1 + 0xd0) = *(int *)((int)param_1 + 0xd0) + *(int *)(iVar9 + 0x20);
    FUN_005130d0((int)local_ec);
    uVar5 = uVar11;
  }
  local_184 = uVar5;
  if ((((uVar11 != 0) &&
       (*(uint *)((int)param_1 + 0x28) = *(uint *)((int)param_1 + 0x28) | 0x1000,
       0 < *(int *)((int)param_1 + 0x58))) && ((*(byte *)((int)param_1 + 0x28) & 3) == 0)) &&
     ((iVar9 = FUN_00509670(local_188,0), iVar9 != 0 && (*(int *)(iVar9 + 0x58) == 0)))) {
    *(uint *)((int)param_1 + 0x28) = *(uint *)((int)param_1 + 0x28) | 0x200;
  }
  uVar11 = 0;
  local_170 = 0;
  FUN_0052c740(local_12c,local_188,3);
  local_4 = CONCAT31(local_4._1_3_,0xb);
  FUN_00513120((int)local_12c);
  uVar5 = local_170;
  while (local_110 != 0) {
    puVar3 = &local_18c;
    pvVar7 = (void *)FUN_0052bed0((int)local_12c);
    puVar3 = (uint *)FUN_00402d80(pvVar7,puVar3);
    local_4._0_1_ = 0xc;
    iVar9 = FUN_0042e190(*(void **)((int)param_1 + 0x40),puVar3);
    local_4 = CONCAT31(local_4._1_3_,0xb);
    FUN_00619730();
    iVar8 = FUN_0052bed0((int)local_12c);
    uVar11 = uVar11 | *(uint *)(iVar8 + 0x50) & 1;
    *(int *)((int)param_1 + 0x5c) = *(int *)((int)param_1 + 0x5c) + 1;
    *(int *)((int)param_1 + 0xcc) = *(int *)((int)param_1 + 0xcc) + *(int *)(iVar9 + 0x20);
    *(int *)((int)param_1 + 0xd4) = *(int *)((int)param_1 + 0xd4) + *(int *)(iVar9 + 0x20);
    FUN_005130d0((int)local_12c);
    uVar5 = uVar11;
  }
  local_170 = uVar5;
  if (((uVar11 != 0) &&
      (*(uint *)((int)param_1 + 0x28) = *(uint *)((int)param_1 + 0x28) | 0x1000,
      0 < *(int *)((int)param_1 + 0x5c))) &&
     (((*(byte *)((int)param_1 + 0x28) & 3) == 0 &&
      ((iVar9 = FUN_00509670(local_188,2), iVar9 != 0 && (*(int *)(iVar9 + 0x58) == 0)))))) {
    *(uint *)((int)param_1 + 0x28) = *(uint *)((int)param_1 + 0x28) | 0x400;
  }
  piVar4 = local_188;
  uVar11 = 0;
  local_18c = 0;
  FUN_0052c420(local_4c,local_188,3);
  local_4 = CONCAT31(local_4._1_3_,0xd);
  FUN_00513120((int)local_4c);
  uVar5 = local_18c;
  while (local_30 != 0) {
    puVar3 = &local_18c;
    pvVar7 = (void *)FUN_0052bed0((int)local_4c);
    puVar3 = (uint *)FUN_00402d80(pvVar7,puVar3);
    local_4._0_1_ = 0xe;
    iVar9 = FUN_0042e190(*(void **)((int)param_1 + 0x40),puVar3);
    local_4 = CONCAT31(local_4._1_3_,0xd);
    FUN_00619730();
    iVar8 = FUN_0052bed0((int)local_4c);
    uVar11 = uVar11 | *(uint *)(iVar8 + 0x50) & 1;
    *(int *)((int)param_1 + 0x60) = *(int *)((int)param_1 + 0x60) + 1;
    *(int *)((int)param_1 + 0xcc) = *(int *)((int)param_1 + 0xcc) + *(int *)(iVar9 + 0x20);
    *(int *)((int)param_1 + 0xd8) = *(int *)((int)param_1 + 0xd8) + *(int *)(iVar9 + 0x20);
    FUN_005130d0((int)local_4c);
    uVar5 = uVar11;
    piVar4 = local_188;
  }
  local_18c = uVar5;
  if ((((uVar11 != 0) &&
       (*(uint *)((int)param_1 + 0x28) = *(uint *)((int)param_1 + 0x28) | 0x1000,
       0 < *(int *)((int)param_1 + 0x60))) && ((*(byte *)((int)param_1 + 0x28) & 3) == 0)) &&
     ((iVar9 = FUN_00509670(piVar4,1), iVar9 != 0 && (*(int *)(iVar9 + 0x58) == 0)))) {
    *(uint *)((int)param_1 + 0x28) = *(uint *)((int)param_1 + 0x28) | 0x800;
  }
  FUN_005278c0(local_cc,piVar4,3);
  local_4 = CONCAT31(local_4._1_3_,0xf);
  FUN_00513120((int)local_cc);
  while (local_b0 != 0) {
    ppuVar14 = &local_180;
    pvVar7 = (void *)FUN_0052bed0((int)local_cc);
    puVar3 = (uint *)FUN_00402d80(pvVar7,ppuVar14);
    local_4._0_1_ = 0x10;
    iVar9 = FUN_0042e190(*(void **)((int)param_1 + 0x40),puVar3);
    local_4 = CONCAT31(local_4._1_3_,0xf);
    FUN_00619730();
    *(int *)((int)param_1 + 100) = *(int *)((int)param_1 + 100) + 1;
    *(int *)((int)param_1 + 0xc4) = *(int *)((int)param_1 + 0xc4) + *(int *)(iVar9 + 0x20);
    iVar8 = FUN_0052bed0((int)local_cc);
    local_178 = *(uint *)(iVar8 + 0x50) & 1;
    if (local_178 != 0) {
      *(int *)((int)param_1 + 0x68) = *(int *)((int)param_1 + 0x68) + *(int *)(iVar9 + 0x34);
    }
    FUN_005130d0((int)local_cc);
  }
  if (1 < *(int *)((int)param_1 + 100)) {
    *(uint *)((int)param_1 + 0x30) = *(uint *)((int)param_1 + 0x30) | 0x100;
  }
  FUN_0051c270(local_8c,local_188,3);
  local_4 = CONCAT31(local_4._1_3_,0x11);
  FUN_00513120((int)local_8c);
  while (local_70 != 0) {
    ppuVar14 = &local_180;
    pvVar7 = (void *)FUN_0052bed0((int)local_8c);
    puVar3 = (uint *)FUN_00402d80(pvVar7,ppuVar14);
    local_4._0_1_ = 0x12;
    iVar9 = FUN_0042e190(*(void **)((int)param_1 + 0x40),puVar3);
    local_4 = CONCAT31(local_4._1_3_,0x11);
    FUN_00619730();
    *(int *)((int)param_1 + 0x70) = *(int *)((int)param_1 + 0x70) + 1;
    *(int *)((int)param_1 + 0xc4) = *(int *)((int)param_1 + 0xc4) + *(int *)(iVar9 + 0x20);
    iVar9 = FUN_0052bed0((int)local_8c);
    local_178 = *(uint *)(iVar9 + 0x50) & 1;
    if (local_178 != 0) {
      *(uint *)((int)param_1 + 0x30) = *(uint *)((int)param_1 + 0x30) | 0x80;
    }
    FUN_005130d0((int)local_8c);
  }
  FUN_005272e0(local_10c,local_188,3);
  local_4 = CONCAT31(local_4._1_3_,0x13);
  FUN_00513120((int)local_10c);
  while (local_f0 != 0) {
    ppuVar14 = &local_180;
    pvVar7 = (void *)FUN_0052bed0((int)local_10c);
    puVar3 = (uint *)FUN_00402d80(pvVar7,ppuVar14);
    local_4._0_1_ = 0x14;
    iVar9 = FUN_0042e190(*(void **)((int)param_1 + 0x40),puVar3);
    local_4 = CONCAT31(local_4._1_3_,0x13);
    FUN_00619730();
    *(int *)((int)param_1 + 0x74) = *(int *)((int)param_1 + 0x74) + 1;
    *(int *)((int)param_1 + 0xc4) = *(int *)((int)param_1 + 0xc4) + *(int *)(iVar9 + 0x20);
    iVar8 = FUN_0052bed0((int)local_10c);
    local_178 = *(uint *)(iVar8 + 0x50) & 1;
    if (local_178 != 0) {
      *(int *)((int)param_1 + 0x78) = *(int *)((int)param_1 + 0x78) + *(int *)(iVar9 + 0x30);
    }
    FUN_005130d0((int)local_10c);
  }
  FUN_00527550(local_14c,local_188,3);
  local_4 = CONCAT31(local_4._1_3_,0x15);
  FUN_00513120((int)local_14c);
  piVar4 = local_188;
  while (local_188 = piVar4, local_130 != 0) {
    ppuVar14 = &local_180;
    pvVar7 = (void *)FUN_0052bed0((int)local_14c);
    puVar3 = (uint *)FUN_00402d80(pvVar7,ppuVar14);
    local_4._0_1_ = 0x16;
    iVar9 = FUN_0042e190(*(void **)((int)param_1 + 0x40),puVar3);
    local_4 = CONCAT31(local_4._1_3_,0x15);
    FUN_00619730();
    *(int *)((int)param_1 + 0x74) = *(int *)((int)param_1 + 0x74) + 1;
    *(int *)((int)param_1 + 0xc4) = *(int *)((int)param_1 + 0xc4) + *(int *)(iVar9 + 0x20);
    iVar8 = FUN_0052bed0((int)local_14c);
    local_178 = *(uint *)(iVar8 + 0x50) & 1;
    if (local_178 != 0) {
      *(int *)((int)param_1 + 0x78) = *(int *)((int)param_1 + 0x78) + *(int *)(iVar9 + 0x30);
    }
    FUN_005130d0((int)local_14c);
    piVar4 = local_188;
  }
  if ((piVar4[0x22] & 1U) == 0) {
    if ((*(byte *)((int)param_1 + 0x28) & 0x20) != 0) {
      uVar11 = *(uint *)((int)param_1 + 0x28);
      *(uint *)((int)param_1 + 0x28) = uVar11 | 0x100;
      if ((*(byte *)((int)param_1 + 0x28) & 0x40) != 0) {
        *(uint *)((int)param_1 + 0x28) = uVar11 | 0x180;
      }
    }
  }
  else {
    if ((piVar4[0x22] & 4U) != 0) {
      *(uint *)((int)param_1 + 0x28) = *(uint *)((int)param_1 + 0x28) | 1;
    }
    uVar11 = *(uint *)((int)param_1 + 0x30);
    *(uint *)((int)param_1 + 0x30) = uVar11 | 8;
    *(uint *)((int)param_1 + 0x30) = uVar11 | 0x28;
    iVar9 = FUN_00507270(piVar4,*(int *)((int)param_1 + 0x1c));
    *(int *)((int)param_1 + 0x48) = iVar9;
    if (iVar9 < 0x46) {
      *(uint *)((int)param_1 + 0x2c) = *(uint *)((int)param_1 + 0x2c) | 0x10000000;
    }
    FUN_005039d0(local_16c,piVar4,3);
    local_4 = CONCAT31(local_4._1_3_,0x17);
    FUN_00513120((int)local_16c);
    while (local_150 != 0) {
      ppuVar14 = &local_180;
      pvVar7 = (void *)FUN_0052bed0((int)local_16c);
      puVar3 = (uint *)FUN_00402d80(pvVar7,ppuVar14);
      local_4._0_1_ = 0x18;
      pvVar7 = (void *)FUN_0042e190(*(void **)((int)param_1 + 0x40),puVar3);
      local_4 = CONCAT31(local_4._1_3_,0x17);
      FUN_00619730();
      *(int *)((int)param_1 + 0xa4) = *(int *)((int)param_1 + 0xa4) + 1;
      *(int *)((int)param_1 + 200) = *(int *)((int)param_1 + 200) + *(int *)((int)pvVar7 + 0x20);
      if (((*(byte *)((int)param_1 + 0x30) & 1) != 0) &&
         (((*(uint *)((int)param_1 + 0x28) & 0x83e00000) == 0 ||
          (*(int *)(*(int *)((int)param_1 + 0x40) + 0x28) <= *(int *)((int)pvVar7 + 0x34))))) {
        *(int *)((int)param_1 + 0xa8) = *(int *)((int)param_1 + 0xa8) + 1;
      }
      iVar9 = FUN_0052bed0((int)local_16c);
      local_178 = *(uint *)(iVar9 + 0x50) & 1;
      if (local_178 != 0) {
        piVar4 = (int *)FUN_0052bed0((int)local_16c);
        local_17c = (**(code **)(*piVar4 + 0x254))();
        iVar9 = (**(code **)(*piVar4 + 0x244))();
        iVar9 = FUN_00473610(pvVar7,iVar9 - local_17c);
        *(int *)((int)param_1 + 0xb0) = *(int *)((int)param_1 + 0xb0) + iVar9;
        piVar4 = (int *)FUN_0052bed0((int)local_16c);
        local_17c = (**(code **)(*piVar4 + 0x254))();
        iVar9 = (**(code **)(*piVar4 + 0x244))();
        iVar9 = FUN_00473620(pvVar7,iVar9 - local_17c);
        *(int *)((int)param_1 + 0xb4) = *(int *)((int)param_1 + 0xb4) + iVar9;
        piVar4 = (int *)FUN_0052bed0((int)local_16c);
        local_17c = (**(code **)(*piVar4 + 0x254))();
        iVar9 = (**(code **)(*piVar4 + 0x244))();
        iVar9 = FUN_00473630(pvVar7,iVar9 - local_17c);
        *(int *)((int)param_1 + 0xb8) = *(int *)((int)param_1 + 0xb8) + iVar9;
        *(int *)((int)param_1 + 0xac) = *(int *)((int)param_1 + 0xac) + 1;
        if ((*(byte *)((int)param_1 + 0x30) & 1) != 0) {
          *(uint *)((int)param_1 + 0x28) = *(uint *)((int)param_1 + 0x28) | 0x80000;
        }
      }
      FUN_005130d0((int)local_16c);
    }
    if ((((*(byte *)((int)param_1 + 0x30) & 1) != 0) &&
        (uVar11 = *(uint *)((int)param_1 + 0x28), (uVar11 & 3) == 0)) && ((uVar11 & 0x3e00000) == 0)
       ) {
      *(uint *)((int)param_1 + 0x28) = uVar11 | 0x4000000;
      iVar9 = *(int *)((int)param_1 + 0x74) + *(int *)((int)param_1 + 0x70) +
              *(int *)((int)param_1 + 0x54) + *(int *)((int)param_1 + 0x50) +
              *(int *)((int)param_1 + 100);
      if ((*(uint *)((int)param_1 + 0x30) & 0x80000000) != 0) {
        iVar9 = iVar9 + 1;
      }
      if (iVar9 < *(int *)((int)param_1 + 0x4c)) {
        uVar11 = *(uint *)((int)param_1 + 0x28);
        *(uint *)((int)param_1 + 0x28) = uVar11 | 0x100;
        if (0 < *(int *)((int)param_1 + 0x80)) {
          *(uint *)((int)param_1 + 0x28) = uVar11 | 0x180;
        }
      }
    }
    uVar11 = *(uint *)((int)param_1 + 0x30);
    if ((uVar11 & 1) == 0) {
      if ((uVar11 & 4) == 0) {
        if ((uVar11 & 2) != 0) {
          if ((0 < -*(int *)((int)param_1 + 0x48) + 0x5a) &&
             (iVar10 = *(int *)((int)param_1 + 0x98) + (-*(int *)((int)param_1 + 0x48) + 99) / 10,
             *(int *)((int)param_1 + 0x98) = iVar10, 6 < iVar10)) {
            *(undefined4 *)((int)param_1 + 0x98) = 6;
          }
          *(int *)((int)param_1 + 0x118) =
               *(int *)((int)param_1 + 0x74) + *(int *)((int)param_1 + 0x98);
          if ((uVar11 & 0x200) == 0) {
            *(uint *)((int)param_1 + 0x2c) = *(uint *)((int)param_1 + 0x2c) | 0x40;
          }
          if ((*(uint *)((int)param_1 + 0x28) & 0x80000000) != 0) {
            *(int *)((int)param_1 + 0x110) = *(int *)((int)param_1 + 0x110) + 1;
          }
        }
      }
      else {
        iVar10 = -*(int *)((int)param_1 + 0x48);
        if ((0 < iVar10 + 0x50) &&
           (iVar10 = *(int *)((int)param_1 + 0x98) + (iVar10 + 0x59) / 10,
           *(int *)((int)param_1 + 0x98) = iVar10, 6 < iVar10)) {
          *(undefined4 *)((int)param_1 + 0x98) = 6;
        }
        *(int *)((int)param_1 + 0x118) =
             *(int *)((int)param_1 + 0x88) + *(int *)((int)param_1 + 0x98) +
             *(int *)((int)param_1 + 0x74);
        if (0x32 < *(int *)((int)param_1 + 0x48)) {
          *(uint *)((int)param_1 + 0x2c) = *(uint *)((int)param_1 + 0x2c) | 0x100;
        }
        *(int *)((int)param_1 + 0xbc) =
             *(int *)((int)param_1 + 0xbc) +
             *(int *)((int)param_1 + 0x90) + *(int *)((int)param_1 + 0xac);
        if ((*(uint *)((int)param_1 + 0x28) & 0x80000000) != 0) {
          *(int *)((int)param_1 + 0x110) = *(int *)((int)param_1 + 0x110) + 1;
        }
        if (0 < *(int *)((int)param_1 + 0x68)) {
          *(uint *)((int)param_1 + 0x2c) = *(uint *)((int)param_1 + 0x2c) | 0x40000;
        }
        if ((*(byte *)((int)param_1 + 0x30) & 0x80) != 0) {
          *(uint *)((int)param_1 + 0x2c) = *(uint *)((int)param_1 + 0x2c) | 0x80000;
        }
        if (0 < *(int *)((int)param_1 + 0x78)) {
          *(uint *)((int)param_1 + 0x2c) = *(uint *)((int)param_1 + 0x2c) | 0x100000;
        }
        if (0 < *(int *)((int)param_1 + 0x90)) {
          *(uint *)((int)param_1 + 0x2c) = *(uint *)((int)param_1 + 0x2c) | 0x200000;
        }
        if (0 < *(int *)((int)param_1 + 0xac)) {
          *(uint *)((int)param_1 + 0x2c) = *(uint *)((int)param_1 + 0x2c) | 0x400000;
        }
        if (((((((0 < *(int *)((int)param_1 + 0x50)) || (0 < *(int *)((int)param_1 + 0x54))) ||
               (0 < *(int *)((int)param_1 + 0x58))) ||
              ((0 < *(int *)((int)param_1 + 0x5c) || (0 < *(int *)((int)param_1 + 0x60))))) ||
             (0 < *(int *)((int)param_1 + 0x70))) || (0 < *(int *)((int)param_1 + 100))) &&
           (*(int *)((int)param_1 + 0x74) == 0)) {
          *(uint *)((int)param_1 + 0x30) = *(uint *)((int)param_1 + 0x30) | 0x800;
        }
        if (((0 < *(int *)((int)param_1 + 0x58)) || (0 < *(int *)((int)param_1 + 0x5c))) ||
           (0 < *(int *)((int)param_1 + 0x60))) {
          *(uint *)((int)param_1 + 0x2c) = *(uint *)((int)param_1 + 0x2c) | 0x20000;
        }
        if ((0 < *(int *)((int)param_1 + 0x50)) || (0 < *(int *)((int)param_1 + 0x54))) {
          *(uint *)((int)param_1 + 0x2c) = *(uint *)((int)param_1 + 0x2c) | 0x10000;
        }
      }
    }
    else {
      iVar9 = 0;
      iVar13 = 0;
      iVar8 = 0;
      if ((0 < -*(int *)((int)param_1 + 0x48) + 0x46) &&
         (*(int *)((int)param_1 + 0x98) =
               *(int *)((int)param_1 + 0x98) + (-*(int *)((int)param_1 + 0x48) + 0x4f) / 10,
         6 < *(int *)((int)param_1 + 0x98))) {
        *(undefined4 *)((int)param_1 + 0x98) = 6;
      }
      if ((uVar11 & 0x40000000) == 0) {
        if ((*(uint *)((int)param_1 + 0x28) & 0x3e00000) == 0) {
          if (0 < *(int *)((int)param_1 + 0x98)) {
            iVar8 = *(int *)(iVar10 + 0x378);
            iVar1 = *(int *)(iVar10 + 0x374);
            if (iVar8 + *(int *)(iVar10 + 0x374) < *(int *)((int)param_1 + 0x4c)) goto LAB_00434f68;
          }
        }
        else {
          iVar8 = *(int *)(iVar10 + 0x370);
          iVar1 = *(int *)(iVar10 + 0x36c);
          if (iVar8 + *(int *)(iVar10 + 0x36c) < *(int *)((int)param_1 + 0x4c)) {
LAB_00434f68:
            iVar9 = iVar1;
            iVar13 = iVar8;
          }
          else if (*(int *)(iVar10 + 0x374) < *(int *)((int)param_1 + 0x4c)) {
            iVar9 = *(int *)(iVar10 + 0x374);
          }
        }
      }
      else {
        if (*(int *)((int)param_1 + 0x1c) == 2) {
          iVar8 = 1;
          uVar11 = FUN_005f4960((undefined4 *)(iVar10 + 4));
          if ((((uVar11 & 0x1000000) != 0) &&
              (uVar11 = FUN_005f4960((undefined4 *)(iVar10 + 4)), (uVar11 & 4) != 0)) &&
             (*(int *)((int)param_1 + 0x70) == 0)) {
            *(uint *)((int)param_1 + 0x28) = *(uint *)((int)param_1 + 0x28) | 0x40000000;
          }
        }
        iVar1 = *(int *)((int)param_1 + 0x4c);
        if (iVar1 < iVar8 + 2) {
          iVar9 = iVar1 - iVar8;
          iVar13 = 0;
        }
        else {
          iVar9 = *(int *)(iVar10 + 0x368);
          iVar13 = *(int *)(iVar10 + 0x364);
          if (iVar1 < iVar8 + *(int *)(iVar10 + 0x364) + iVar9) {
            iVar8 = (iVar1 - iVar9) - iVar8;
            iVar1 = iVar9;
            goto LAB_00434f68;
          }
        }
      }
      *(int *)((int)param_1 + 0x6c) = iVar9;
      iVar9 = *(int *)((int)param_1 + 100) - iVar9;
      iVar8 = *(int *)((int)param_1 + 0x74) - iVar13;
      *(int *)((int)param_1 + 0x7c) = iVar13;
      if (iVar9 < 1) {
        if (iVar9 < 0) {
          *(uint *)((int)param_1 + 0x28) = *(uint *)((int)param_1 + 0x28) | 0x8000;
        }
        if (iVar8 < 1) {
          *(int *)((int)param_1 + 0xdc) = iVar9 + iVar8;
          if (iVar9 + iVar8 < 0) {
            *(uint *)((int)param_1 + 0x28) = *(uint *)((int)param_1 + 0x28) | 0x4000;
          }
        }
        else {
          *(int *)((int)param_1 + 0xdc) = iVar8;
        }
      }
      else if (iVar8 < 1) {
        *(int *)((int)param_1 + 0xdc) = iVar9;
      }
      else {
        *(int *)((int)param_1 + 0xdc) = iVar8 + iVar9;
      }
      uVar11 = *(uint *)((int)param_1 + 0x28);
      if ((uVar11 & 1) != 0) {
        *(int *)((int)param_1 + 0x98) = *(int *)((int)param_1 + 0x98) << 1;
      }
      if ((uVar11 & 0x80000000) == 0) {
        if ((uVar11 & 0x3e00000) == 0) {
          uVar6 = *(undefined4 *)(iVar10 + 0x37c);
        }
        else {
          uVar6 = *(undefined4 *)(iVar10 + 0x380);
        }
      }
      else {
        uVar6 = *(undefined4 *)(iVar10 + 900);
      }
      *(undefined4 *)((int)param_1 + 0xa0) = uVar6;
      if (*(int *)((int)param_1 + 0xa0) < *(int *)((int)param_1 + 0x98)) {
        *(int *)((int)param_1 + 0xa0) = *(int *)((int)param_1 + 0x98);
      }
      if (((uVar11 & 2) == 0) || ((*(uint *)((int)param_1 + 0x30) & 0x30000) == 0)) {
        iVar9 = *(int *)((int)param_1 + 0xa0);
        if (*(int *)((int)param_1 + 0x90) != iVar9) {
          if (*(int *)((int)param_1 + 0x90) <= iVar9) {
            *(uint *)((int)param_1 + 0x28) = *(uint *)((int)param_1 + 0x28) | 0x2000;
            *(int *)((int)param_1 + 0x9c) =
                 *(int *)((int)param_1 + 0xa0) - *(int *)((int)param_1 + 0x90);
            goto LAB_00435097;
          }
          *(uint *)((int)param_1 + 0x28) = *(uint *)((int)param_1 + 0x28) | 0x10000;
          iVar9 = *(int *)((int)param_1 + 0xa0);
          *(int *)((int)param_1 + 0x94) = *(int *)((int)param_1 + 0x90) - iVar9;
        }
        if (*(int *)((int)param_1 + 0x8c) < iVar9) {
          *(uint *)((int)param_1 + 0x28) = *(uint *)((int)param_1 + 0x28) | 0x20000;
          *(int *)((int)param_1 + 0x9c) =
               *(int *)((int)param_1 + 0xa0) - *(int *)((int)param_1 + 0x8c);
        }
      }
LAB_00435097:
      if ((*(uint *)((int)param_1 + 0x28) & 0x80000000) == 0) {
        if ((*(uint *)((int)param_1 + 0x28) & 0x3e00000) == 0) {
          if (*(int *)((int)param_1 + 0x8c) < 1) {
            *(undefined4 *)((int)param_1 + 0xe0) = *(undefined4 *)((int)param_1 + 0xa4);
            *(undefined4 *)((int)param_1 + 0x11c) = *(undefined4 *)((int)param_1 + 0xa4);
          }
          else {
            *(int *)((int)param_1 + 0xe0) = *(int *)((int)param_1 + 0xa8) - *(int *)(iVar10 + 0x388)
            ;
            *(int *)((int)param_1 + 0x11c) =
                 *(int *)((int)param_1 + 0xac) - *(int *)(iVar10 + 0x388);
          }
        }
        else {
          *(int *)((int)param_1 + 0xe0) = *(int *)((int)param_1 + 0xa8) - *(int *)(iVar10 + 0x38c);
          *(int *)((int)param_1 + 0x11c) = *(int *)((int)param_1 + 0xac) - *(int *)(iVar10 + 0x38c);
        }
      }
      else {
        *(int *)((int)param_1 + 0xe0) = *(int *)((int)param_1 + 0xa8) - *(int *)(iVar10 + 0x390);
        *(int *)((int)param_1 + 0x11c) = *(int *)((int)param_1 + 0xac) - *(int *)(iVar10 + 0x390);
      }
      if (*(int *)((int)param_1 + 0xe0) < 0) {
        *(uint *)((int)param_1 + 0x28) = *(uint *)((int)param_1 + 0x28) | 0x40000;
      }
      if (*(int *)((int)param_1 + 0x11c) < 0) {
        *(uint *)((int)param_1 + 0x28) = *(uint *)((int)param_1 + 0x28) | 0x100000;
      }
      uVar11 = *(uint *)((int)param_1 + 0x28);
      iVar9 = 0;
      if ((uVar11 & 0x3800000) != 0) {
        iVar9 = *(int *)((int)param_1 + 0x60);
      }
      if ((uVar11 & 0x200000) != 0) {
        iVar9 = iVar9 + *(int *)((int)param_1 + 0x58);
      }
      if ((uVar11 & 0x400000) != 0) {
        iVar9 = iVar9 + *(int *)((int)param_1 + 0x5c);
      }
      iVar8 = *(int *)((int)param_1 + 100);
      if (iVar8 < *(int *)((int)param_1 + 0x6c)) {
        iVar8 = *(int *)((int)param_1 + 0x6c);
      }
      iVar13 = *(int *)((int)param_1 + 0x74);
      if (iVar13 < *(int *)((int)param_1 + 0x7c)) {
        iVar13 = *(int *)((int)param_1 + 0x7c);
      }
      iVar13 = iVar9 + iVar8 + iVar13;
      if ((*(uint *)((int)param_1 + 0x30) & 0x80000000) != 0) {
        iVar13 = iVar13 + 1;
      }
      *(int *)((int)param_1 + 0x114) = *(int *)((int)param_1 + 0x4c) - iVar13;
      if ((uVar11 & 3) == 0) {
        if ((0 < *(int *)(iVar10 + 0xa4)) && ((*(uint *)((int)param_1 + 0x30) & 0x8000000) == 0)) {
          *(uint *)((int)param_1 + 0x2c) = *(uint *)((int)param_1 + 0x2c) | 2;
        }
        if (((((*(uint *)((int)param_1 + 0x28) & 0x200000) != 0) &&
             (uVar11 = FUN_005f4960((undefined4 *)(iVar10 + 4)), (uVar11 & 0x2000000) == 0)) &&
            (local_184 != 0)) && (0 < *(int *)((int)param_1 + 0x90))) {
          *(uint *)((int)param_1 + 0x2c) = *(uint *)((int)param_1 + 0x2c) | 4;
        }
        if ((((*(uint *)((int)param_1 + 0x28) & 0x400000) != 0) &&
            (uVar11 = FUN_005f4960((undefined4 *)(iVar10 + 4)), (uVar11 & 0x4000000) == 0)) &&
           ((local_170 != 0 && (0 < *(int *)((int)param_1 + 0x90))))) {
          *(uint *)((int)param_1 + 0x2c) = *(uint *)((int)param_1 + 0x2c) | 8;
        }
        if ((((*(uint *)((int)param_1 + 0x28) & 0x3800000) != 0) &&
            (uVar11 = FUN_005f4960((undefined4 *)(iVar10 + 4)), (uVar11 & 0x8000000) == 0)) &&
           ((local_18c != 0 && (0 < *(int *)((int)param_1 + 0x90))))) {
          *(uint *)((int)param_1 + 0x2c) = *(uint *)((int)param_1 + 0x2c) | 0x10;
        }
      }
      if (((*(byte *)((int)param_1 + 0x28) & 1) != 0) &&
         ((*(uint *)((int)param_1 + 0x30) & 0x200) == 0)) {
        *(uint *)((int)param_1 + 0x2c) = *(uint *)((int)param_1 + 0x2c) | 0x20;
      }
      if ((((*(uint *)((int)param_1 + 0x2c) & 0x10000000) != 0) &&
          ((*(uint *)((int)param_1 + 0x30) & 0x200) == 0)) &&
         ((*(byte *)((int)param_1 + 0x28) & 1) == 0)) {
        *(uint *)((int)param_1 + 0x2c) = *(uint *)((int)param_1 + 0x2c) | 0x40;
      }
    }
    local_4 = CONCAT31(local_4._1_3_,0x15);
    FUN_00503ad0(local_16c);
  }
  if ((*(byte *)((int)param_1 + 0x30) & 1) == 0) {
    *(uint *)((int)param_1 + 0x2c) = *(uint *)((int)param_1 + 0x2c) | 0x80;
    if ((*(byte *)((int)param_1 + 0x30) & 4) == 0) goto LAB_004354e4;
  }
  else {
    *(uint *)((int)param_1 + 0x2c) = *(uint *)((int)param_1 + 0x2c) | 1;
  }
  *(uint *)((int)param_1 + 0x2c) = *(uint *)((int)param_1 + 0x2c) | 0x200;
LAB_004354e4:
  local_4._0_1_ = 0x13;
  FUN_00527650(local_14c);
  local_4._0_1_ = 0x11;
  FUN_00527360(local_10c);
  local_4._0_1_ = 0xf;
  FUN_0051c2f0(local_8c);
  local_4._0_1_ = 0xd;
  FUN_005279c0(local_cc);
  local_4._0_1_ = 0xb;
  FUN_0052c520(local_4c);
  local_4._0_1_ = 9;
  FUN_0052c840(local_12c);
  local_4._0_1_ = 6;
  FUN_0052cbb0(local_ec);
  local_4._0_1_ = 3;
  FUN_00536ea0(local_ac);
  local_4 = CONCAT31(local_4._1_3_,1);
  FUN_004fff70(local_6c);
  local_4 = 0xffffffff;
  FUN_00504d40(local_2c);
  ExceptionList = local_c;
  return;
}

