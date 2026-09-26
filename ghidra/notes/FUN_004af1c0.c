
void __thiscall FUN_004af1c0(void *this,uint *param_1,undefined4 *param_2,uint param_3,uint param_4)

{
  bool bVar1;
  bool bVar2;
  ushort uVar3;
  int *piVar4;
  void *pvVar5;
  uint *puVar6;
  undefined1 *puVar7;
  undefined4 *puVar8;
  void *pvVar9;
  int iVar10;
  uint **ppuVar11;
  void **ppvVar12;
  uint local_68;
  void *local_64;
  void *local_60;
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
  puStack_8 = &LAB_00638c14;
  local_c = ExceptionList;
  bVar2 = false;
  ExceptionList = &local_c;
  local_64 = this;
  FUN_005f4950(&local_68,0);
  local_4 = 0;
  FUN_004f5510(local_54);
  local_4._0_1_ = 1;
  FUN_004ece80(param_2);
  puVar6 = param_1;
  local_5c = (undefined1 *)0x90;
  local_58 = 0x98;
  if ((*param_1 >> 0x18 < 0x90) || (0x97 < *param_1 >> 0x18)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_00619730();
  if (bVar1) {
    piVar4 = FUN_004f3220(*(int *)((int)this + 4),puVar6);
    iVar10 = *(int *)((int)this + 0xc);
    param_1 = (uint *)&stack0xffffff80;
    FUN_004f26d0(&stack0xffffff80,puVar6);
    pvVar5 = FUN_00403d30((void *)(iVar10 + 0x2c));
    local_60 = pvVar5;
    if ((piVar4 != (int *)0x0) && (pvVar5 != (void *)0x0)) {
      if ((param_3 & 0x400) != 0) {
        FUN_00536da0(local_4c,piVar4,3);
        local_4._0_1_ = 2;
        FUN_00513120((int)local_4c);
        while (local_30 != 0) {
          ppuVar11 = &param_1;
          pvVar5 = (void *)FUN_0052bed0((int)local_4c);
          puVar6 = FUN_004025b0(pvVar5,(uint *)ppuVar11);
          local_4._0_1_ = 3;
          pvVar5 = FUN_004f5940((void *)(*(int *)((int)local_64 + 0xc) + 0x8c),puVar6);
          local_4._0_1_ = 2;
          FUN_00619730();
          if ((((pvVar5 != (void *)0x0) &&
               (local_68 = *(uint *)((int)pvVar5 + 0x30), (local_68 & 0x20000000) != 0)) &&
              ((local_68 & 4) != 0)) && (((param_4 & local_68) != 0 && ((local_68 & 0x20) == 0)))) {
            puVar7 = (undefined1 *)FUN_00618b70(0x1c);
            local_4._0_1_ = 4;
            local_5c = puVar7;
            if (puVar7 == (undefined1 *)0x0) {
              pvVar5 = (void *)0x0;
            }
            else {
              puVar8 = FUN_00403040(pvVar5,&param_3);
              bVar2 = true;
              local_4 = CONCAT31(local_4._1_3_,5);
              pvVar5 = FUN_004ecc70(puVar7,puVar8);
            }
            local_4 = 4;
            if (bVar2) {
              bVar2 = false;
              FUN_00619730();
            }
            local_4._0_1_ = 2;
            if (pvVar5 != (void *)0x0) {
              FUN_004f57b0(local_54,pvVar5);
            }
          }
          FUN_005130d0((int)local_4c);
          this = local_64;
          pvVar5 = local_60;
        }
        local_4._0_1_ = 1;
        FUN_00536ea0(local_4c);
      }
      bVar2 = false;
      if ((*(uint *)((int)pvVar5 + 0x2c) & 0x800) != 0) {
        FUN_004ffe70(local_2c,piVar4,1);
        local_4._0_1_ = 6;
        FUN_00513120((int)local_2c);
        while (local_10 != 0) {
          ppuVar11 = &param_1;
          pvVar5 = (void *)FUN_0052bed0((int)local_2c);
          puVar6 = FUN_004025b0(pvVar5,(uint *)ppuVar11);
          iVar10 = *(int *)((int)this + 0xc);
          local_4._0_1_ = 7;
          local_5c = &stack0xffffff80;
          FUN_004f26d0(&stack0xffffff80,puVar6);
          pvVar5 = FUN_00419e40((void *)(iVar10 + 0x78));
          local_4._0_1_ = 6;
          FUN_00619730();
          if (((pvVar5 != (void *)0x0) && ((*(uint *)((int)pvVar5 + 0x38) & 1) == 0)) &&
             ((*(uint *)((int)pvVar5 + 0x38) & 0x600000) != 0)) {
            for (pvVar5 = (void *)thunk_FUN_005f5060((int)pvVar5 + 0x48); pvVar5 != (void *)0x0;
                pvVar5 = *(void **)((int)pvVar5 + 0x10)) {
              puVar6 = FUN_00403040(pvVar5,&param_3);
              local_4._0_1_ = 8;
              piVar4 = FUN_004f2e20(*(int *)((int)this + 4),puVar6);
              local_4._0_1_ = 6;
              FUN_00619730();
              if (piVar4 != (int *)0x0) {
                FUN_00536da0(local_4c,piVar4,3);
                local_4._0_1_ = 9;
                FUN_00513120((int)local_4c);
                while (local_30 != 0) {
                  ppvVar12 = &local_60;
                  pvVar9 = (void *)FUN_0052bed0((int)local_4c);
                  puVar6 = FUN_004025b0(pvVar9,(uint *)ppvVar12);
                  local_4._0_1_ = 10;
                  pvVar9 = FUN_004f5940((void *)(*(int *)((int)this + 0xc) + 0x8c),puVar6);
                  local_4._0_1_ = 9;
                  FUN_00619730();
                  if ((((pvVar9 != (void *)0x0) &&
                       (local_68 = *(uint *)((int)pvVar9 + 0x30), (local_68 & 0x20000000) != 0)) &&
                      ((local_68 & 4) != 0)) &&
                     (((param_4 & local_68) != 0 && ((local_68 & 0x20) == 0)))) {
                    puVar7 = (undefined1 *)FUN_00618b70(0x1c);
                    local_4._0_1_ = 0xb;
                    local_5c = puVar7;
                    if (puVar7 == (undefined1 *)0x0) {
                      pvVar9 = (void *)0x0;
                    }
                    else {
                      puVar8 = FUN_00403040(pvVar9,&local_64);
                      bVar2 = true;
                      local_4 = CONCAT31(local_4._1_3_,0xc);
                      pvVar9 = FUN_004ecc70(puVar7,puVar8);
                    }
                    local_4 = 0xb;
                    if (bVar2) {
                      bVar2 = false;
                      FUN_00619730();
                    }
                    local_4._0_1_ = 9;
                    if (pvVar9 != (void *)0x0) {
                      FUN_004f57b0(local_54,pvVar9);
                    }
                  }
                  FUN_005130d0((int)local_4c);
                }
                local_4._0_1_ = 6;
                FUN_00536ea0(local_4c);
              }
            }
          }
          FUN_005130d0((int)local_2c);
        }
        local_4._0_1_ = 1;
        FUN_004fff70(local_2c);
      }
    }
    uVar3 = FUN_005f50e0((int)local_54);
    if (uVar3 != 0) {
      iVar10 = FUN_0041cd80((uint)uVar3);
      pvVar5 = (void *)thunk_FUN_005f5060((int)local_54);
      if (pvVar5 != (void *)0x0) {
        do {
          if (iVar10 < 1) break;
          pvVar5 = *(void **)((int)pvVar5 + 0x10);
          iVar10 = iVar10 + -1;
        } while (pvVar5 != (void *)0x0);
        if (pvVar5 != (void *)0x0) {
          puVar8 = FUN_00403040(pvVar5,&param_4);
          local_4._0_1_ = 0xd;
          FUN_004f26d0(param_2,puVar8);
          local_4._0_1_ = 1;
          FUN_00619730();
        }
      }
    }
  }
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_004f55c0(local_54);
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return;
}

