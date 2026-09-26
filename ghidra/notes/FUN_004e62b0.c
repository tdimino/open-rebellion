
void __fastcall FUN_004e62b0(int param_1)

{
  bool bVar1;
  ushort uVar2;
  short sVar3;
  void *pvVar4;
  undefined4 *puVar5;
  int iVar6;
  void *pvVar7;
  uint uVar8;
  int iVar9;
  uint local_34;
  uint local_30;
  int local_2c;
  undefined1 local_28 [4];
  void *local_24;
  undefined4 local_20;
  undefined4 local_1c;
  undefined4 local_18 [3];
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063e47c;
  pvStack_c = ExceptionList;
  iVar9 = 0;
  ExceptionList = &pvStack_c;
  FUN_004ece30(&local_34);
  local_4 = 0;
  FUN_004e5110(local_18);
  local_4 = CONCAT31(local_4._1_3_,1);
  local_2c = 0;
  uVar2 = FUN_005f50e0(*(int *)(param_1 + 0x2c));
  uVar8 = (uint)uVar2;
  if (uVar8 != 0) {
    local_2c = 1;
  }
  if (*(int *)(param_1 + 0x14) == 0) {
    if (*(int *)(param_1 + 0x18) != 0) {
      uVar8 = uVar8 - 1;
    }
  }
  else {
    uVar8 = uVar8 + 1;
  }
  local_30 = uVar8;
  thunk_FUN_005f5060(*(int *)(param_1 + 0x2c));
  if (0 < (int)uVar8) {
    pvVar4 = FUN_00419a00(*(void **)(param_1 + 0xc),0x40000000,0,4,0,0,0,0x26,(void *)0x1);
    pvVar7 = (void *)(param_1 + 0x1c);
    FUN_00435790(pvVar7,(int)pvVar4);
    if (local_2c != 0) {
      pvVar4 = FUN_00419e90(*(void **)(param_1 + 0xc),0x40000000,0,4,0,0,0,&DAT_00000002);
      FUN_00435790(pvVar7,(int)pvVar4);
      pvVar4 = FUN_00419e90(*(void **)(param_1 + 0xc),0x40000000,0,4,0,0,0,&DAT_00000002);
      FUN_00435790(pvVar7,(int)pvVar4);
    }
    puVar5 = FUN_004357b0(pvVar7,local_28);
    local_4._0_1_ = 2;
    FUN_004f26d0(&local_34,puVar5);
    local_4 = CONCAT31(local_4._1_3_,1);
    FUN_00619730();
    while( true ) {
      local_20 = 0x80;
      local_1c = 0x90;
      if (((local_34 >> 0x18 < 0x80) || (0x8f < local_34 >> 0x18)) || ((int)uVar8 <= iVar9)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (!bVar1) break;
      iVar6 = FUN_004e5260(local_18,(int *)&local_34);
      if (iVar6 == 0) {
        local_24 = (void *)FUN_00618b70(0x28);
        local_4._0_1_ = 3;
        if (local_24 == (void *)0x0) {
          pvVar4 = (void *)0x0;
        }
        else {
          pvVar4 = FUN_004ecac0(local_24,&local_34,3,0);
        }
        local_4 = CONCAT31(local_4._1_3_,1);
        FUN_005f5440(local_18,pvVar4);
        iVar9 = iVar9 + 1;
      }
      puVar5 = FUN_004357b0(pvVar7,local_28);
      local_4._0_1_ = 4;
      FUN_004f26d0(&local_34,puVar5);
      local_4 = CONCAT31(local_4._1_3_,1);
      FUN_00619730();
      uVar8 = local_30;
    }
    FUN_005f58b0(pvVar7);
    uVar8 = local_30;
    if (iVar9 < (int)local_30) {
      pvVar4 = FUN_00419a00(*(void **)(param_1 + 0xc),0,0,0x16,0,0,8,0x26,(void *)0x1);
      pvVar7 = (void *)(param_1 + 0x1c);
      FUN_00435790(pvVar7,(int)pvVar4);
      if (local_2c != 0) {
        pvVar4 = FUN_00419e90(*(void **)(param_1 + 0xc),0,0,0x16,0,0,8,&DAT_00000002);
        FUN_00435790(pvVar7,(int)pvVar4);
        pvVar4 = FUN_00419e90(*(void **)(param_1 + 0xc),0,0,0x16,0,0,8,&DAT_00000002);
        FUN_00435790(pvVar7,(int)pvVar4);
      }
      puVar5 = FUN_004357b0(pvVar7,local_28);
      local_4._0_1_ = 5;
      FUN_004f26d0(&local_34,puVar5);
      local_4 = CONCAT31(local_4._1_3_,1);
      FUN_00619730();
      while( true ) {
        local_20 = 0x80;
        local_1c = 0x90;
        if (((local_34 >> 0x18 < 0x80) || (0x8f < local_34 >> 0x18)) || ((int)uVar8 <= iVar9)) {
          bVar1 = false;
        }
        else {
          bVar1 = true;
        }
        FUN_00619730();
        if (!bVar1) break;
        iVar6 = FUN_004e5260(local_18,(int *)&local_34);
        if (iVar6 == 0) {
          local_24 = (void *)FUN_00618b70(0x28);
          local_4._0_1_ = 6;
          if (local_24 == (void *)0x0) {
            pvVar4 = (void *)0x0;
          }
          else {
            pvVar4 = FUN_004ecac0(local_24,&local_34,3,0);
          }
          local_4 = CONCAT31(local_4._1_3_,1);
          FUN_005f5440(local_18,pvVar4);
          iVar9 = iVar9 + 1;
        }
        puVar5 = FUN_004357b0(pvVar7,local_28);
        local_4._0_1_ = 7;
        FUN_004f26d0(&local_34,puVar5);
        local_4 = CONCAT31(local_4._1_3_,1);
        FUN_00619730();
        uVar8 = local_30;
      }
      FUN_005f58b0(pvVar7);
      uVar8 = local_30;
      if (iVar9 < (int)local_30) {
        pvVar4 = FUN_00419a00(*(void **)(param_1 + 0xc),0,0,6,0,0,0,0x26,(void *)0x1);
        pvVar7 = (void *)(param_1 + 0x1c);
        FUN_00435790(pvVar7,(int)pvVar4);
        if (local_2c != 0) {
          pvVar4 = FUN_00419e90(*(void **)(param_1 + 0xc),0,0,6,0,0,0,&DAT_00000002);
          FUN_00435790(pvVar7,(int)pvVar4);
          pvVar4 = FUN_00419e90(*(void **)(param_1 + 0xc),0,0,6,0,0,0,&DAT_00000002);
          FUN_00435790(pvVar7,(int)pvVar4);
        }
        puVar5 = FUN_004357b0(pvVar7,local_28);
        local_4._0_1_ = 8;
        FUN_004f26d0(&local_34,puVar5);
        local_4 = CONCAT31(local_4._1_3_,1);
        FUN_00619730();
        while( true ) {
          local_20 = 0x80;
          local_1c = 0x90;
          if (((local_34 >> 0x18 < 0x80) || (0x8f < local_34 >> 0x18)) || ((int)uVar8 <= iVar9)) {
            bVar1 = false;
          }
          else {
            bVar1 = true;
          }
          FUN_00619730();
          if (!bVar1) break;
          iVar6 = FUN_004e5260(local_18,(int *)&local_34);
          if (iVar6 == 0) {
            local_24 = (void *)FUN_00618b70(0x28);
            local_4._0_1_ = 9;
            if (local_24 == (void *)0x0) {
              pvVar4 = (void *)0x0;
            }
            else {
              pvVar4 = FUN_004ecac0(local_24,&local_34,3,0);
            }
            local_4 = CONCAT31(local_4._1_3_,1);
            FUN_005f5440(local_18,pvVar4);
            iVar9 = iVar9 + 1;
          }
          puVar5 = FUN_004357b0(pvVar7,local_28);
          local_4._0_1_ = 10;
          FUN_004f26d0(&local_34,puVar5);
          local_4 = CONCAT31(local_4._1_3_,1);
          FUN_00619730();
          uVar8 = local_30;
        }
        FUN_005f58b0(pvVar7);
        uVar8 = local_30;
        if (iVar9 < (int)local_30) {
          pvVar4 = FUN_00419a00(*(void **)(param_1 + 0xc),0,0,0x80000,0,0,4,4,(void *)0x1);
          pvVar7 = (void *)(param_1 + 0x1c);
          FUN_00435790(pvVar7,(int)pvVar4);
          if (local_2c != 0) {
            pvVar4 = FUN_00419e90(*(void **)(param_1 + 0xc),0,0,0x80000,0,0,4,&DAT_00000002);
            FUN_00435790(pvVar7,(int)pvVar4);
            pvVar4 = FUN_00419e90(*(void **)(param_1 + 0xc),0,0,0x80000,0,0,4,&DAT_00000002);
            FUN_00435790(pvVar7,(int)pvVar4);
          }
          puVar5 = FUN_004357b0(pvVar7,local_28);
          local_4._0_1_ = 0xb;
          FUN_004f26d0(&local_34,puVar5);
          local_4 = CONCAT31(local_4._1_3_,1);
          FUN_00619730();
          while( true ) {
            local_20 = 0x80;
            local_1c = 0x90;
            if (((local_34 >> 0x18 < 0x80) || (0x8f < local_34 >> 0x18)) || ((int)uVar8 <= iVar9)) {
              bVar1 = false;
            }
            else {
              bVar1 = true;
            }
            FUN_00619730();
            if (!bVar1) break;
            iVar6 = FUN_004e5260(local_18,(int *)&local_34);
            if (iVar6 == 0) {
              local_24 = (void *)FUN_00618b70(0x28);
              local_4._0_1_ = 0xc;
              if (local_24 == (void *)0x0) {
                pvVar4 = (void *)0x0;
              }
              else {
                pvVar4 = FUN_004ecac0(local_24,&local_34,1,0);
              }
              local_4 = CONCAT31(local_4._1_3_,1);
              FUN_005f5440(local_18,pvVar4);
              iVar9 = iVar9 + 1;
            }
            puVar5 = FUN_004357b0(pvVar7,local_28);
            local_4._0_1_ = 0xd;
            FUN_004f26d0(&local_34,puVar5);
            local_4 = CONCAT31(local_4._1_3_,1);
            FUN_00619730();
            uVar8 = local_30;
          }
          FUN_005f58b0(pvVar7);
          if (iVar9 < (int)local_30) {
            *(uint *)(param_1 + 0x10) = *(uint *)(param_1 + 0x10) | 1;
          }
        }
      }
    }
  }
  sVar3 = FUN_005f50e0((int)local_18);
  if (sVar3 != 0) {
    for (iVar9 = thunk_FUN_005f5060(*(int *)(param_1 + 0x2c)); iVar9 != 0;
        iVar9 = *(int *)(iVar9 + 0x10)) {
      pvVar7 = FUN_004f5940((void *)(*(int *)(param_1 + 0xc) + 0x44),(uint *)(iVar9 + 0x1c));
      if (pvVar7 != (void *)0x0) {
        *(uint *)((int)pvVar7 + 0x2c) = *(uint *)((int)pvVar7 + 0x2c) & 0xf7ffffff;
        FUN_004317b0((int)pvVar7);
        *(undefined4 *)((int)pvVar7 + 0x3c) = 0;
      }
    }
    (**(code **)(**(int **)(param_1 + 0x2c) + 4))();
    pvVar7 = (void *)thunk_FUN_005f5060((int)local_18);
    while (pvVar7 != (void *)0x0) {
      pvVar4 = FUN_004f5940((void *)(*(int *)(param_1 + 0xc) + 0x44),(uint *)((int)pvVar7 + 0x1c));
      if (pvVar4 != (void *)0x0) {
        *(uint *)((int)pvVar4 + 0x2c) = *(uint *)((int)pvVar4 + 0x2c) | 0x8000000;
        FUN_00431790(pvVar4,*(undefined4 *)((int)pvVar7 + 0x20),*(undefined4 *)((int)pvVar7 + 0x24))
        ;
        *(undefined4 *)((int)pvVar4 + 0x3c) = *(undefined4 *)((int)pvVar7 + 0x18);
      }
      pvVar4 = *(void **)((int)pvVar7 + 0x10);
      FUN_005f54d0(local_18,*(uint *)((int)pvVar7 + 0x18));
      FUN_005f5440(*(void **)(param_1 + 0x2c),pvVar7);
      pvVar7 = pvVar4;
    }
  }
  local_4 = local_4 & 0xffffff00;
  FUN_004e5150(local_18);
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return;
}

