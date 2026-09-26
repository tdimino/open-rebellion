
int * __fastcall FUN_004bfe30(int param_1)

{
  bool bVar1;
  uint uVar2;
  void *pvVar3;
  int *piVar4;
  void *this;
  uint *puVar5;
  int iVar6;
  int *piVar7;
  void *pvVar8;
  undefined1 **ppuVar9;
  undefined1 *local_44;
  int *local_40;
  void *local_3c;
  undefined4 local_38;
  undefined4 local_34 [2];
  undefined4 local_2c [7];
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063a306;
  pvStack_c = ExceptionList;
  pvVar8 = (void *)0x0;
  local_40 = (int *)0x0;
  ExceptionList = &pvStack_c;
  FUN_004f4340(local_34);
  uVar2 = *(uint *)(param_1 + 0x50) >> 0x18;
  local_4._0_1_ = 0;
  local_4._1_3_ = 0;
  local_3c = (void *)0x90;
  local_38 = 0x98;
  if ((uVar2 < 0x90) || (0x97 < uVar2)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_00619730();
  piVar7 = local_40;
  if (bVar1) {
    local_44 = &stack0xffffff9c;
    iVar6 = *(int *)(param_1 + 0x2c);
    FUN_004f26d0(&stack0xffffff9c,(uint *)(param_1 + 0x50));
    pvVar3 = FUN_00403d30((void *)(iVar6 + 0x2c));
    piVar4 = FUN_004f3220(*(int *)(param_1 + 0x30),(uint *)(param_1 + 0x50));
    piVar7 = local_40;
    if ((pvVar3 != (void *)0x0) && (piVar4 != (int *)0x0)) {
      if (*(int *)((int)pvVar3 + 0x84) == 0) {
        if ((*(int *)(*(int *)(param_1 + 0x2c) + 0x1d0) < *(int *)(*(int *)(param_1 + 0x2c) + 0x1d4)
            ) || (*(int *)((int)pvVar3 + 0x50) == 0)) {
          FUN_0052d1f0(local_2c,piVar4,1);
          local_4._0_1_ = 1;
          FUN_00513120((int)local_2c);
          if (local_10 != 0) {
            this = (void *)FUN_00618b70(0x20);
            local_4._0_1_ = 2;
            local_3c = this;
            if (this == (void *)0x0) {
              pvVar8 = (void *)0x0;
            }
            else {
              ppuVar9 = &local_44;
              pvVar8 = (void *)FUN_0052bed0((int)local_2c);
              puVar5 = FUN_004025b0(pvVar8,(uint *)ppuVar9);
              local_4 = CONCAT31(local_4._1_3_,3);
              pvVar8 = FUN_004f5b10(this,puVar5,0);
            }
            local_4 = 2;
            if (this != (void *)0x0) {
              FUN_00619730();
            }
            local_4._0_1_ = 1;
            if (pvVar8 != (void *)0x0) {
              FUN_004f57b0(local_34,pvVar8);
            }
          }
          local_4._0_1_ = 0;
          FUN_0052d2f0(local_2c);
        }
        if (pvVar8 == (void *)0x0) {
          pvVar8 = (void *)0x0;
          if ((*(int *)(*(int *)(param_1 + 0x2c) + 0x1d4) <=
               *(int *)(*(int *)(param_1 + 0x2c) + 0x1d0)) || (*(int *)((int)pvVar3 + 0x54) == 0)) {
            FUN_0052d610(local_2c,piVar4,1);
            local_4._0_1_ = 4;
            FUN_00513120((int)local_2c);
            if (local_10 != 0) {
              pvVar3 = (void *)FUN_00618b70(0x20);
              local_4._0_1_ = 5;
              local_3c = pvVar3;
              if (pvVar3 == (void *)0x0) {
                pvVar8 = (void *)0x0;
              }
              else {
                ppuVar9 = &local_44;
                pvVar8 = (void *)FUN_0052bed0((int)local_2c);
                puVar5 = FUN_004025b0(pvVar8,(uint *)ppuVar9);
                local_4 = CONCAT31(local_4._1_3_,6);
                pvVar8 = FUN_004f5b10(pvVar3,puVar5,0);
              }
              local_4 = 5;
              if (pvVar3 != (void *)0x0) {
                FUN_00619730();
              }
              local_4._0_1_ = 4;
              if (pvVar8 != (void *)0x0) {
                FUN_004f57b0(local_34,pvVar8);
              }
            }
            local_4._0_1_ = 0;
            FUN_0052d710(local_2c);
          }
          if (pvVar8 == (void *)0x0) {
            FUN_00527550(local_2c,piVar4,1);
            local_4._0_1_ = 7;
            FUN_00513120((int)local_2c);
            pvVar8 = (void *)0x0;
            if (local_10 != 0) {
              pvVar3 = (void *)FUN_00618b70(0x20);
              local_4._0_1_ = 8;
              local_3c = pvVar3;
              if (pvVar3 == (void *)0x0) {
                pvVar8 = (void *)0x0;
              }
              else {
                ppuVar9 = &local_44;
                pvVar8 = (void *)FUN_0052bed0((int)local_2c);
                puVar5 = FUN_004025b0(pvVar8,(uint *)ppuVar9);
                local_4 = CONCAT31(local_4._1_3_,9);
                pvVar8 = FUN_004f5b10(pvVar3,puVar5,0);
              }
              local_4 = 8;
              if (pvVar3 != (void *)0x0) {
                FUN_00619730();
              }
              local_4._0_1_ = 7;
              if (pvVar8 != (void *)0x0) {
                FUN_004f57b0(local_34,pvVar8);
              }
            }
            local_4._0_1_ = 0;
            FUN_00527650(local_2c);
            if (pvVar8 == (void *)0x0) {
              FUN_005272e0(local_2c,piVar4,1);
              local_4._0_1_ = 10;
              FUN_00513120((int)local_2c);
              pvVar8 = (void *)0x0;
              if (local_10 != 0) {
                pvVar3 = (void *)FUN_00618b70(0x20);
                local_4._0_1_ = 0xb;
                local_3c = pvVar3;
                if (pvVar3 == (void *)0x0) {
                  pvVar8 = (void *)0x0;
                }
                else {
                  ppuVar9 = &local_44;
                  pvVar8 = (void *)FUN_0052bed0((int)local_2c);
                  puVar5 = FUN_004025b0(pvVar8,(uint *)ppuVar9);
                  local_4 = CONCAT31(local_4._1_3_,0xc);
                  pvVar8 = FUN_004f5b10(pvVar3,puVar5,0);
                }
                local_4 = 0xb;
                if (pvVar3 != (void *)0x0) {
                  FUN_00619730();
                }
                local_4._0_1_ = 10;
                if (pvVar8 != (void *)0x0) {
                  FUN_004f57b0(local_34,pvVar8);
                }
              }
              local_4._0_1_ = 0;
              FUN_00527360(local_2c);
              if (pvVar8 == (void *)0x0) {
                FUN_0051c270(local_2c,piVar4,1);
                local_4._0_1_ = 0xd;
                FUN_00513120((int)local_2c);
                pvVar8 = (void *)0x0;
                if (local_10 != 0) {
                  pvVar3 = (void *)FUN_00618b70(0x20);
                  local_4._0_1_ = 0xe;
                  local_3c = pvVar3;
                  if (pvVar3 == (void *)0x0) {
                    pvVar8 = (void *)0x0;
                  }
                  else {
                    ppuVar9 = &local_44;
                    pvVar8 = (void *)FUN_0052bed0((int)local_2c);
                    puVar5 = FUN_004025b0(pvVar8,(uint *)ppuVar9);
                    local_4 = CONCAT31(local_4._1_3_,0xf);
                    pvVar8 = FUN_004f5b10(pvVar3,puVar5,0);
                  }
                  local_4 = 0xe;
                  if (pvVar3 != (void *)0x0) {
                    FUN_00619730();
                  }
                  local_4._0_1_ = 0xd;
                  if (pvVar8 != (void *)0x0) {
                    FUN_004f57b0(local_34,pvVar8);
                  }
                }
                local_4._0_1_ = 0;
                FUN_0051c2f0(local_2c);
                if (pvVar8 == (void *)0x0) {
                  FUN_005278c0(local_2c,piVar4,1);
                  local_4._0_1_ = 0x10;
                  FUN_00513120((int)local_2c);
                  pvVar8 = (void *)0x0;
                  if (local_10 != 0) {
                    pvVar3 = (void *)FUN_00618b70(0x20);
                    local_4._0_1_ = 0x11;
                    local_3c = pvVar3;
                    if (pvVar3 == (void *)0x0) {
                      pvVar8 = (void *)0x0;
                    }
                    else {
                      ppuVar9 = &local_44;
                      pvVar8 = (void *)FUN_0052bed0((int)local_2c);
                      puVar5 = FUN_004025b0(pvVar8,(uint *)ppuVar9);
                      local_4 = CONCAT31(local_4._1_3_,0x12);
                      pvVar8 = FUN_004f5b10(pvVar3,puVar5,0);
                    }
                    local_4 = 0x11;
                    if (pvVar3 != (void *)0x0) {
                      FUN_00619730();
                    }
                    local_4._0_1_ = 0x10;
                    if (pvVar8 != (void *)0x0) {
                      FUN_004f57b0(local_34,pvVar8);
                    }
                  }
                  local_4._0_1_ = 0;
                  FUN_005279c0(local_2c);
                  if (pvVar8 == (void *)0x0) {
                    FUN_0052c740(local_2c,piVar4,1);
                    local_4._0_1_ = 0x13;
                    FUN_00513120((int)local_2c);
                    pvVar8 = (void *)0x0;
                    if (local_10 != 0) {
                      pvVar3 = (void *)FUN_00618b70(0x20);
                      local_4._0_1_ = 0x14;
                      local_3c = pvVar3;
                      if (pvVar3 == (void *)0x0) {
                        pvVar8 = (void *)0x0;
                      }
                      else {
                        ppuVar9 = &local_44;
                        pvVar8 = (void *)FUN_0052bed0((int)local_2c);
                        puVar5 = FUN_004025b0(pvVar8,(uint *)ppuVar9);
                        local_4 = CONCAT31(local_4._1_3_,0x15);
                        pvVar8 = FUN_004f5b10(pvVar3,puVar5,0);
                      }
                      local_4 = 0x14;
                      if (pvVar3 != (void *)0x0) {
                        FUN_00619730();
                      }
                      local_4._0_1_ = 0x13;
                      if (pvVar8 != (void *)0x0) {
                        FUN_004f57b0(local_34,pvVar8);
                      }
                    }
                    local_4._0_1_ = 0;
                    FUN_0052c840(local_2c);
                    if (pvVar8 == (void *)0x0) {
                      FUN_0052cab0(local_2c,piVar4,1);
                      local_4._0_1_ = 0x16;
                      FUN_00513120((int)local_2c);
                      pvVar8 = (void *)0x0;
                      if (local_10 != 0) {
                        pvVar3 = (void *)FUN_00618b70(0x20);
                        local_4._0_1_ = 0x17;
                        local_3c = pvVar3;
                        if (pvVar3 == (void *)0x0) {
                          pvVar8 = (void *)0x0;
                        }
                        else {
                          ppuVar9 = &local_44;
                          pvVar8 = (void *)FUN_0052bed0((int)local_2c);
                          puVar5 = FUN_004025b0(pvVar8,(uint *)ppuVar9);
                          local_4 = CONCAT31(local_4._1_3_,0x18);
                          pvVar8 = FUN_004f5b10(pvVar3,puVar5,0);
                        }
                        local_4 = 0x17;
                        if (pvVar3 != (void *)0x0) {
                          FUN_00619730();
                        }
                        local_4._0_1_ = 0x16;
                        if (pvVar8 != (void *)0x0) {
                          FUN_004f57b0(local_34,pvVar8);
                        }
                      }
                      local_4._0_1_ = 0;
                      FUN_0052cbb0(local_2c);
                      if (pvVar8 == (void *)0x0) {
                        FUN_0052c420(local_2c,piVar4,1);
                        local_4._0_1_ = 0x19;
                        FUN_00513120((int)local_2c);
                        if (local_10 != 0) {
                          pvVar8 = (void *)FUN_00618b70(0x20);
                          local_4._0_1_ = 0x1a;
                          local_3c = pvVar8;
                          if (pvVar8 == (void *)0x0) {
                            pvVar3 = (void *)0x0;
                          }
                          else {
                            ppuVar9 = &local_44;
                            pvVar3 = (void *)FUN_0052bed0((int)local_2c);
                            puVar5 = FUN_004025b0(pvVar3,(uint *)ppuVar9);
                            local_4 = CONCAT31(local_4._1_3_,0x1b);
                            pvVar3 = FUN_004f5b10(pvVar8,puVar5,0);
                          }
                          local_4 = 0x1a;
                          if (pvVar8 != (void *)0x0) {
                            FUN_00619730();
                          }
                          local_4._0_1_ = 0x19;
                          if (pvVar3 != (void *)0x0) {
                            FUN_004f57b0(local_34,pvVar3);
                          }
                        }
                        local_4._0_1_ = 0;
                        FUN_0052c520(local_2c);
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
      iVar6 = thunk_FUN_005f5060((int)local_34);
      piVar7 = local_40;
      if ((iVar6 != 0) && (piVar7 = (int *)FUN_004f5cd0(0x200), piVar7 != (int *)0x0)) {
        piVar7[8] = *(int *)(param_1 + 0x30);
        (**(code **)(*piVar7 + 0x24))();
      }
    }
  }
  local_4 = 0xffffffff;
  FUN_004f4380(local_34);
  ExceptionList = pvStack_c;
  return piVar7;
}

