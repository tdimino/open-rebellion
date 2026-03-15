
void __thiscall FUN_004927c0(void *this,int param_1,int param_2)

{
  bool bVar1;
  ushort uVar2;
  uint uVar3;
  int iVar4;
  uint *puVar5;
  int *piVar6;
  int *piVar7;
  uint *puVar8;
  undefined4 uVar9;
  int *piVar10;
  int *piVar11;
  void *pvVar12;
  int *piVar13;
  int *local_148;
  int *local_140;
  uint local_13c;
  uint local_138;
  int *local_134;
  uint local_130;
  undefined4 local_12c [2];
  uint local_124;
  uint local_120;
  uint local_11c [2];
  uint local_114;
  uint local_110;
  undefined4 uStack_10c;
  undefined4 auStack_108 [2];
  uint local_100;
  undefined4 local_fc;
  undefined4 local_f8;
  undefined4 local_f4;
  undefined4 local_f0;
  undefined4 local_ec;
  undefined4 local_e8;
  undefined4 local_e4;
  undefined4 local_e0;
  undefined4 local_dc;
  undefined4 local_d8;
  undefined4 local_d4;
  undefined4 local_d0;
  undefined4 local_cc;
  undefined4 local_c8;
  undefined4 local_c4;
  undefined4 local_c0;
  undefined4 local_bc;
  undefined4 local_b8;
  undefined4 local_b4;
  undefined4 local_b0;
  undefined4 local_ac;
  undefined4 local_a8;
  undefined4 local_a4;
  undefined4 local_a0;
  undefined4 local_9c;
  undefined4 local_98;
  undefined4 local_94;
  undefined4 local_90;
  undefined4 local_8c;
  undefined4 local_88;
  undefined4 local_84;
  undefined4 local_80;
  undefined4 local_7c;
  undefined4 local_78;
  undefined4 local_74;
  undefined4 local_70;
  undefined4 local_6c;
  undefined4 local_68;
  undefined4 local_64;
  undefined4 local_60;
  undefined4 local_5c;
  undefined4 local_58;
  undefined4 local_54;
  undefined4 local_50;
  undefined4 local_4c;
  undefined4 local_48;
  undefined4 local_44;
  undefined4 local_40;
  undefined4 local_3c;
  undefined4 local_38;
  undefined4 local_34;
  undefined4 local_30;
  undefined4 local_2c;
  undefined4 local_28;
  undefined4 local_24;
  undefined4 local_20;
  undefined4 local_1c;
  undefined4 local_18;
  undefined4 uStack_14;
  undefined4 uStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00635f2f;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_004ece30(&local_13c);
  local_4 = 0;
  FUN_004f4340(local_12c);
  local_4._0_1_ = 1;
  local_138 = 0;
  if (param_1 != 0x1f0) goto LAB_00494b3a;
  FUN_004f26d0(&local_13c,(undefined4 *)(param_2 + 0x20));
  FUN_004f26d0((void *)((int)this + 0x78),(undefined4 *)(param_2 + 0x20));
  local_148 = (int *)0x0;
  local_134 = FUN_004f3000(*(int *)((int)this + 0x20),&local_13c);
  if (*(int *)((int)this + 100) != 0) goto LAB_00494b3a;
  *(undefined4 *)((int)this + 100) = 1;
  *(undefined4 *)((int)this + 0x40) = 1;
  *(ushort *)((int)this + 0x2e) = (*(int *)((int)this + 0x20) != 1) + 0x412;
  *(ushort *)((int)this + 0x30) = (*(int *)((int)this + 0x20) != 1) + 0x47c;
  local_cc = 0x52;
  local_c8 = 0x53;
  if ((local_13c >> 0x18 < 0x52) || (0x52 < local_13c >> 0x18)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_00619730();
  if (bVar1) {
    *(undefined2 *)((int)this + 0x2e) = 0x415;
    *(undefined4 *)((int)this + 0x80) = 0x52;
    *(undefined2 *)((int)this + 0x30) = 0x480;
  }
  else {
    local_ec = 0x51;
    local_e8 = 0x52;
    if ((local_13c >> 0x18 < 0x51) || (0x51 < local_13c >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (bVar1) {
      *(undefined4 *)((int)this + 0x80) = 0x51;
      *(undefined2 *)((int)this + 0x2e) = 0x414;
      *(undefined2 *)((int)this + 0x30) = 0x47e;
    }
    else {
      local_7c = 0x53;
      local_78 = 0x54;
      if ((local_13c >> 0x18 < 0x53) || (0x53 < local_13c >> 0x18)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (bVar1) {
        *(undefined4 *)((int)this + 0x80) = 0x53;
        *(undefined2 *)((int)this + 0x30) = 0x47f;
        *(undefined2 *)((int)this + 0x2e) = 0x3f9;
      }
      else {
        local_dc = 0x54;
        local_d8 = 0x55;
        if ((local_13c >> 0x18 < 0x54) || (0x54 < local_13c >> 0x18)) {
          bVar1 = false;
        }
        else {
          bVar1 = true;
        }
        FUN_00619730();
        if (bVar1) {
          *(undefined4 *)((int)this + 0x80) = 0x54;
        }
        else {
          local_2c = 0x55;
          local_28 = 0x56;
          if ((local_13c >> 0x18 < 0x55) || (0x55 < local_13c >> 0x18)) {
            bVar1 = false;
          }
          else {
            bVar1 = true;
          }
          FUN_00619730();
          if (bVar1) {
            *(undefined4 *)((int)this + 0x80) = 0x55;
          }
          else {
            local_fc = 0x56;
            local_f8 = 0x57;
            if ((local_13c >> 0x18 < 0x56) || (0x56 < local_13c >> 0x18)) {
              bVar1 = false;
            }
            else {
              bVar1 = true;
            }
            FUN_00619730();
            if (bVar1) {
              *(undefined4 *)((int)this + 0x80) = 0x56;
            }
            else {
              local_6c = 0x57;
              local_68 = 0x58;
              if ((local_13c >> 0x18 < 0x57) || (0x57 < local_13c >> 0x18)) {
                bVar1 = false;
              }
              else {
                bVar1 = true;
              }
              FUN_00619730();
              if (bVar1) {
                *(undefined4 *)((int)this + 0x80) = 0x57;
                *(undefined2 *)((int)this + 0x2e) = 0x3f2;
              }
              else {
                local_bc = 0x58;
                local_b8 = 0x59;
                if ((local_13c >> 0x18 < 0x58) || (0x58 < local_13c >> 0x18)) {
                  bVar1 = false;
                }
                else {
                  bVar1 = true;
                }
                FUN_00619730();
                if (bVar1) {
                  *(undefined4 *)((int)this + 0x80) = 0x58;
                }
                else {
                  local_3c = 0x61;
                  local_38 = 0x62;
                  if ((local_13c >> 0x18 < 0x61) || (0x61 < local_13c >> 0x18)) {
                    bVar1 = false;
                  }
                  else {
                    bVar1 = true;
                  }
                  FUN_00619730();
                  if (bVar1) {
                    *(undefined4 *)((int)this + 0x80) = 0x61;
                  }
                  else {
                    local_ac = 0x62;
                    local_a8 = 99;
                    if ((local_13c >> 0x18 < 0x62) || (0x62 < local_13c >> 0x18)) {
                      bVar1 = false;
                    }
                    else {
                      bVar1 = true;
                    }
                    FUN_00619730();
                    if (bVar1) {
                      *(undefined4 *)((int)this + 0x80) = 0x62;
                    }
                    else {
                      local_5c = 99;
                      local_58 = 100;
                      if ((local_13c >> 0x18 < 99) || (99 < local_13c >> 0x18)) {
                        bVar1 = false;
                      }
                      else {
                        bVar1 = true;
                      }
                      FUN_00619730();
                      if (bVar1) {
                        *(undefined4 *)((int)this + 0x80) = 99;
                      }
                      else {
                        local_9c = 0x69;
                        local_98 = 0x6a;
                        if ((local_13c >> 0x18 < 0x69) || (0x69 < local_13c >> 0x18)) {
                          bVar1 = false;
                        }
                        else {
                          bVar1 = true;
                        }
                        FUN_00619730();
                        if (bVar1) {
                          *(undefined4 *)((int)this + 0x80) = 0x69;
                        }
                        else {
                          local_1c = 0x6a;
                          local_18 = 0x6b;
                          if ((local_13c >> 0x18 < 0x6a) || (0x6a < local_13c >> 0x18)) {
                            bVar1 = false;
                          }
                          else {
                            bVar1 = true;
                          }
                          FUN_00619730();
                          if ((!bVar1) ||
                             (*(undefined4 *)((int)this + 0x80) = 0x6a,
                             *(int *)(param_2 + 0x28) == 3)) {
                            *(undefined4 *)((int)this + 0x40) = 0;
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  }
  piVar10 = local_134;
  if (local_134 == (int *)0x0) goto LAB_004948ce;
  if (*(int *)((int)this + 0x40) != 0) {
    FUN_004f26d0((uint *)((int)this + 0x5c),local_134 + 0x1e);
    local_8c = 0x90;
    uVar3 = *(uint *)((int)this + 0x5c) >> 0x18;
    local_88 = 0x98;
    if ((uVar3 < 0x90) || (0x97 < uVar3)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (!bVar1) {
      *(undefined4 *)((int)this + 0x40) = 0;
    }
  }
  if ((piVar10 == (int *)0x0) || (*(int *)((int)this + 0x40) == 0)) goto LAB_004948ce;
  local_140 = (int *)0x0;
  iVar4 = thunk_FUN_005f5060((int)(piVar10 + 0x27));
  local_138 = 1;
  FUN_004ece30(&local_130);
  local_4._0_1_ = 2;
  piVar10 = local_134;
  while ((local_134 = piVar10, iVar4 != 0 && (local_138 != 0))) {
    FUN_004f26d0(&local_130,(undefined4 *)(iVar4 + 0x18));
    local_140 = FUN_004f2d10(*(int *)((int)this + 0x20),&local_130);
    if (local_140 != (int *)0x0) {
      if ((*(byte *)(local_140 + 0x14) & 8) == 0) {
        local_4c = 0x30;
        local_48 = 0x3c;
        if ((local_130 >> 0x18 < 0x30) || (0x3b < local_130 >> 0x18)) {
          bVar1 = false;
        }
        else {
          bVar1 = true;
        }
        FUN_00619730();
        if ((bVar1) && ((*(byte *)(local_140 + 0x2b) & 1) != 0)) goto LAB_00492d8f;
      }
      else {
LAB_00492d8f:
        local_140 = (int *)0x0;
      }
      while( true ) {
        while( true ) {
          if ((local_140 == (int *)0x0) || (local_138 == 0)) goto LAB_00492e7c;
          puVar5 = FUN_004025b0(local_140,&local_100);
          local_f4 = 0x90;
          local_f0 = 0x98;
          local_4._0_1_ = 3;
          if ((*puVar5 >> 0x18 < 0x90) || (0x97 < *puVar5 >> 0x18)) {
            bVar1 = false;
          }
          else {
            bVar1 = true;
          }
          FUN_00619730();
          local_4._0_1_ = 2;
          FUN_00619730();
          if (!bVar1) break;
          local_138 = 0;
        }
        puVar5 = FUN_004025b0(local_140,&local_124);
        local_e4 = 8;
        local_e0 = 0x10;
        local_4._0_1_ = 4;
        if ((*puVar5 >> 0x18 < 8) || (0xf < *puVar5 >> 0x18)) {
          bVar1 = false;
        }
        else {
          bVar1 = true;
        }
        FUN_00619730();
        local_4._0_1_ = 2;
        FUN_00619730();
        if (bVar1) break;
        local_140 = (int *)local_140[7];
      }
      local_138 = 0;
    }
LAB_00492e7c:
    iVar4 = *(int *)(iVar4 + 0x10);
    piVar10 = local_134;
  }
  puVar5 = (uint *)((int)this + 0x74);
  FUN_004f26d0(puVar5,piVar10 + 0x1f);
  local_148 = FUN_004f2d10(*(int *)((int)this + 0x20),puVar5);
  puVar8 = (uint *)((int)this + 0x5c);
  local_138 = (uint)(*(int *)(param_2 + 0x28) == 3);
  piVar6 = FUN_004f3220(*(int *)((int)this + 0x20),puVar8);
  switch(*(undefined4 *)((int)this + 0x80)) {
  case 0x51:
    *(undefined4 *)((int)this + 0x84) = 5;
    iVar4 = FUN_004f5cd0(0x250);
    *(int *)((int)this + 0x90) = iVar4;
    if (iVar4 != 0) {
      *(undefined4 *)(iVar4 + 0x20) = *(undefined4 *)((int)this + 0x20);
      puVar8 = FUN_004025b0(piVar10,local_11c);
      local_4._0_1_ = 5;
      FUN_004f44b0(local_12c,puVar8,0);
      local_4._0_1_ = 2;
      FUN_00619730();
      (**(code **)(**(int **)((int)this + 0x90) + 0x24))(local_12c);
      uVar9 = FUN_004ece30(&local_110);
      puStack_8 = (undefined1 *)CONCAT31(puStack_8._1_3_,6);
      (**(code **)(**(int **)((int)this + 0x90) + 0x2c))(uVar9);
      local_4._0_1_ = 2;
      FUN_00619730();
    }
    local_d4 = 0x30;
    local_d0 = 0x38;
    if ((*puVar5 >> 0x18 < 0x30) || (0x37 < *puVar5 >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (bVar1) {
      if (piVar6 == (int *)0x0) {
        piVar10 = (int *)0x0;
      }
      else {
        piVar10 = piVar6 + 0xc;
      }
      if (local_148 == (int *)0x0) {
        piVar11 = (int *)0x0;
      }
      else {
        piVar11 = local_148 + 0xc;
      }
      FUN_0060b9d0((void *)((int)this + 0x44),0x70d0,piVar11,piVar10,(undefined4 *)0x0,
                   (undefined4 *)0x0);
      iVar4 = *(int *)(param_2 + 0x28);
      if (iVar4 == 1) {
        if (piVar6 == (int *)0x0) {
          piVar11 = (int *)0x0;
        }
        else {
          piVar11 = piVar6 + 0xc;
        }
        if (local_148 == (int *)0x0) {
          piVar10 = (int *)0x0;
          uVar3 = 0x70d3;
        }
        else {
          piVar10 = local_148 + 0xc;
          uVar3 = 0x70d3;
        }
        goto LAB_00493171;
      }
      if (iVar4 == 2) {
        if (piVar6 == (int *)0x0) {
          piVar11 = (int *)0x0;
        }
        else {
          piVar11 = piVar6 + 0xc;
        }
        if (local_148 == (int *)0x0) {
          piVar10 = (int *)0x0;
          uVar3 = 0x70d2;
        }
        else {
          piVar10 = local_148 + 0xc;
          uVar3 = 0x70d2;
        }
        goto LAB_00493171;
      }
      if (iVar4 == 3) {
        if (piVar6 == (int *)0x0) {
          piVar11 = (int *)0x0;
        }
        else {
          piVar11 = piVar6 + 0xc;
        }
        if (local_148 == (int *)0x0) {
          piVar10 = (int *)0x0;
          uVar3 = 0x70d1;
        }
        else {
          piVar10 = local_148 + 0xc;
          uVar3 = 0x70d1;
        }
        goto LAB_00493171;
      }
      *(undefined4 *)((int)this + 0x40) = 0;
    }
    else {
      if (piVar6 == (int *)0x0) {
        piVar10 = (int *)0x0;
      }
      else {
        piVar10 = piVar6 + 0xc;
      }
      if (local_148 == (int *)0x0) {
        piVar11 = (int *)0x0;
      }
      else {
        piVar11 = local_148 + 0xc;
      }
      FUN_0060b9d0((void *)((int)this + 0x44),0x70d4,piVar11,piVar10,(undefined4 *)0x0,
                   (undefined4 *)0x0);
      iVar4 = *(int *)(param_2 + 0x28);
      if (iVar4 == 1) {
        if (piVar6 == (int *)0x0) {
          piVar10 = (int *)0x0;
        }
        else {
          piVar10 = piVar6 + 0xc;
        }
        piVar11 = (int *)0x0;
        uVar3 = 0x70d7;
      }
      else if (iVar4 == 2) {
        if (piVar6 == (int *)0x0) {
          piVar11 = (int *)0x0;
          piVar10 = (int *)0x0;
          uVar3 = 0x70d6;
        }
        else {
          piVar10 = piVar6 + 0xc;
          piVar11 = (int *)0x0;
          uVar3 = 0x70d6;
        }
      }
      else {
        if (iVar4 != 3) {
          *(undefined4 *)((int)this + 0x40) = 0;
          goto LAB_00493179;
        }
        if (piVar6 == (int *)0x0) {
          piVar11 = (int *)0x0;
          piVar10 = (int *)0x0;
          uVar3 = 0x70d5;
        }
        else {
          piVar10 = piVar6 + 0xc;
          piVar11 = (int *)0x0;
          uVar3 = 0x70d5;
        }
      }
LAB_00493171:
      FUN_0060b9d0((void *)((int)this + 0x50),uVar3,piVar10,piVar11,(undefined4 *)0x0,
                   (undefined4 *)0x0);
    }
LAB_00493179:
    uVar3 = (uint)piVar6[0x21] >> 2 & 3;
    if (uVar3 == 1) {
      FUN_0060b9d0((void *)((int)this + 0x50),0x70d9,(undefined4 *)0x0,(undefined4 *)0x0,
                   (undefined4 *)0x0,(undefined4 *)0x0);
    }
    else if (uVar3 == 2) {
      FUN_0060b9d0((void *)((int)this + 0x50),0x70d8,(undefined4 *)0x0,(undefined4 *)0x0,
                   (undefined4 *)0x0,(undefined4 *)0x0);
    }
    else {
      FUN_0060b9d0((void *)((int)this + 0x50),0x70da,(undefined4 *)0x0,(undefined4 *)0x0,
                   (undefined4 *)0x0,(undefined4 *)0x0);
    }
LAB_004941d6:
    FUN_0060b9d0((void *)((int)this + 0x50),0x70db,(undefined4 *)0x0,(undefined4 *)0x0,
                 (undefined4 *)0x0,(undefined4 *)0x0);
    break;
  case 0x52:
    *(undefined4 *)((int)this + 0x40) = 1;
    *(undefined4 *)((int)this + 0x84) = 3;
    FUN_004f26d0(puVar8,piVar10 + 0x1d);
    local_74 = 0x30;
    local_70 = 0x38;
    if ((*puVar5 >> 0x18 < 0x30) || (0x37 < *puVar5 >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (bVar1) {
      if (piVar6 == (int *)0x0) {
        piVar10 = (int *)0x0;
      }
      else {
        piVar10 = piVar6 + 0xc;
      }
      if (local_148 == (int *)0x0) {
        piVar11 = (int *)0x0;
      }
      else {
        piVar11 = local_148 + 0xc;
      }
      FUN_0060b9d0((void *)((int)this + 0x44),0x7108,piVar11,piVar10,(undefined4 *)0x0,
                   (undefined4 *)0x0);
      if (*(int *)(param_2 + 0x28) == 2) {
        if (piVar6 == (int *)0x0) {
          piVar6 = (int *)0x0;
        }
        else {
          piVar6 = piVar6 + 0xc;
        }
        if (local_148 == (int *)0x0) {
          piVar10 = (int *)0x0;
          uVar3 = 0x710a;
        }
        else {
          piVar10 = local_148 + 0xc;
          uVar3 = 0x710a;
        }
LAB_00493f46:
        FUN_0060b9d0((void *)((int)this + 0x50),uVar3,piVar10,piVar6,(undefined4 *)0x0,
                     (undefined4 *)0x0);
      }
      else if (*(int *)(param_2 + 0x28) == 3) {
        if (piVar6 == (int *)0x0) {
          piVar6 = (int *)0x0;
        }
        else {
          piVar6 = piVar6 + 0xc;
        }
        if (local_148 == (int *)0x0) {
          piVar10 = (int *)0x0;
        }
        else {
          piVar10 = local_148 + 0xc;
        }
        FUN_0060b9d0((void *)((int)this + 0x50),0x7109,piVar10,piVar6,(undefined4 *)0x0,
                     (undefined4 *)0x0);
        FUN_00494df0((int)this);
      }
      else {
        *(undefined4 *)((int)this + 0x40) = 0;
      }
    }
    else {
      if (piVar6 == (int *)0x0) {
        piVar10 = (int *)0x0;
      }
      else {
        piVar10 = piVar6 + 0xc;
      }
      FUN_0060b9d0((void *)((int)this + 0x44),0x710b,piVar10,(undefined4 *)0x0,(undefined4 *)0x0,
                   (undefined4 *)0x0);
      if (*(int *)(param_2 + 0x28) == 2) {
        if (piVar6 == (int *)0x0) {
          piVar10 = (int *)0x0;
        }
        else {
          piVar10 = piVar6 + 0xc;
        }
        piVar6 = (int *)0x0;
        uVar3 = 0x710d;
        goto LAB_00493f46;
      }
      if (*(int *)(param_2 + 0x28) == 3) {
        if (piVar6 == (int *)0x0) {
          piVar6 = (int *)0x0;
        }
        else {
          piVar6 = piVar6 + 0xc;
        }
        FUN_0060b9d0((void *)((int)this + 0x50),0x710c,piVar6,(undefined4 *)0x0,(undefined4 *)0x0,
                     (undefined4 *)0x0);
        FUN_00494df0((int)this);
      }
      else {
        *(undefined4 *)((int)this + 0x40) = 0;
      }
    }
    goto joined_r0x004946ad;
  case 0x53:
    *(undefined4 *)((int)this + 0x84) = 5;
    iVar4 = FUN_004f5cd0(0x250);
    *(int *)((int)this + 0x90) = iVar4;
    if (iVar4 != 0) {
      *(undefined4 *)(iVar4 + 0x20) = *(undefined4 *)((int)this + 0x20);
      puVar8 = FUN_004025b0(piVar10,&local_110);
      local_4._0_1_ = 0xb;
      FUN_004f44b0(local_12c,puVar8,0);
      local_4._0_1_ = 2;
      FUN_00619730();
      (**(code **)(**(int **)((int)this + 0x90) + 0x24))(local_12c);
      uVar9 = FUN_004ece30(&uStack_10c);
      puStack_8 = (undefined1 *)CONCAT31(puStack_8._1_3_,0xc);
      (**(code **)(**(int **)((int)this + 0x90) + 0x2c))(uVar9);
      local_4._0_1_ = 2;
      FUN_00619730();
    }
    local_54 = 0x30;
    local_50 = 0x38;
    if ((*puVar5 >> 0x18 < 0x30) || (0x37 < *puVar5 >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (bVar1) {
      piVar13 = piVar10 + 0xc;
      if (piVar6 == (int *)0x0) {
        piVar11 = (int *)0x0;
      }
      else {
        piVar11 = piVar6 + 0xc;
      }
      if (local_148 == (int *)0x0) {
        piVar7 = (int *)0x0;
      }
      else {
        piVar7 = local_148 + 0xc;
      }
      FUN_0060b9d0((void *)((int)this + 0x44),0x7120,piVar7,piVar11,piVar13,(undefined4 *)0x0);
      if (*(int *)(param_2 + 0x28) == 2) {
        if (piVar6 == (int *)0x0) {
          piVar11 = (int *)0x0;
        }
        else {
          piVar11 = piVar6 + 0xc;
        }
        if (local_148 == (int *)0x0) {
          piVar6 = (int *)0x0;
          uVar3 = 0x7123;
        }
        else {
          piVar6 = local_148 + 0xc;
          uVar3 = 0x7123;
        }
        goto LAB_00494491;
      }
      if (*(int *)(param_2 + 0x28) == 3) {
        if (piVar10[0x2b] == 0) {
          if (piVar6 == (int *)0x0) {
            piVar11 = (int *)0x0;
          }
          else {
            piVar11 = piVar6 + 0xc;
          }
          if (local_148 == (int *)0x0) {
            piVar6 = (int *)0x0;
            uVar3 = 0x7121;
          }
          else {
            piVar6 = local_148 + 0xc;
            uVar3 = 0x7121;
          }
        }
        else {
          if (piVar6 == (int *)0x0) {
            piVar11 = (int *)0x0;
          }
          else {
            piVar11 = piVar6 + 0xc;
          }
          if (local_148 == (int *)0x0) {
            piVar6 = (int *)0x0;
            uVar3 = 0x7122;
          }
          else {
            piVar6 = local_148 + 0xc;
            uVar3 = 0x7122;
          }
        }
        goto LAB_00494491;
      }
      *(undefined4 *)((int)this + 0x40) = 0;
    }
    else {
      piVar11 = piVar10 + 0xc;
      if (piVar6 == (int *)0x0) {
        piVar13 = (int *)0x0;
      }
      else {
        piVar13 = piVar6 + 0xc;
      }
      FUN_0060b9d0((void *)((int)this + 0x44),0x7124,piVar13,piVar11,(undefined4 *)0x0,
                   (undefined4 *)0x0);
      if (*(int *)(param_2 + 0x28) == 2) {
        if (piVar6 == (int *)0x0) {
          piVar6 = (int *)0x0;
        }
        else {
          piVar6 = piVar6 + 0xc;
        }
        piVar13 = (int *)0x0;
        uVar3 = 0x7127;
      }
      else {
        if (*(int *)(param_2 + 0x28) != 3) {
          *(undefined4 *)((int)this + 0x40) = 0;
          goto LAB_00494499;
        }
        if (piVar10[0x2b] == 0) {
          if (piVar6 == (int *)0x0) {
            piVar13 = (int *)0x0;
            piVar6 = (int *)0x0;
            uVar3 = 0x7125;
          }
          else {
            piVar6 = piVar6 + 0xc;
            piVar13 = (int *)0x0;
            uVar3 = 0x7125;
          }
        }
        else if (piVar6 == (int *)0x0) {
          piVar13 = (int *)0x0;
          piVar6 = (int *)0x0;
          uVar3 = 0x7126;
        }
        else {
          piVar6 = piVar6 + 0xc;
          piVar13 = (int *)0x0;
          uVar3 = 0x7126;
        }
      }
LAB_00494491:
      FUN_0060b9d0((void *)((int)this + 0x50),uVar3,piVar6,piVar11,piVar13,(undefined4 *)0x0);
    }
LAB_00494499:
    FUN_0060b9d0((void *)((int)this + 0x50),0x70db,(undefined4 *)0x0,(undefined4 *)0x0,
                 (undefined4 *)0x0,(undefined4 *)0x0);
    break;
  case 0x54:
    if (local_148 != (int *)0x0) {
      FUN_0060b9d0((void *)((int)this + 0x44),0x70e8,(undefined4 *)0x0,(undefined4 *)0x0,
                   (undefined4 *)0x0,(undefined4 *)0x0);
      if (*(int *)(param_2 + 0x28) == 2) {
        if (piVar6 == (int *)0x0) {
          piVar6 = (int *)0x0;
        }
        else {
          piVar6 = piVar6 + 0xc;
        }
        uVar3 = 0x70ea;
LAB_00493413:
        FUN_0060b9d0((void *)((int)this + 0x50),uVar3,piVar6,(undefined4 *)0x0,(undefined4 *)0x0,
                     (undefined4 *)0x0);
      }
      else {
        if (*(int *)(param_2 + 0x28) == 3) {
          if (piVar6 == (int *)0x0) {
            piVar6 = (int *)0x0;
          }
          else {
            piVar6 = piVar6 + 0xc;
          }
          uVar3 = 0x70e9;
          goto LAB_00493413;
        }
        *(undefined4 *)((int)this + 0x40) = 0;
      }
      if (local_140 != (int *)0x0) {
        FUN_0060b9d0((void *)((int)this + 0x50),0x70e6,local_140 + 0xc,(undefined4 *)0x0,
                     (undefined4 *)0x0,(undefined4 *)0x0);
      }
      uVar3 = FUN_004c5000(local_148);
      *(short *)((int)this + 0x2e) = (short)uVar3;
    }
    break;
  case 0x55:
    *(undefined4 *)((int)this + 0x84) = 3;
    local_84 = 0x30;
    local_80 = 0x38;
    if ((*puVar5 >> 0x18 < 0x30) || (0x37 < *puVar5 >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (bVar1) {
      if (*(int *)(param_2 + 0x28) == 2) {
        if (piVar6 == (int *)0x0) {
          piVar10 = (int *)0x0;
        }
        else {
          piVar10 = piVar6 + 0xc;
        }
        if (local_148 == (int *)0x0) {
          piVar11 = (int *)0x0;
        }
        else {
          piVar11 = local_148 + 0xc;
        }
        FUN_0060b9d0((void *)((int)this + 0x44),0x7118,piVar11,piVar10,(undefined4 *)0x0,
                     (undefined4 *)0x0);
        if (piVar6 == (int *)0x0) {
          piVar6 = (int *)0x0;
        }
        else {
          piVar6 = piVar6 + 0xc;
        }
        if (local_148 == (int *)0x0) {
          piVar10 = (int *)0x0;
        }
        else {
          piVar10 = local_148 + 0xc;
        }
        FUN_0060b9d0((void *)((int)this + 0x50),0x711a,piVar10,piVar6,(undefined4 *)0x0,
                     (undefined4 *)0x0);
      }
      else {
        if (*(int *)(param_2 + 0x28) != 3) goto switchD_00492ef6_caseD_59;
        piVar10 = FUN_004f2d10(*(int *)((int)this + 0x20),(uint *)(piVar10 + 0x2a));
        if (piVar10 == (int *)0x0) {
          piVar11 = (int *)0x0;
        }
        else {
          piVar11 = piVar10 + 0xc;
        }
        if (piVar6 == (int *)0x0) {
          piVar13 = (int *)0x0;
        }
        else {
          piVar13 = piVar6 + 0xc;
        }
        if (local_148 == (int *)0x0) {
          piVar7 = (int *)0x0;
        }
        else {
          piVar7 = local_148 + 0xc;
        }
        FUN_0060b9d0((void *)((int)this + 0x44),0x711b,piVar7,piVar13,piVar11,(undefined4 *)0x0);
        if (piVar10 == (int *)0x0) {
          piVar10 = (int *)0x0;
        }
        else {
          piVar10 = piVar10 + 0xc;
        }
        if (piVar6 == (int *)0x0) {
          piVar6 = (int *)0x0;
        }
        else {
          piVar6 = piVar6 + 0xc;
        }
        if (local_148 == (int *)0x0) {
          piVar11 = (int *)0x0;
        }
        else {
          piVar11 = local_148 + 0xc;
        }
        FUN_0060b9d0((void *)((int)this + 0x50),0x7119,piVar11,piVar6,piVar10,(undefined4 *)0x0);
      }
    }
    break;
  case 0x56:
    *(undefined4 *)((int)this + 0x84) = 5;
    iVar4 = FUN_004f5cd0(0x250);
    *(int *)((int)this + 0x90) = iVar4;
    if (iVar4 != 0) {
      *(undefined4 *)(iVar4 + 0x20) = *(undefined4 *)((int)this + 0x20);
      puVar8 = FUN_004025b0(piVar10,&local_114);
      local_4._0_1_ = 7;
      FUN_004f44b0(local_12c,puVar8,0);
      local_4._0_1_ = 2;
      FUN_00619730();
      (**(code **)(**(int **)((int)this + 0x90) + 0x24))(local_12c);
      uVar9 = FUN_004ece30(auStack_108);
      puStack_8 = (undefined1 *)CONCAT31(puStack_8._1_3_,8);
      (**(code **)(**(int **)((int)this + 0x90) + 0x2c))(uVar9);
      local_4._0_1_ = 2;
      FUN_00619730();
    }
    local_94 = 0x30;
    local_90 = 0x38;
    if ((*puVar5 >> 0x18 < 0x30) || (0x37 < *puVar5 >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (bVar1) {
      if (piVar6 == (int *)0x0) {
        piVar10 = (int *)0x0;
      }
      else {
        piVar10 = piVar6 + 0xc;
      }
      if (local_148 == (int *)0x0) {
        piVar11 = (int *)0x0;
      }
      else {
        piVar11 = local_148 + 0xc;
      }
      FUN_0060b9d0((void *)((int)this + 0x44),0x7100,piVar11,piVar10,(undefined4 *)0x0,
                   (undefined4 *)0x0);
      if (*(int *)(param_2 + 0x28) == 2) {
        if ((*(byte *)(piVar6 + 0x22) & 4) == 0) {
          if (piVar6 == (int *)0x0) {
            piVar6 = (int *)0x0;
          }
          else {
            piVar6 = piVar6 + 0xc;
          }
          if (local_148 == (int *)0x0) {
            piVar10 = (int *)0x0;
          }
          else {
            piVar10 = local_148 + 0xc;
          }
          FUN_0060b9d0((void *)((int)this + 0x50),0x7102,piVar10,piVar6,(undefined4 *)0x0,
                       (undefined4 *)0x0);
        }
        else {
          if (piVar6 == (int *)0x0) {
            piVar6 = (int *)0x0;
          }
          else {
            piVar6 = piVar6 + 0xc;
          }
          if (local_148 == (int *)0x0) {
            FUN_0060b9d0((void *)((int)this + 0x50),0x7107,(undefined4 *)0x0,piVar6,
                         (undefined4 *)0x0,(undefined4 *)0x0);
          }
          else {
            FUN_0060b9d0((void *)((int)this + 0x50),0x7107,local_148 + 0xc,piVar6,(undefined4 *)0x0,
                         (undefined4 *)0x0);
          }
        }
        goto LAB_004941d6;
      }
      if (*(int *)(param_2 + 0x28) == 3) {
        *(undefined2 *)((int)this + 0x2e) = 0x3ed;
        if (piVar6 == (int *)0x0) {
          piVar6 = (int *)0x0;
        }
        else {
          piVar6 = piVar6 + 0xc;
        }
        if (local_148 == (int *)0x0) {
          piVar10 = (int *)0x0;
          uVar3 = 0x7101;
        }
        else {
          piVar10 = local_148 + 0xc;
          uVar3 = 0x7101;
        }
LAB_00494193:
        FUN_0060b9d0((void *)((int)this + 0x50),uVar3,piVar10,piVar6,(undefined4 *)0x0,
                     (undefined4 *)0x0);
        *(undefined4 *)((int)this + 0x84) = 3;
      }
      else {
        *(undefined4 *)((int)this + 0x40) = 0;
      }
    }
    else {
      if (piVar6 == (int *)0x0) {
        piVar10 = (int *)0x0;
      }
      else {
        piVar10 = piVar6 + 0xc;
      }
      FUN_0060b9d0((void *)((int)this + 0x44),0x7103,piVar10,(undefined4 *)0x0,(undefined4 *)0x0,
                   (undefined4 *)0x0);
      if (*(int *)(param_2 + 0x28) == 2) {
        if ((*(byte *)(piVar6 + 0x22) & 4) == 0) {
          if (piVar6 == (int *)0x0) {
            piVar6 = (int *)0x0;
          }
          else {
            piVar6 = piVar6 + 0xc;
          }
          FUN_0060b9d0((void *)((int)this + 0x50),0x7105,piVar6,(undefined4 *)0x0,(undefined4 *)0x0,
                       (undefined4 *)0x0);
        }
        else if (piVar6 == (int *)0x0) {
          FUN_0060b9d0((void *)((int)this + 0x50),0x7106,(undefined4 *)0x0,(undefined4 *)0x0,
                       (undefined4 *)0x0,(undefined4 *)0x0);
        }
        else {
          FUN_0060b9d0((void *)((int)this + 0x50),0x7106,piVar6 + 0xc,(undefined4 *)0x0,
                       (undefined4 *)0x0,(undefined4 *)0x0);
        }
        goto LAB_004941d6;
      }
      if (*(int *)(param_2 + 0x28) == 3) {
        *(undefined2 *)((int)this + 0x2e) = 0x3ed;
        if (piVar6 == (int *)0x0) {
          piVar6 = (int *)0x0;
          piVar10 = (int *)0x0;
          uVar3 = 0x7104;
        }
        else {
          piVar10 = piVar6 + 0xc;
          piVar6 = (int *)0x0;
          uVar3 = 0x7104;
        }
        goto LAB_00494193;
      }
      *(undefined4 *)((int)this + 0x40) = 0;
    }
    break;
  case 0x57:
    *(undefined4 *)((int)this + 0x84) = 5;
    iVar4 = FUN_004f5cd0(0x250);
    *(int *)((int)this + 0x90) = iVar4;
    if (iVar4 != 0) {
      *(undefined4 *)(iVar4 + 0x20) = *(undefined4 *)((int)this + 0x20);
      puVar8 = FUN_004025b0(piVar10,&local_120);
      local_4._0_1_ = 9;
      FUN_004f44b0(local_12c,puVar8,0);
      local_4._0_1_ = 2;
      FUN_00619730();
      (**(code **)(**(int **)((int)this + 0x90) + 0x24))(local_12c);
      uVar9 = FUN_004ece30(local_11c);
      puStack_8 = (undefined1 *)CONCAT31(puStack_8._1_3_,10);
      (**(code **)(**(int **)((int)this + 0x90) + 0x2c))(uVar9);
      local_4._0_1_ = 2;
      FUN_00619730();
    }
    local_64 = 0x30;
    local_60 = 0x38;
    if ((*puVar5 >> 0x18 < 0x30) || (0x37 < *puVar5 >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (bVar1) {
      if (piVar6 == (int *)0x0) {
        piVar10 = (int *)0x0;
      }
      else {
        piVar10 = piVar6 + 0xc;
      }
      if (local_148 == (int *)0x0) {
        piVar11 = (int *)0x0;
      }
      else {
        piVar11 = local_148 + 0xc;
      }
      FUN_0060b9d0((void *)((int)this + 0x44),0x7110,piVar11,piVar10,(undefined4 *)0x0,
                   (undefined4 *)0x0);
      if (*(int *)(param_2 + 0x28) == 2) {
        if (piVar6 == (int *)0x0) {
          piVar6 = (int *)0x0;
        }
        else {
          piVar6 = piVar6 + 0xc;
        }
        if (local_148 == (int *)0x0) {
          piVar10 = (int *)0x0;
        }
        else {
          piVar10 = local_148 + 0xc;
        }
        FUN_0060b9d0((void *)((int)this + 0x50),0x7112,piVar10,piVar6,(undefined4 *)0x0,
                     (undefined4 *)0x0);
        FUN_0060b9d0((void *)((int)this + 0x50),0x70db,(undefined4 *)0x0,(undefined4 *)0x0,
                     (undefined4 *)0x0,(undefined4 *)0x0);
        *(undefined2 *)((int)this + 0x2e) = 0x3f2;
        *(undefined2 *)((int)this + 0x30) = 0x454;
      }
      else {
        if (*(int *)(param_2 + 0x28) == 3) {
          *(undefined2 *)((int)this + 0x30) = 0x455;
          *(ushort *)((int)this + 0x2e) = (*(int *)((int)this + 0x20) != 1) + 0x3f3;
          if (piVar6 == (int *)0x0) {
            piVar6 = (int *)0x0;
          }
          else {
            piVar6 = piVar6 + 0xc;
          }
          if (local_148 == (int *)0x0) {
            piVar10 = (int *)0x0;
            uVar3 = 0x7111;
          }
          else {
            piVar10 = local_148 + 0xc;
            uVar3 = 0x7111;
          }
          goto LAB_00494193;
        }
        *(undefined4 *)((int)this + 0x40) = 0;
      }
    }
    else {
      if (piVar6 == (int *)0x0) {
        piVar10 = (int *)0x0;
      }
      else {
        piVar10 = piVar6 + 0xc;
      }
      FUN_0060b9d0((void *)((int)this + 0x44),0x7113,piVar10,(undefined4 *)0x0,(undefined4 *)0x0,
                   (undefined4 *)0x0);
      if (*(int *)(param_2 + 0x28) == 2) {
        if (piVar6 == (int *)0x0) {
          piVar6 = (int *)0x0;
        }
        else {
          piVar6 = piVar6 + 0xc;
        }
        FUN_0060b9d0((void *)((int)this + 0x50),0x7115,piVar6,(undefined4 *)0x0,(undefined4 *)0x0,
                     (undefined4 *)0x0);
        *(undefined2 *)((int)this + 0x2e) = 0x3f2;
        *(undefined2 *)((int)this + 0x30) = 0x454;
        goto LAB_004941d6;
      }
      if (*(int *)(param_2 + 0x28) == 3) {
        *(undefined2 *)((int)this + 0x30) = 0x455;
        *(ushort *)((int)this + 0x2e) = (*(int *)((int)this + 0x20) != 1) + 0x3f3;
        if (piVar6 == (int *)0x0) {
          piVar10 = (int *)0x0;
        }
        else {
          piVar10 = piVar6 + 0xc;
        }
        piVar6 = (int *)0x0;
        uVar3 = 0x7114;
        goto LAB_00494193;
      }
      *(undefined4 *)((int)this + 0x40) = 0;
    }
    break;
  case 0x58:
    *(undefined4 *)((int)this + 0x84) = 3;
    piVar10 = FUN_004f2d10(*(int *)((int)this + 0x20),(uint *)(local_134 + 0x1c));
    FUN_004f26d0(puVar8,local_134 + 0x1e);
    local_34 = 0x30;
    local_30 = 0x38;
    if ((*puVar5 >> 0x18 < 0x30) || (0x37 < *puVar5 >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (bVar1) {
      if (piVar10 == (int *)0x0) {
        piVar11 = (int *)0x0;
      }
      else {
        piVar11 = piVar10 + 0xc;
      }
      if (piVar6 == (int *)0x0) {
        piVar13 = (int *)0x0;
      }
      else {
        piVar13 = piVar6 + 0xc;
      }
      if (local_148 == (int *)0x0) {
        piVar7 = (int *)0x0;
      }
      else {
        piVar7 = local_148 + 0xc;
      }
      FUN_0060b9d0((void *)((int)this + 0x44),0x71c0,piVar7,piVar13,piVar11,(undefined4 *)0x0);
      if (*(int *)(param_2 + 0x28) == 2) {
        if (piVar10 == (int *)0x0) {
          piVar10 = (int *)0x0;
        }
        else {
          piVar10 = piVar10 + 0xc;
        }
        if (piVar6 == (int *)0x0) {
          piVar6 = (int *)0x0;
        }
        else {
          piVar6 = piVar6 + 0xc;
        }
        if (local_148 == (int *)0x0) {
          piVar11 = (int *)0x0;
        }
        else {
          piVar11 = local_148 + 0xc;
        }
        FUN_0060b9d0((void *)((int)this + 0x50),0x71c2,piVar11,piVar6,piVar10,(undefined4 *)0x0);
      }
      else {
        if (*(int *)(param_2 + 0x28) != 3) goto switchD_00492ef6_caseD_59;
        if (piVar10 == (int *)0x0) {
          piVar10 = (int *)0x0;
        }
        else {
          piVar10 = piVar10 + 0xc;
        }
        if (piVar6 == (int *)0x0) {
          piVar6 = (int *)0x0;
        }
        else {
          piVar6 = piVar6 + 0xc;
        }
        if (local_148 == (int *)0x0) {
          piVar11 = (int *)0x0;
        }
        else {
          piVar11 = local_148 + 0xc;
        }
        FUN_0060b9d0((void *)((int)this + 0x50),0x71c1,piVar11,piVar6,piVar10,(undefined4 *)0x0);
      }
    }
    else {
      if (piVar10 == (int *)0x0) {
        piVar11 = (int *)0x0;
      }
      else {
        piVar11 = piVar10 + 0xc;
      }
      if (piVar6 == (int *)0x0) {
        piVar13 = (int *)0x0;
      }
      else {
        piVar13 = piVar6 + 0xc;
      }
      FUN_0060b9d0((void *)((int)this + 0x44),0x71c3,piVar13,piVar11,(undefined4 *)0x0,
                   (undefined4 *)0x0);
      if (*(int *)(param_2 + 0x28) == 2) {
        if (piVar10 == (int *)0x0) {
          piVar10 = (int *)0x0;
        }
        else {
          piVar10 = piVar10 + 0xc;
        }
        if (piVar6 == (int *)0x0) {
          piVar6 = (int *)0x0;
        }
        else {
          piVar6 = piVar6 + 0xc;
        }
        FUN_0060b9d0((void *)((int)this + 0x50),0x71c5,piVar6,piVar10,(undefined4 *)0x0,
                     (undefined4 *)0x0);
      }
      else {
        if (*(int *)(param_2 + 0x28) != 3) goto switchD_00492ef6_caseD_59;
        if (piVar10 == (int *)0x0) {
          piVar10 = (int *)0x0;
        }
        else {
          piVar10 = piVar10 + 0xc;
        }
        if (piVar6 == (int *)0x0) {
          piVar6 = (int *)0x0;
        }
        else {
          piVar6 = piVar6 + 0xc;
        }
        FUN_0060b9d0((void *)((int)this + 0x50),0x71c4,piVar6,piVar10,(undefined4 *)0x0,
                     (undefined4 *)0x0);
      }
    }
    break;
  default:
switchD_00492ef6_caseD_59:
    *(undefined4 *)((int)this + 0x40) = 0;
    break;
  case 0x61:
    *(undefined4 *)((int)this + 0x84) = 3;
    piVar10 = FUN_004f2d10(*(int *)((int)this + 0x20),(uint *)(piVar10 + 0x1c));
    local_44 = 0x30;
    local_40 = 0x38;
    if ((*puVar5 >> 0x18 < 0x30) || (0x37 < *puVar5 >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (bVar1) {
      if (piVar10 == (int *)0x0) {
        piVar11 = (int *)0x0;
      }
      else {
        piVar11 = piVar10 + 0xc;
      }
      if (piVar6 == (int *)0x0) {
        piVar13 = (int *)0x0;
      }
      else {
        piVar13 = piVar6 + 0xc;
      }
      if (local_148 == (int *)0x0) {
        piVar7 = (int *)0x0;
      }
      else {
        piVar7 = local_148 + 0xc;
      }
      FUN_0060b9d0((void *)((int)this + 0x44),0x7130,piVar7,piVar13,piVar11,(undefined4 *)0x0);
      if (*(int *)(param_2 + 0x28) == 2) {
        if (piVar10 == (int *)0x0) {
          piVar10 = (int *)0x0;
        }
        else {
          piVar10 = piVar10 + 0xc;
        }
        if (piVar6 == (int *)0x0) {
          piVar11 = (int *)0x0;
        }
        else {
          piVar11 = piVar6 + 0xc;
        }
        if (local_148 == (int *)0x0) {
          piVar6 = (int *)0x0;
          uVar3 = 0x7132;
        }
        else {
          piVar6 = local_148 + 0xc;
          uVar3 = 0x7132;
        }
      }
      else {
        if (*(int *)(param_2 + 0x28) != 3) {
          *(undefined4 *)((int)this + 0x40) = 0;
          goto joined_r0x004946ad;
        }
        if (piVar10 == (int *)0x0) {
          piVar10 = (int *)0x0;
        }
        else {
          piVar10 = piVar10 + 0xc;
        }
        if (piVar6 == (int *)0x0) {
          piVar11 = (int *)0x0;
        }
        else {
          piVar11 = piVar6 + 0xc;
        }
        if (local_148 == (int *)0x0) {
          piVar6 = (int *)0x0;
          uVar3 = 0x7131;
        }
        else {
          piVar6 = local_148 + 0xc;
          uVar3 = 0x7131;
        }
      }
    }
    else {
      if (piVar10 == (int *)0x0) {
        piVar11 = (int *)0x0;
      }
      else {
        piVar11 = piVar10 + 0xc;
      }
      if (piVar6 == (int *)0x0) {
        piVar13 = (int *)0x0;
      }
      else {
        piVar13 = piVar6 + 0xc;
      }
      FUN_0060b9d0((void *)((int)this + 0x44),0x7133,piVar13,piVar11,(undefined4 *)0x0,
                   (undefined4 *)0x0);
      if (*(int *)(param_2 + 0x28) == 2) {
        if (piVar10 == (int *)0x0) {
          piVar11 = (int *)0x0;
        }
        else {
          piVar11 = piVar10 + 0xc;
        }
        if (piVar6 == (int *)0x0) {
          piVar6 = (int *)0x0;
        }
        else {
          piVar6 = piVar6 + 0xc;
        }
        piVar10 = (int *)0x0;
        uVar3 = 0x7135;
      }
      else {
        if (*(int *)(param_2 + 0x28) != 3) {
          *(undefined4 *)((int)this + 0x40) = 0;
          goto joined_r0x004946ad;
        }
        if (piVar10 == (int *)0x0) {
          piVar11 = (int *)0x0;
        }
        else {
          piVar11 = piVar10 + 0xc;
        }
        if (piVar6 == (int *)0x0) {
          piVar10 = (int *)0x0;
          piVar6 = (int *)0x0;
          uVar3 = 0x7134;
        }
        else {
          piVar6 = piVar6 + 0xc;
          piVar10 = (int *)0x0;
          uVar3 = 0x7134;
        }
      }
    }
    FUN_0060b9d0((void *)((int)this + 0x50),uVar3,piVar6,piVar11,piVar10,(undefined4 *)0x0);
joined_r0x004946ad:
    if (local_140 != (int *)0x0) {
      FUN_0060b9d0((void *)((int)this + 0x50),0x70e6,local_140 + 0xc,(undefined4 *)0x0,
                   (undefined4 *)0x0,(undefined4 *)0x0);
    }
    break;
  case 0x62:
    *(undefined4 *)((int)this + 0x84) = 3;
    piVar10 = FUN_004f2d10(*(int *)((int)this + 0x20),(uint *)(piVar10 + 0x1c));
    local_b4 = 0x30;
    local_b0 = 0x38;
    if ((*puVar5 >> 0x18 < 0x30) || (0x37 < *puVar5 >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (bVar1) {
      if (piVar10 == (int *)0x0) {
        piVar11 = (int *)0x0;
      }
      else {
        piVar11 = piVar10 + 0xc;
      }
      if (piVar6 == (int *)0x0) {
        piVar13 = (int *)0x0;
      }
      else {
        piVar13 = piVar6 + 0xc;
      }
      if (local_148 == (int *)0x0) {
        piVar7 = (int *)0x0;
      }
      else {
        piVar7 = local_148 + 0xc;
      }
      FUN_0060b9d0((void *)((int)this + 0x44),0x70f0,piVar7,piVar13,piVar11,(undefined4 *)0x0);
      if (*(int *)(param_2 + 0x28) == 2) {
        if (piVar10 == (int *)0x0) {
          piVar10 = (int *)0x0;
        }
        else {
          piVar10 = piVar10 + 0xc;
        }
        if (piVar6 == (int *)0x0) {
          piVar11 = (int *)0x0;
        }
        else {
          piVar11 = piVar6 + 0xc;
        }
        if (local_148 == (int *)0x0) {
          piVar6 = (int *)0x0;
          uVar3 = 0x70f2;
        }
        else {
          piVar6 = local_148 + 0xc;
          uVar3 = 0x70f2;
        }
        goto LAB_004936be;
      }
      if (*(int *)(param_2 + 0x28) == 3) {
        if ((*(byte *)(piVar10 + 0x14) & 8) == 0) {
          if (piVar10 == (int *)0x0) {
            piVar10 = (int *)0x0;
          }
          else {
            piVar10 = piVar10 + 0xc;
          }
          if (piVar6 == (int *)0x0) {
            piVar11 = (int *)0x0;
          }
          else {
            piVar11 = piVar6 + 0xc;
          }
          if (local_148 == (int *)0x0) {
            piVar6 = (int *)0x0;
            uVar3 = 0x70f1;
          }
          else {
            piVar6 = local_148 + 0xc;
            uVar3 = 0x70f1;
          }
        }
        else {
          if (piVar10 == (int *)0x0) {
            piVar10 = (int *)0x0;
          }
          else {
            piVar10 = piVar10 + 0xc;
          }
          if (piVar6 == (int *)0x0) {
            piVar11 = (int *)0x0;
          }
          else {
            piVar11 = piVar6 + 0xc;
          }
          if (local_148 == (int *)0x0) {
            piVar6 = (int *)0x0;
            uVar3 = 0x70f6;
          }
          else {
            piVar6 = local_148 + 0xc;
            uVar3 = 0x70f6;
          }
        }
        goto LAB_004936be;
      }
      *(undefined4 *)((int)this + 0x40) = 0;
    }
    else {
      if (piVar10 == (int *)0x0) {
        piVar11 = (int *)0x0;
      }
      else {
        piVar11 = piVar10 + 0xc;
      }
      if (piVar6 == (int *)0x0) {
        piVar13 = (int *)0x0;
      }
      else {
        piVar13 = piVar6 + 0xc;
      }
      FUN_0060b9d0((void *)((int)this + 0x44),0x70f3,piVar13,piVar11,(undefined4 *)0x0,
                   (undefined4 *)0x0);
      if (*(int *)(param_2 + 0x28) == 2) {
        if (piVar10 == (int *)0x0) {
          piVar11 = (int *)0x0;
        }
        else {
          piVar11 = piVar10 + 0xc;
        }
        if (piVar6 == (int *)0x0) {
          piVar6 = (int *)0x0;
        }
        else {
          piVar6 = piVar6 + 0xc;
        }
        piVar10 = (int *)0x0;
        uVar3 = 0x70f5;
      }
      else {
        if (*(int *)(param_2 + 0x28) != 3) {
          *(undefined4 *)((int)this + 0x40) = 0;
          goto LAB_004936c6;
        }
        if ((*(byte *)(piVar10 + 0x14) & 8) == 0) {
          if (piVar10 == (int *)0x0) {
            piVar11 = (int *)0x0;
          }
          else {
            piVar11 = piVar10 + 0xc;
          }
          if (piVar6 == (int *)0x0) {
            piVar10 = (int *)0x0;
            piVar6 = (int *)0x0;
            uVar3 = 0x70f4;
          }
          else {
            piVar6 = piVar6 + 0xc;
            piVar10 = (int *)0x0;
            uVar3 = 0x70f4;
          }
        }
        else {
          if (piVar10 == (int *)0x0) {
            piVar11 = (int *)0x0;
          }
          else {
            piVar11 = piVar10 + 0xc;
          }
          if (piVar6 == (int *)0x0) {
            piVar10 = (int *)0x0;
            piVar6 = (int *)0x0;
            uVar3 = 0x70f7;
          }
          else {
            piVar6 = piVar6 + 0xc;
            piVar10 = (int *)0x0;
            uVar3 = 0x70f7;
          }
        }
      }
LAB_004936be:
      FUN_0060b9d0((void *)((int)this + 0x50),uVar3,piVar6,piVar11,piVar10,(undefined4 *)0x0);
    }
LAB_004936c6:
    if (local_140 != (int *)0x0) {
      FUN_0060b9d0((void *)((int)this + 0x50),0x70e6,local_140 + 0xc,(undefined4 *)0x0,
                   (undefined4 *)0x0,(undefined4 *)0x0);
    }
    break;
  case 99:
    *(undefined4 *)((int)this + 0x84) = 3;
    piVar10 = FUN_004f2d10(*(int *)((int)this + 0x20),(uint *)(piVar10 + 0x1c));
    local_a4 = 0x30;
    local_a0 = 0x38;
    if ((*puVar5 >> 0x18 < 0x30) || (0x37 < *puVar5 >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (bVar1) {
      if (piVar10 == (int *)0x0) {
        piVar11 = (int *)0x0;
      }
      else {
        piVar11 = piVar10 + 0xc;
      }
      if (piVar6 == (int *)0x0) {
        piVar13 = (int *)0x0;
      }
      else {
        piVar13 = piVar6 + 0xc;
      }
      if (local_148 == (int *)0x0) {
        piVar7 = (int *)0x0;
      }
      else {
        piVar7 = local_148 + 0xc;
      }
      FUN_0060b9d0((void *)((int)this + 0x44),0x70f8,piVar7,piVar13,piVar11,(undefined4 *)0x0);
      if (*(int *)(param_2 + 0x28) == 2) {
        if ((piVar10[0x14] & 0x200U) == 0) {
          if (piVar10 == (int *)0x0) {
            piVar10 = (int *)0x0;
          }
          else {
            piVar10 = piVar10 + 0xc;
          }
          if (piVar6 == (int *)0x0) {
            piVar11 = (int *)0x0;
          }
          else {
            piVar11 = piVar6 + 0xc;
          }
          if (local_148 == (int *)0x0) {
            piVar6 = (int *)0x0;
            uVar3 = 0x70fa;
          }
          else {
            piVar6 = local_148 + 0xc;
            uVar3 = 0x70fa;
          }
        }
        else {
          if (piVar10 == (int *)0x0) {
            piVar10 = (int *)0x0;
          }
          else {
            piVar10 = piVar10 + 0xc;
          }
          if (piVar6 == (int *)0x0) {
            piVar11 = (int *)0x0;
          }
          else {
            piVar11 = piVar6 + 0xc;
          }
          if (local_148 == (int *)0x0) {
            piVar6 = (int *)0x0;
            uVar3 = 0x70ff;
          }
          else {
            piVar6 = local_148 + 0xc;
            uVar3 = 0x70ff;
          }
        }
        goto LAB_00493965;
      }
      if (*(int *)(param_2 + 0x28) == 3) {
        if ((*(byte *)(piVar10 + 0x14) & 8) != 0) {
          if (piVar10 == (int *)0x0) {
            piVar10 = (int *)0x0;
          }
          else {
            piVar10 = piVar10 + 0xc;
          }
          if (piVar6 == (int *)0x0) {
            piVar11 = (int *)0x0;
          }
          else {
            piVar11 = piVar6 + 0xc;
          }
          if (local_148 == (int *)0x0) {
            piVar6 = (int *)0x0;
            uVar3 = 0x70f9;
          }
          else {
            piVar6 = local_148 + 0xc;
            uVar3 = 0x70f9;
          }
          goto LAB_00493965;
        }
      }
      else {
        *(undefined4 *)((int)this + 0x40) = 0;
      }
    }
    else {
      if (piVar10 == (int *)0x0) {
        piVar11 = (int *)0x0;
      }
      else {
        piVar11 = piVar10 + 0xc;
      }
      if (piVar6 == (int *)0x0) {
        piVar13 = (int *)0x0;
      }
      else {
        piVar13 = piVar6 + 0xc;
      }
      FUN_0060b9d0((void *)((int)this + 0x44),0x70fb,piVar13,piVar11,(undefined4 *)0x0,
                   (undefined4 *)0x0);
      if (*(int *)(param_2 + 0x28) == 2) {
        if (piVar10 == (int *)0x0) {
          piVar11 = (int *)0x0;
        }
        else {
          piVar11 = piVar10 + 0xc;
        }
        if (piVar6 == (int *)0x0) {
          piVar6 = (int *)0x0;
        }
        else {
          piVar6 = piVar6 + 0xc;
        }
        piVar10 = (int *)0x0;
        uVar3 = 0x70fd;
      }
      else {
        if (*(int *)(param_2 + 0x28) != 3) {
          *(undefined4 *)((int)this + 0x40) = 0;
          goto LAB_0049396d;
        }
        if ((*(byte *)(piVar10 + 0x14) & 8) == 0) {
          if (piVar10 == (int *)0x0) {
            piVar11 = (int *)0x0;
          }
          else {
            piVar11 = piVar10 + 0xc;
          }
          if (piVar6 == (int *)0x0) {
            piVar10 = (int *)0x0;
            piVar6 = (int *)0x0;
            uVar3 = 0x70fe;
          }
          else {
            piVar6 = piVar6 + 0xc;
            piVar10 = (int *)0x0;
            uVar3 = 0x70fe;
          }
        }
        else {
          if (piVar10 == (int *)0x0) {
            piVar11 = (int *)0x0;
          }
          else {
            piVar11 = piVar10 + 0xc;
          }
          if (piVar6 == (int *)0x0) {
            piVar10 = (int *)0x0;
            piVar6 = (int *)0x0;
            uVar3 = 0x70fc;
          }
          else {
            piVar6 = piVar6 + 0xc;
            piVar10 = (int *)0x0;
            uVar3 = 0x70fc;
          }
        }
      }
LAB_00493965:
      FUN_0060b9d0((void *)((int)this + 0x50),uVar3,piVar6,piVar11,piVar10,(undefined4 *)0x0);
    }
LAB_0049396d:
    if (local_140 != (int *)0x0) {
      FUN_0060b9d0((void *)((int)this + 0x50),0x70e6,local_140 + 0xc,(undefined4 *)0x0,
                   (undefined4 *)0x0,(undefined4 *)0x0);
    }
    break;
  case 0x69:
    *(undefined4 *)((int)this + 0x40) = 1;
    *(undefined4 *)((int)this + 0x84) = 3;
    local_134 = FUN_004f2d10(*(int *)((int)this + 0x20),(uint *)(piVar10 + 0x1c));
    FUN_004f26d0(puVar8,piVar10 + 0x1d);
    local_c4 = 0x30;
    local_c0 = 0x38;
    if ((*puVar5 >> 0x18 < 0x30) || (0x37 < *puVar5 >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (bVar1) {
      if (piVar6 == (int *)0x0) {
        piVar10 = (int *)0x0;
      }
      else {
        piVar10 = piVar6 + 0xc;
      }
      if (local_148 == (int *)0x0) {
        piVar11 = (int *)0x0;
      }
      else {
        piVar11 = local_148 + 0xc;
      }
      FUN_0060b9d0((void *)((int)this + 0x44),0x70e0,piVar11,piVar10,(undefined4 *)0x0,
                   (undefined4 *)0x0);
      if (local_134 == (int *)0x0) {
        piVar11 = (int *)0x0;
      }
      else {
        piVar11 = local_134 + 0xc;
      }
      if (piVar6 == (int *)0x0) {
        piVar10 = (int *)0x0;
      }
      else {
        piVar10 = piVar6 + 0xc;
      }
      if (local_148 == (int *)0x0) {
        piVar6 = (int *)0x0;
        uVar3 = 0x70e3;
      }
      else {
        piVar6 = local_148 + 0xc;
        uVar3 = 0x70e3;
      }
    }
    else {
      if (piVar6 == (int *)0x0) {
        piVar10 = (int *)0x0;
      }
      else {
        piVar10 = piVar6 + 0xc;
      }
      FUN_0060b9d0((void *)((int)this + 0x44),0x70e1,piVar10,(undefined4 *)0x0,(undefined4 *)0x0,
                   (undefined4 *)0x0);
      if (local_134 == (int *)0x0) {
        piVar10 = (int *)0x0;
      }
      else {
        piVar10 = local_134 + 0xc;
      }
      if (piVar6 == (int *)0x0) {
        piVar6 = (int *)0x0;
      }
      else {
        piVar6 = piVar6 + 0xc;
      }
      piVar11 = (int *)0x0;
      uVar3 = 0x70e2;
    }
    piVar13 = local_134;
    pvVar12 = (void *)((int)this + 0x50);
    FUN_0060b9d0(pvVar12,uVar3,piVar6,piVar10,piVar11,(undefined4 *)0x0);
    if (*(int *)(param_2 + 0x28) == 2) {
      if (piVar13 == (int *)0x0) {
        piVar13 = (int *)0x0;
      }
      else {
        piVar13 = piVar13 + 0xc;
      }
      uVar3 = 0x70e5;
LAB_00493361:
      FUN_0060b9d0(pvVar12,uVar3,piVar13,(undefined4 *)0x0,(undefined4 *)0x0,(undefined4 *)0x0);
    }
    else {
      if (*(int *)(param_2 + 0x28) == 3) {
        if (piVar13 == (int *)0x0) {
          piVar13 = (int *)0x0;
          uVar3 = 0x70e4;
        }
        else {
          piVar13 = piVar13 + 0xc;
          uVar3 = 0x70e4;
        }
        goto LAB_00493361;
      }
      *(undefined4 *)((int)this + 0x40) = 0;
    }
    if (local_140 != (int *)0x0) {
      FUN_0060b9d0(pvVar12,0x70e6,local_140 + 0xc,(undefined4 *)0x0,(undefined4 *)0x0,
                   (undefined4 *)0x0);
    }
  }
  local_4._0_1_ = 1;
  FUN_00619730();
LAB_004948ce:
  puVar5 = (uint *)((int)this + 0x74);
  uVar3 = *(uint *)((int)this + 0x74) >> 0x18;
  local_24 = 0x30;
  local_20 = 0x38;
  if ((uVar3 < 0x30) || (0x37 < uVar3)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_00619730();
  if (bVar1) {
    switch(*puVar5 & 0xffffff) {
    case 0x240:
      *(undefined4 *)((int)this + 0x28) = 0x17;
      if (local_138 == 0) {
        uVar2 = FUN_004c4990(&DAT_000000f3);
        *(ushort *)((int)this + 0x32) = uVar2;
      }
      else {
        uVar2 = FUN_004c4990(&DAT_000000f2);
        *(ushort *)((int)this + 0x32) = uVar2;
      }
      break;
    case 0x241:
      *(undefined4 *)((int)this + 0x28) = 0x18;
      if (local_138 == 0) {
        uVar2 = FUN_004c4990(&DAT_000000df);
        *(ushort *)((int)this + 0x32) = uVar2;
      }
      else {
        uVar2 = FUN_004c4990(&DAT_000000de);
        *(ushort *)((int)this + 0x32) = uVar2;
      }
      break;
    case 0x242:
      uVar3 = 0;
      *(undefined4 *)((int)this + 0x28) = 0x16;
      if (local_148 != (int *)0x0) {
        uVar3 = (uint)~local_148[0x2c] >> 3 & 1;
      }
      if (local_138 == 0) {
        uVar2 = FUN_004c4990((HGLOBAL)((-(uint)(uVar3 != 0) & 0xfffffff6) + 0xd5));
        *(ushort *)((int)this + 0x32) = uVar2;
      }
      else {
        uVar2 = FUN_004c4990((HGLOBAL)((-(uint)(uVar3 != 0) & 0xfffffff6) + 0xd4));
        *(ushort *)((int)this + 0x32) = uVar2;
      }
      break;
    case 0x243:
      *(undefined4 *)((int)this + 0x28) = 0x19;
      if (local_138 == 0) {
        uVar2 = FUN_004c4990(&DAT_000000e9);
        *(ushort *)((int)this + 0x32) = uVar2;
      }
      else {
        uVar2 = FUN_004c4990(&DAT_000000e8);
        *(ushort *)((int)this + 0x32) = uVar2;
      }
      break;
    case 0x280:
      *(undefined4 *)((int)this + 0x28) = 0x1a;
      if (local_138 == 0) {
        uVar2 = FUN_004c4990(&DAT_00000107);
        *(ushort *)((int)this + 0x32) = uVar2;
      }
      else {
        uVar2 = FUN_004c4990(&DAT_00000106);
        *(ushort *)((int)this + 0x32) = uVar2;
      }
      break;
    case 0x281:
      *(undefined4 *)((int)this + 0x28) = 0x1b;
      if (local_138 == 0) {
        uVar2 = FUN_004c4990(&DAT_000000fd);
        *(ushort *)((int)this + 0x32) = uVar2;
      }
      else {
        uVar2 = FUN_004c4990(&DAT_000000fc);
        *(ushort *)((int)this + 0x32) = uVar2;
      }
    }
  }
  else {
    *(undefined4 *)((int)this + 0x28) = 0x15;
  }
  uStack_14 = 0x30;
  uStack_10 = 0x3c;
  if ((*puVar5 >> 0x18 < 0x30) || (0x3b < *puVar5 >> 0x18)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_00619730();
  if (bVar1) {
    piVar10 = FUN_004f2d10(*(int *)((int)this + 0x20),puVar5);
    if (piVar10 != (int *)0x0) {
      uVar3 = FUN_004c5000(piVar10);
      *(short *)((int)this + 0x2c) = (short)uVar3;
    }
  }
  else {
    piVar10 = FUN_004f2d10(*(int *)((int)this + 0x20),puVar5);
    if (piVar10 != (int *)0x0) {
      uVar3 = FUN_004c5000(piVar10);
      *(short *)((int)this + 0x2e) = (short)uVar3;
    }
  }
  FUN_004f26d0((void *)((int)this + 0x5c),puVar5);
LAB_00494b3a:
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_004f4380(local_12c);
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return;
}

