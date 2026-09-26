
void __fastcall FUN_00484630(int param_1)

{
  bool bVar1;
  int *piVar2;
  void *pvVar3;
  int iVar4;
  uint *puVar5;
  int *piVar6;
  int iVar7;
  uint extraout_ECX;
  uint extraout_ECX_00;
  undefined4 *puVar8;
  uint uVar9;
  uint local_84;
  uint uStack_80;
  uint uStack_7c;
  undefined1 *puStack_78;
  undefined4 uStack_74;
  undefined4 uStack_70;
  undefined4 auStack_6c [7];
  int iStack_50;
  undefined4 auStack_4c [7];
  int iStack_30;
  undefined4 auStack_2c [7];
  int iStack_10;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006346c3;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f4950(&local_84,0);
  local_4 = 0;
  if (*(int *)(param_1 + 0x1c) != 0) {
    *(undefined4 *)(param_1 + 0x1c) = 0;
    *(uint *)(param_1 + 0x24) = *(uint *)(param_1 + 0x24) & 0x1b0204ff;
    puVar8 = (undefined4 *)(param_1 + 0x44);
    for (iVar7 = 0x1e; iVar7 != 0; iVar7 = iVar7 + -1) {
      *puVar8 = 0;
      puVar8 = puVar8 + 1;
    }
    iVar7 = *(int *)(param_1 + 0x20);
    if (iVar7 == 1) {
      uVar9 = 1;
    }
    else if (iVar7 == 2) {
      uVar9 = 2;
    }
    else {
      uVar9 = 0;
    }
    piVar2 = FUN_004f2e20(iVar7,(uint *)(param_1 + 0x18));
    if ((piVar2 != (int *)0x0) && ((*(byte *)(piVar2 + 0x14) & 8) == 0)) {
      if (((uint)piVar2[9] >> 6 & 3) == uVar9) {
        *(uint *)(param_1 + 0x24) = *(uint *)(param_1 + 0x24) | 0x4000;
      }
      if ((*(byte *)(piVar2 + 0x14) & 0x10) != 0) {
        *(uint *)(param_1 + 0x24) = *(uint *)(param_1 + 0x24) | 0x800;
      }
      if ((*(byte *)(piVar2 + 0x14) & 4) == 0) {
        uVar9 = *(uint *)(param_1 + 0x24);
        *(uint *)(param_1 + 0x24) = uVar9 | 0x2000;
        *(uint *)(param_1 + 0x24) = uVar9 & 0xf7ffffff | 0x2000;
      }
      if ((piVar2[0x14] & 0x200U) != 0) {
        iVar7 = (**(code **)(*piVar2 + 0x34))();
        if (iVar7 == 0) {
          uVar9 = *(uint *)(param_1 + 0x24) | 0x800000;
        }
        else {
          uVar9 = *(uint *)(param_1 + 0x24) | 0x1000;
        }
        *(uint *)(param_1 + 0x24) = uVar9;
      }
      pvVar3 = (void *)FUN_0042e190(*(void **)(param_1 + 0x3c),(uint *)(param_1 + 0x28));
      *(undefined4 *)(param_1 + 0x44) = *(undefined4 *)((int)pvVar3 + 0x40);
      if ((*(uint *)(param_1 + 0x24) & 0x1000) == 0) {
        *(undefined4 *)(param_1 + 0x48) = *(undefined4 *)((int)pvVar3 + 0x38);
      }
      iVar7 = (**(code **)(*piVar2 + 0x294))();
      iVar4 = (**(code **)(*piVar2 + 0x268))();
      iVar7 = FUN_00473910(pvVar3,iVar4 - iVar7);
      *(int *)(param_1 + 0x50) = iVar7;
      iVar7 = (**(code **)(*piVar2 + 0x290))();
      iVar4 = (**(code **)(*piVar2 + 0x264))();
      iVar7 = FUN_004738d0(pvVar3,iVar4 - iVar7);
      *(int *)(param_1 + 0x7c) = iVar7;
      iVar7 = (**(code **)(*piVar2 + 0x290))();
      iVar4 = (**(code **)(*piVar2 + 0x264))();
      iVar7 = FUN_004738d0(pvVar3,iVar4 - iVar7);
      *(int *)(param_1 + 0x90) = iVar7;
      if ((*(byte *)(param_1 + 0x24) & 2) != 0) {
        iVar7 = (**(code **)(*piVar2 + 0x290))();
        iVar4 = (**(code **)(*piVar2 + 0x264))();
        iVar7 = FUN_004738f0(pvVar3,iVar4 - iVar7);
        *(int *)(param_1 + 0x80) = iVar7;
        iVar7 = (**(code **)(*piVar2 + 0x290))();
        iVar4 = (**(code **)(*piVar2 + 0x264))();
        iVar7 = FUN_004738f0(pvVar3,iVar4 - iVar7);
        *(int *)(param_1 + 0x94) = iVar7;
      }
      *(undefined4 *)(param_1 + 0x60) = *(undefined4 *)((int)pvVar3 + 0x44);
      *(undefined4 *)(param_1 + 0x6c) = *(undefined4 *)((int)pvVar3 + 0x48);
      *(undefined4 *)(param_1 + 0x70) = *(undefined4 *)((int)pvVar3 + 0x20);
      *(int *)(param_1 + 0x88) = *(int *)(param_1 + 0x88) + 1;
      FUN_005039d0(auStack_6c,piVar2,3);
      local_4 = CONCAT31(local_4._1_3_,1);
      FUN_00513120((int)auStack_6c);
      while (iStack_50 != 0) {
        puVar5 = &uStack_80;
        pvVar3 = (void *)FUN_0052bed0((int)auStack_6c);
        puVar5 = FUN_00402d80(pvVar3,puVar5);
        local_4._0_1_ = 2;
        pvVar3 = (void *)FUN_0042e190(*(void **)(param_1 + 0x3c),puVar5);
        local_4 = CONCAT31(local_4._1_3_,1);
        FUN_00619730();
        *(int *)(param_1 + 0x78) = *(int *)(param_1 + 0x78) + *(int *)((int)pvVar3 + 0x20);
        *(int *)(param_1 + 0x58) = *(int *)(param_1 + 0x58) + 1;
        *(uint *)(param_1 + 0x24) = *(uint *)(param_1 + 0x24) | 0x100;
        iVar7 = FUN_0052bed0((int)auStack_6c);
        uStack_7c = *(uint *)(iVar7 + 0x50) >> 2 & 1;
        if (uStack_7c == 0) {
          *(uint *)(param_1 + 0x24) = *(uint *)(param_1 + 0x24) | 0x8000;
          uStack_7c = 0;
        }
        else {
          piVar6 = (int *)FUN_0052bed0((int)auStack_6c);
          uStack_7c = (**(code **)(*piVar6 + 0x254))();
          iVar7 = (**(code **)(*piVar6 + 0x244))();
          iVar7 = FUN_00473610(pvVar3,iVar7 - uStack_7c);
          *(int *)(param_1 + 0x90) = *(int *)(param_1 + 0x90) + iVar7;
          piVar6 = (int *)FUN_0052bed0((int)auStack_6c);
          uStack_7c = (**(code **)(*piVar6 + 0x254))();
          iVar7 = (**(code **)(*piVar6 + 0x244))();
          iVar7 = FUN_00473620(pvVar3,iVar7 - uStack_7c);
          *(int *)(param_1 + 0x94) = *(int *)(param_1 + 0x94) + iVar7;
          iVar7 = FUN_0052bed0((int)auStack_6c);
          uStack_7c = *(uint *)(iVar7 + 0x50) >> 5 & 1;
          if (uStack_7c == 0) {
            piVar6 = (int *)FUN_0052bed0((int)auStack_6c);
            uStack_7c = (**(code **)(*piVar6 + 0x254))();
            iVar7 = (**(code **)(*piVar6 + 0x244))();
            iVar7 = FUN_00473630(pvVar3,iVar7 - uStack_7c);
            *(int *)(param_1 + 0x84) = *(int *)(param_1 + 0x84) + iVar7;
            piVar6 = (int *)FUN_0052bed0((int)auStack_6c);
            uStack_7c = (**(code **)(*piVar6 + 0x254))();
            iVar7 = (**(code **)(*piVar6 + 0x244))();
            iVar7 = FUN_00473610(pvVar3,iVar7 - uStack_7c);
            *(int *)(param_1 + 0x7c) = *(int *)(param_1 + 0x7c) + iVar7;
            piVar6 = (int *)FUN_0052bed0((int)auStack_6c);
            uStack_7c = (**(code **)(*piVar6 + 0x254))();
            iVar7 = (**(code **)(*piVar6 + 0x244))();
            iVar7 = FUN_00473620(pvVar3,iVar7 - uStack_7c);
            *(int *)(param_1 + 0x80) = *(int *)(param_1 + 0x80) + iVar7;
            *(int *)(param_1 + 0x48) = *(int *)(param_1 + 0x48) + *(int *)((int)pvVar3 + 0x40);
            *(int *)(param_1 + 0x88) = *(int *)(param_1 + 0x88) + 1;
            *(int *)(param_1 + 0x5c) = *(int *)(param_1 + 0x5c) + 1;
          }
        }
        iVar7 = FUN_0052bed0((int)auStack_6c);
        uStack_7c = *(uint *)(iVar7 + 0x50) >> 5 & 1;
        if (uStack_7c != 0) {
          *(uint *)(param_1 + 0x24) = *(uint *)(param_1 + 0x24) | 0x10000;
        }
        FUN_005130d0((int)auStack_6c);
      }
      FUN_00504c40(auStack_2c,piVar2,3);
      local_4 = CONCAT31(local_4._1_3_,3);
      FUN_00513120((int)auStack_2c);
      while (iStack_10 != 0) {
        puVar5 = &uStack_7c;
        pvVar3 = (void *)FUN_0052bed0((int)auStack_2c);
        puVar5 = FUN_00402d80(pvVar3,puVar5);
        local_4._0_1_ = 4;
        iVar7 = FUN_0042e190(*(void **)(param_1 + 0x3c),puVar5);
        local_4 = CONCAT31(local_4._1_3_,3);
        FUN_00619730();
        *(int *)(param_1 + 0x74) = *(int *)(param_1 + 0x74) + *(int *)(iVar7 + 0x20);
        *(uint *)(param_1 + 0x24) = *(uint *)(param_1 + 0x24) | 0x200;
        *(int *)(param_1 + 100) = *(int *)(param_1 + 100) + 1;
        iVar4 = FUN_0052bed0((int)auStack_2c);
        uStack_80 = *(uint *)(iVar4 + 0x50) >> 2 & 1;
        if (uStack_80 == 0) {
LAB_00484b55:
          *(uint *)(param_1 + 0x24) = *(uint *)(param_1 + 0x24) | 0x8000;
        }
        else {
          iVar4 = FUN_0052bed0((int)auStack_2c);
          uStack_80 = *(uint *)(iVar4 + 0x50) >> 5 & 1;
          if (uStack_80 != 0) goto LAB_00484b55;
          *(int *)(param_1 + 0x4c) = *(int *)(param_1 + 0x4c) + *(int *)(iVar7 + 0x30);
          *(int *)(param_1 + 0x88) = *(int *)(param_1 + 0x88) + 1;
          *(int *)(param_1 + 0x68) = *(int *)(param_1 + 0x68) + 1;
        }
        iVar7 = FUN_0052bed0((int)auStack_2c);
        uStack_80 = *(uint *)(iVar7 + 0x50) >> 5 & 1;
        if (uStack_80 != 0) {
          *(uint *)(param_1 + 0x24) = *(uint *)(param_1 + 0x24) | 0x10000;
        }
        FUN_005130d0((int)auStack_2c);
      }
      FUN_00536da0(auStack_4c,piVar2,3);
      local_4._0_1_ = 5;
      FUN_00513120((int)auStack_4c);
      while (iStack_30 != 0) {
        puVar5 = &uStack_7c;
        pvVar3 = (void *)FUN_0052bed0((int)auStack_4c);
        puVar5 = FUN_004025b0(pvVar3,puVar5);
        uStack_74 = 0x30;
        uStack_70 = 0x3c;
        local_4._0_1_ = 6;
        if ((*puVar5 >> 0x18 < 0x30) || (0x3b < *puVar5 >> 0x18)) {
          bVar1 = false;
        }
        else {
          bVar1 = true;
        }
        FUN_00619730();
        local_4._0_1_ = 5;
        FUN_00619730();
        if (bVar1) {
          puVar5 = &uStack_80;
          pvVar3 = (void *)FUN_0052bed0((int)auStack_4c);
          puVar5 = FUN_004025b0(pvVar3,puVar5);
          local_4._0_1_ = 7;
          pvVar3 = FUN_004f5940((void *)(*(int *)(param_1 + 0x40) + 0x8c),puVar5);
          local_4._0_1_ = 5;
          FUN_00619730();
          if (pvVar3 != (void *)0x0) {
            local_84 = *(uint *)((int)pvVar3 + 0x30);
            if ((local_84 & 0x2000000) != 0) {
              *(uint *)(param_1 + 0x24) = *(uint *)(param_1 + 0x24) | 0x20000000;
            }
            if ((local_84 & 4) == 0) {
              if ((local_84 & 0x40) == 0) {
                uVar9 = *(uint *)(param_1 + 0x24) | 0x400000;
              }
              else {
                uVar9 = *(uint *)(param_1 + 0x24) | 0x80000;
              }
            }
            else if ((local_84 & 0x40) == 0) {
              puStack_78 = &stack0xffffff68;
              puVar5 = (uint *)&stack0xffffff68;
              uVar9 = extraout_ECX;
              pvVar3 = (void *)thunk_FUN_0052bed0((int)auStack_4c);
              FUN_004025b0(pvVar3,puVar5);
              iVar7 = FUN_004025f0(uVar9);
              if (iVar7 == 0) {
                if ((local_84 & 0x20) == 0) {
                  *(uint *)(param_1 + 0x24) = *(uint *)(param_1 + 0x24) | 0x100000;
                }
              }
              else {
                if ((local_84 & 0x20) == 0) {
                  *(uint *)(param_1 + 0x24) = *(uint *)(param_1 + 0x24) | 0x40000;
                }
                uVar9 = FUN_005f4960((undefined4 *)(*(int *)(param_1 + 0x40) + 4));
                if ((uVar9 & 1) == 0) {
                  puStack_78 = &stack0xffffff68;
                  puVar5 = (uint *)&stack0xffffff68;
                  uVar9 = extraout_ECX_00;
                  pvVar3 = (void *)thunk_FUN_0052bed0((int)auStack_4c);
                  FUN_004025b0(pvVar3,puVar5);
                  iVar7 = FUN_004024d0(uVar9);
                  if (iVar7 != 0) {
                    *(int *)(param_1 + 0x8c) = *(int *)(param_1 + 0x8c) + 1;
                  }
                }
              }
              if ((local_84 & 0x7000000) == 0) goto LAB_00484d3d;
              uVar9 = *(uint *)(param_1 + 0x24) | 0x4000000;
            }
            else {
              uVar9 = *(uint *)(param_1 + 0x24) | 0x200000;
            }
            *(uint *)(param_1 + 0x24) = uVar9;
          }
LAB_00484d3d:
          iVar7 = FUN_0052bed0((int)auStack_4c);
          puStack_78 = (undefined1 *)(*(uint *)(iVar7 + 0x50) >> 5 & 1);
          if (puStack_78 != (undefined1 *)0x0) {
            *(uint *)(param_1 + 0x24) = *(uint *)(param_1 + 0x24) | 0x10000;
          }
        }
        FUN_005130d0((int)auStack_4c);
      }
      local_4._0_1_ = 3;
      FUN_00536ea0(auStack_4c);
      local_4._0_1_ = 1;
      FUN_00504d40(auStack_2c);
      local_4 = (uint)local_4._1_3_ << 8;
      FUN_00503ad0(auStack_6c);
    }
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return;
}

