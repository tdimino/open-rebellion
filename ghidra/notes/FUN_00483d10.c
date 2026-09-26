
void __fastcall FUN_00483d10(int param_1)

{
  bool bVar1;
  bool bVar2;
  ushort uVar3;
  uint uVar4;
  int *piVar5;
  undefined1 *puVar6;
  void *pvVar7;
  uint *puVar8;
  undefined4 *puVar9;
  void *pvVar10;
  int iVar11;
  undefined1 **ppuVar12;
  int local_6c;
  uint local_68;
  undefined1 local_64 [4];
  undefined1 *local_60;
  undefined1 *local_5c;
  undefined4 local_58;
  undefined4 local_54 [2];
  undefined4 local_4c [7];
  int local_30;
  undefined4 local_2c [7];
  int local_10;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006345bc;
  local_c = ExceptionList;
  bVar2 = false;
  ExceptionList = &local_c;
  local_6c = param_1;
  FUN_004f5510(local_54);
  uVar4 = *(uint *)(param_1 + 0x34) >> 0x18;
  local_4 = 0;
  local_5c = (undefined1 *)0x90;
  local_58 = 0x98;
  if ((uVar4 < 0x90) || (0x97 < uVar4)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_00619730();
  if (bVar1) {
    piVar5 = FUN_004f3220(*(int *)(param_1 + 0x24),(uint *)(param_1 + 0x34));
    iVar11 = *(int *)(param_1 + 0x68);
    local_60 = &stack0xffffff7c;
    FUN_004f26d0(&stack0xffffff7c,(uint *)(param_1 + 0x34));
    puVar6 = FUN_00403d30((void *)(iVar11 + 0x2c));
    local_60 = puVar6;
    if ((piVar5 != (int *)0x0) && (puVar6 != (undefined1 *)0x0)) {
      if ((*(uint *)(puVar6 + 0x2c) & 0x1000) != 0) {
        FUN_004f25c0(local_4c,piVar5,1);
        local_4._0_1_ = 1;
        FUN_00513120((int)local_4c);
        while (local_30 != 0) {
          puVar8 = &local_68;
          pvVar7 = (void *)FUN_0052bed0((int)local_4c);
          puVar8 = FUN_004025b0(pvVar7,puVar8);
          local_4._0_1_ = 2;
          pvVar7 = FUN_004f5940(*(void **)(local_6c + 100),puVar8);
          local_4._0_1_ = 1;
          FUN_00619730();
          if (((pvVar7 != (void *)0x0) && ((*(uint *)((int)pvVar7 + 0x30) & 8) != 0)) &&
             ((*(uint *)((int)pvVar7 + 0x30) & 4) != 0)) {
            puVar6 = (undefined1 *)FUN_00618b70(0x1c);
            local_4._0_1_ = 3;
            local_5c = puVar6;
            if (puVar6 == (undefined1 *)0x0) {
              pvVar7 = (void *)0x0;
            }
            else {
              puVar9 = FUN_00403040(pvVar7,local_64);
              bVar2 = true;
              local_4 = CONCAT31(local_4._1_3_,4);
              pvVar7 = FUN_004ecc70(puVar6,puVar9);
            }
            local_4 = 3;
            if (bVar2) {
              bVar2 = false;
              FUN_00619730();
            }
            local_4._0_1_ = 1;
            if (pvVar7 != (void *)0x0) {
              FUN_004f57b0(local_54,pvVar7);
            }
          }
          FUN_005130d0((int)local_4c);
          param_1 = local_6c;
          puVar6 = local_60;
        }
        local_4 = (uint)local_4._1_3_ << 8;
        FUN_004f26c0(local_4c);
      }
      bVar2 = false;
      if ((*(uint *)(puVar6 + 0x2c) & 0x2000) != 0) {
        FUN_004ffe70(local_2c,piVar5,1);
        local_4._0_1_ = 5;
        FUN_00513120((int)local_2c);
        while (local_10 != 0) {
          ppuVar12 = &local_60;
          pvVar7 = (void *)FUN_0052bed0((int)local_2c);
          puVar8 = FUN_004025b0(pvVar7,(uint *)ppuVar12);
          iVar11 = *(int *)(param_1 + 0x68);
          local_4._0_1_ = 6;
          local_5c = &stack0xffffff7c;
          FUN_004f26d0(&stack0xffffff7c,puVar8);
          pvVar7 = FUN_00419e40((void *)(iVar11 + 0x78));
          local_4._0_1_ = 5;
          FUN_00619730();
          if (((pvVar7 != (void *)0x0) && ((*(uint *)((int)pvVar7 + 0x38) & 1) == 0)) &&
             ((*(uint *)((int)pvVar7 + 0x38) & 0x200000) != 0)) {
            for (pvVar7 = (void *)thunk_FUN_005f5060((int)pvVar7 + 0x48); pvVar7 != (void *)0x0;
                pvVar7 = *(void **)((int)pvVar7 + 0x10)) {
              puVar8 = FUN_00403040(pvVar7,local_64);
              local_4._0_1_ = 7;
              piVar5 = FUN_004f2e20(*(int *)(param_1 + 0x24),puVar8);
              local_4._0_1_ = 5;
              FUN_00619730();
              if (piVar5 != (int *)0x0) {
                FUN_004f25c0(local_4c,piVar5,1);
                local_4._0_1_ = 8;
                FUN_00513120((int)local_4c);
                while (local_30 != 0) {
                  puVar8 = &local_68;
                  pvVar10 = (void *)FUN_0052bed0((int)local_4c);
                  puVar8 = FUN_004025b0(pvVar10,puVar8);
                  local_4._0_1_ = 9;
                  pvVar10 = FUN_004f5940(*(void **)(param_1 + 100),puVar8);
                  local_4._0_1_ = 8;
                  FUN_00619730();
                  if (((pvVar10 != (void *)0x0) && ((*(uint *)((int)pvVar10 + 0x30) & 8) != 0)) &&
                     ((*(uint *)((int)pvVar10 + 0x30) & 4) != 0)) {
                    puVar6 = (undefined1 *)FUN_00618b70(0x1c);
                    local_4._0_1_ = 10;
                    local_5c = puVar6;
                    if (puVar6 == (undefined1 *)0x0) {
                      pvVar10 = (void *)0x0;
                    }
                    else {
                      puVar9 = FUN_00403040(pvVar10,&local_6c);
                      bVar2 = true;
                      local_4 = CONCAT31(local_4._1_3_,0xb);
                      pvVar10 = FUN_004ecc70(puVar6,puVar9);
                    }
                    local_4 = 10;
                    if (bVar2) {
                      bVar2 = false;
                      FUN_00619730();
                    }
                    local_4._0_1_ = 8;
                    if (pvVar10 != (void *)0x0) {
                      FUN_004f57b0(local_54,pvVar10);
                    }
                  }
                  FUN_005130d0((int)local_4c);
                }
                local_4._0_1_ = 5;
                FUN_004f26c0(local_4c);
              }
            }
          }
          FUN_005130d0((int)local_2c);
        }
        local_4 = (uint)local_4._1_3_ << 8;
        FUN_004fff70(local_2c);
      }
    }
    uVar3 = FUN_005f50e0((int)local_54);
    if (uVar3 != 0) {
      iVar11 = FUN_0041cd80((uint)uVar3);
      pvVar7 = (void *)thunk_FUN_005f5060((int)local_54);
      if (pvVar7 != (void *)0x0) {
        do {
          if (iVar11 < 1) break;
          pvVar7 = *(void **)((int)pvVar7 + 0x10);
          iVar11 = iVar11 + -1;
        } while (pvVar7 != (void *)0x0);
        if (pvVar7 != (void *)0x0) {
          puVar9 = FUN_00403040(pvVar7,&local_60);
          local_4._0_1_ = 0xc;
          FUN_004f26d0((void *)(param_1 + 0x30),puVar9);
          local_4 = (uint)local_4._1_3_ << 8;
          FUN_00619730();
        }
      }
    }
  }
  local_4 = 0xffffffff;
  FUN_004f55c0(local_54);
  ExceptionList = local_c;
  return;
}

