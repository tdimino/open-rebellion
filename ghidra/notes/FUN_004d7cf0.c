
int * __fastcall FUN_004d7cf0(int param_1)

{
  bool bVar1;
  bool bVar2;
  uint uVar3;
  void *pvVar4;
  int *piVar5;
  void *pvVar6;
  uint *puVar7;
  int iVar8;
  void *pvVar9;
  undefined1 **ppuVar10;
  undefined1 *local_44;
  int *local_40;
  void *local_3c;
  undefined4 local_38;
  undefined4 local_34 [2];
  undefined4 local_2c [7];
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063cb5a;
  pvStack_c = ExceptionList;
  pvVar9 = (void *)0x0;
  bVar2 = false;
  local_40 = (int *)0x0;
  ExceptionList = &pvStack_c;
  FUN_004f4340(local_34);
  uVar3 = *(uint *)(param_1 + 0x54) >> 0x18;
  local_4 = 0;
  local_3c = (void *)0x90;
  local_38 = 0x98;
  if ((uVar3 < 0x90) || (bVar1 = true, 0x97 < uVar3)) {
    bVar1 = false;
  }
  FUN_00619730();
  if (bVar1) {
    local_44 = &stack0xffffff98;
    iVar8 = *(int *)(param_1 + 0x2c);
    FUN_004f26d0(&stack0xffffff98,(uint *)(param_1 + 0x54));
    pvVar4 = FUN_00403d30((void *)(iVar8 + 0x2c));
    piVar5 = FUN_004f3220(*(int *)(param_1 + 0x30),(uint *)(param_1 + 0x54));
    if (piVar5 != (int *)0x0) {
      if ((*(uint *)((int)pvVar4 + 0x28) & 0x200000) == 0) {
        FUN_0052cab0(local_2c,piVar5,1);
        local_4 = CONCAT31(local_4._1_3_,1);
        FUN_00513120((int)local_2c);
        while (local_10 != 0) {
          pvVar9 = (void *)FUN_00618b70(0x20);
          local_4._0_1_ = 2;
          local_3c = pvVar9;
          if (pvVar9 == (void *)0x0) {
            pvVar9 = (void *)0x0;
          }
          else {
            ppuVar10 = &local_44;
            pvVar6 = (void *)FUN_0052bed0((int)local_2c);
            puVar7 = FUN_004025b0(pvVar6,(uint *)ppuVar10);
            bVar2 = true;
            local_4 = CONCAT31(local_4._1_3_,3);
            pvVar9 = FUN_004f5b10(pvVar9,puVar7,0);
          }
          local_4 = 2;
          if (bVar2) {
            bVar2 = false;
            FUN_00619730();
          }
          local_4 = CONCAT31(local_4._1_3_,1);
          if (pvVar9 != (void *)0x0) {
            FUN_004f57b0(local_34,pvVar9);
          }
          FUN_005130d0((int)local_2c);
        }
        local_4 = local_4 & 0xffffff00;
        FUN_0052cbb0(local_2c);
      }
      bVar2 = false;
      if ((*(uint *)((int)pvVar4 + 0x28) & 0x400000) == 0) {
        FUN_0052c740(local_2c,piVar5,1);
        local_4 = CONCAT31(local_4._1_3_,4);
        FUN_00513120((int)local_2c);
        while (local_10 != 0) {
          pvVar9 = (void *)FUN_00618b70(0x20);
          local_4._0_1_ = 5;
          local_3c = pvVar9;
          if (pvVar9 == (void *)0x0) {
            pvVar9 = (void *)0x0;
          }
          else {
            ppuVar10 = &local_44;
            pvVar6 = (void *)FUN_0052bed0((int)local_2c);
            puVar7 = FUN_004025b0(pvVar6,(uint *)ppuVar10);
            bVar2 = true;
            local_4 = CONCAT31(local_4._1_3_,6);
            pvVar9 = FUN_004f5b10(pvVar9,puVar7,0);
          }
          local_4 = 5;
          if (bVar2) {
            bVar2 = false;
            FUN_00619730();
          }
          local_4 = CONCAT31(local_4._1_3_,4);
          if (pvVar9 != (void *)0x0) {
            FUN_004f57b0(local_34,pvVar9);
          }
          FUN_005130d0((int)local_2c);
        }
        local_4 = local_4 & 0xffffff00;
        FUN_0052c840(local_2c);
      }
      bVar2 = false;
      if ((*(uint *)((int)pvVar4 + 0x28) & 0x3800000) == 0) {
        FUN_0052c420(local_2c,piVar5,1);
        local_4 = CONCAT31(local_4._1_3_,7);
        FUN_00513120((int)local_2c);
        while (local_10 != 0) {
          pvVar9 = (void *)FUN_00618b70(0x20);
          local_4._0_1_ = 8;
          local_3c = pvVar9;
          if (pvVar9 == (void *)0x0) {
            pvVar9 = (void *)0x0;
          }
          else {
            ppuVar10 = &local_44;
            pvVar6 = (void *)FUN_0052bed0((int)local_2c);
            puVar7 = FUN_004025b0(pvVar6,(uint *)ppuVar10);
            bVar2 = true;
            local_4 = CONCAT31(local_4._1_3_,9);
            pvVar9 = FUN_004f5b10(pvVar9,puVar7,0);
          }
          local_4 = 8;
          if (bVar2) {
            bVar2 = false;
            FUN_00619730();
          }
          local_4 = CONCAT31(local_4._1_3_,7);
          if (pvVar9 != (void *)0x0) {
            FUN_004f57b0(local_34,pvVar9);
          }
          FUN_005130d0((int)local_2c);
        }
        local_4 = local_4 & 0xffffff00;
        FUN_0052c520(local_2c);
      }
      iVar8 = thunk_FUN_005f5060((int)local_34);
      if ((iVar8 == 0) && (*(int *)((int)pvVar4 + 0x84) == 0)) {
        if ((*(int *)(*(int *)(param_1 + 0x2c) + 0x1d0) < *(int *)(*(int *)(param_1 + 0x2c) + 0x1d4)
            ) || (*(int *)((int)pvVar4 + 0x50) == 0)) {
          FUN_0052d1f0(local_2c,piVar5,1);
          local_4._0_1_ = 10;
          FUN_00513120((int)local_2c);
          if (local_10 != 0) {
            pvVar6 = (void *)FUN_00618b70(0x20);
            local_4._0_1_ = 0xb;
            local_3c = pvVar6;
            if (pvVar6 == (void *)0x0) {
              pvVar9 = (void *)0x0;
            }
            else {
              ppuVar10 = &local_44;
              pvVar9 = (void *)FUN_0052bed0((int)local_2c);
              puVar7 = FUN_004025b0(pvVar9,(uint *)ppuVar10);
              local_4 = CONCAT31(local_4._1_3_,0xc);
              pvVar9 = FUN_004f5b10(pvVar6,puVar7,0);
            }
            local_4 = 0xb;
            if (pvVar6 != (void *)0x0) {
              FUN_00619730();
            }
            local_4._0_1_ = 10;
            if (pvVar9 != (void *)0x0) {
              FUN_004f57b0(local_34,pvVar9);
            }
          }
          local_4 = (uint)local_4._1_3_ << 8;
          FUN_0052d2f0(local_2c);
        }
        if (pvVar9 == (void *)0x0) {
          pvVar9 = (void *)0x0;
          if ((*(int *)(*(int *)(param_1 + 0x2c) + 0x1d4) <=
               *(int *)(*(int *)(param_1 + 0x2c) + 0x1d0)) || (*(int *)((int)pvVar4 + 0x54) == 0)) {
            FUN_0052d610(local_2c,piVar5,1);
            local_4._0_1_ = 0xd;
            FUN_00513120((int)local_2c);
            if (local_10 != 0) {
              pvVar4 = (void *)FUN_00618b70(0x20);
              local_4._0_1_ = 0xe;
              local_3c = pvVar4;
              if (pvVar4 == (void *)0x0) {
                pvVar9 = (void *)0x0;
              }
              else {
                ppuVar10 = &local_44;
                pvVar9 = (void *)FUN_0052bed0((int)local_2c);
                puVar7 = FUN_004025b0(pvVar9,(uint *)ppuVar10);
                local_4 = CONCAT31(local_4._1_3_,0xf);
                pvVar9 = FUN_004f5b10(pvVar4,puVar7,0);
              }
              local_4 = 0xe;
              if (pvVar4 != (void *)0x0) {
                FUN_00619730();
              }
              local_4._0_1_ = 0xd;
              if (pvVar9 != (void *)0x0) {
                FUN_004f57b0(local_34,pvVar9);
              }
            }
            local_4 = (uint)local_4._1_3_ << 8;
            FUN_0052d710(local_2c);
          }
          if (pvVar9 == (void *)0x0) {
            FUN_0052cab0(local_2c,piVar5,1);
            local_4._0_1_ = 0x10;
            FUN_00513120((int)local_2c);
            pvVar9 = (void *)0x0;
            if (local_10 != 0) {
              pvVar4 = (void *)FUN_00618b70(0x20);
              local_4._0_1_ = 0x11;
              local_3c = pvVar4;
              if (pvVar4 == (void *)0x0) {
                pvVar9 = (void *)0x0;
              }
              else {
                ppuVar10 = &local_44;
                pvVar9 = (void *)FUN_0052bed0((int)local_2c);
                puVar7 = FUN_004025b0(pvVar9,(uint *)ppuVar10);
                local_4 = CONCAT31(local_4._1_3_,0x12);
                pvVar9 = FUN_004f5b10(pvVar4,puVar7,0);
              }
              local_4 = 0x11;
              if (pvVar4 != (void *)0x0) {
                FUN_00619730();
              }
              local_4._0_1_ = 0x10;
              if (pvVar9 != (void *)0x0) {
                FUN_004f57b0(local_34,pvVar9);
              }
            }
            local_4 = (uint)local_4._1_3_ << 8;
            FUN_0052cbb0(local_2c);
            if (pvVar9 == (void *)0x0) {
              FUN_0052c740(local_2c,piVar5,1);
              local_4._0_1_ = 0x13;
              FUN_00513120((int)local_2c);
              pvVar9 = (void *)0x0;
              if (local_10 != 0) {
                pvVar4 = (void *)FUN_00618b70(0x20);
                local_4._0_1_ = 0x14;
                local_3c = pvVar4;
                if (pvVar4 == (void *)0x0) {
                  pvVar9 = (void *)0x0;
                }
                else {
                  ppuVar10 = &local_44;
                  pvVar9 = (void *)FUN_0052bed0((int)local_2c);
                  puVar7 = FUN_004025b0(pvVar9,(uint *)ppuVar10);
                  local_4 = CONCAT31(local_4._1_3_,0x15);
                  pvVar9 = FUN_004f5b10(pvVar4,puVar7,0);
                }
                local_4 = 0x14;
                if (pvVar4 != (void *)0x0) {
                  FUN_00619730();
                }
                local_4._0_1_ = 0x13;
                if (pvVar9 != (void *)0x0) {
                  FUN_004f57b0(local_34,pvVar9);
                }
              }
              local_4 = (uint)local_4._1_3_ << 8;
              FUN_0052c840(local_2c);
              if (pvVar9 == (void *)0x0) {
                FUN_0052c420(local_2c,piVar5,1);
                local_4._0_1_ = 0x16;
                FUN_00513120((int)local_2c);
                if (local_10 != 0) {
                  pvVar9 = (void *)FUN_00618b70(0x20);
                  local_4._0_1_ = 0x17;
                  local_3c = pvVar9;
                  if (pvVar9 == (void *)0x0) {
                    pvVar4 = (void *)0x0;
                  }
                  else {
                    ppuVar10 = &local_44;
                    pvVar4 = (void *)FUN_0052bed0((int)local_2c);
                    puVar7 = FUN_004025b0(pvVar4,(uint *)ppuVar10);
                    local_4 = CONCAT31(local_4._1_3_,0x18);
                    pvVar4 = FUN_004f5b10(pvVar9,puVar7,0);
                  }
                  local_4 = 0x17;
                  if (pvVar9 != (void *)0x0) {
                    FUN_00619730();
                  }
                  local_4._0_1_ = 0x16;
                  if (pvVar4 != (void *)0x0) {
                    FUN_004f57b0(local_34,pvVar4);
                  }
                }
                local_4 = (uint)local_4._1_3_ << 8;
                FUN_0052c520(local_2c);
              }
            }
          }
        }
      }
    }
  }
  iVar8 = thunk_FUN_005f5060((int)local_34);
  piVar5 = local_40;
  if ((iVar8 != 0) && (piVar5 = (int *)FUN_004f5cd0(0x200), piVar5 != (int *)0x0)) {
    piVar5[8] = *(int *)(param_1 + 0x30);
    (**(code **)(*piVar5 + 0x24))();
  }
  local_4 = 0xffffffff;
  FUN_004f4380(local_34);
  ExceptionList = pvStack_c;
  return piVar5;
}

