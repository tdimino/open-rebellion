
void __fastcall FUN_004e6c00(int param_1)

{
  bool bVar1;
  bool bVar2;
  ushort uVar3;
  short sVar4;
  int iVar5;
  void *pvVar6;
  undefined4 *puVar7;
  void *pvVar8;
  void *pvVar9;
  uint uVar10;
  uint local_44;
  void *local_40;
  uint local_3c;
  int local_38;
  int local_34;
  uint local_30;
  void *local_2c;
  void *local_28;
  undefined4 local_24;
  undefined4 local_20;
  undefined4 local_1c;
  undefined4 local_18 [3];
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063e56a;
  pvStack_c = ExceptionList;
  bVar1 = false;
  local_40 = (void *)0x0;
  ExceptionList = &pvStack_c;
  local_38 = param_1;
  FUN_004ece30(&local_30);
  local_4 = 0;
  FUN_004ece30(&local_44);
  local_4._0_1_ = 1;
  FUN_004e5110(local_18);
  local_4._0_1_ = 2;
  local_34 = 0;
  bVar2 = false;
  uVar3 = FUN_005f50e0(*(int *)(param_1 + 0x2c));
  uVar10 = (uint)uVar3;
  if (uVar10 != 0) {
    local_34 = 1;
  }
  if (*(int *)(param_1 + 0x14) == 0) {
    if (*(int *)(param_1 + 0x18) != 0) {
      uVar10 = uVar10 - 1;
    }
  }
  else {
    uVar10 = uVar10 + 1;
  }
  local_3c = uVar10;
  iVar5 = thunk_FUN_005f5060(*(int *)(param_1 + 0x2c));
  if (iVar5 != 0) {
    FUN_004f26d0(&local_30,(undefined4 *)(iVar5 + 0x1c));
  }
  if (0 < (int)uVar10) {
    pvVar6 = FUN_00419a00(*(void **)(param_1 + 0xc),0,0,0,0,0,2,4,&DAT_00000002);
    pvVar8 = (void *)(param_1 + 0x1c);
    FUN_00435790(pvVar8,(int)pvVar6);
    local_28 = (void *)0x80;
    local_24 = 0x90;
    if ((0x7f < local_30 >> 0x18) && (local_30 >> 0x18 < 0x90)) {
      bVar1 = true;
    }
    FUN_00619730();
    if (bVar1) {
      puVar7 = FUN_00419b80(*(void **)(param_1 + 0xc),&local_30,0,0,0,0,0,2,2);
      FUN_00435790(pvVar8,(int)puVar7);
    }
    if (local_34 != 0) {
      pvVar6 = FUN_00419e90(*(void **)(param_1 + 0xc),0,0,0,0,0,2,&DAT_00000002);
      FUN_00435790(pvVar8,(int)pvVar6);
      pvVar6 = FUN_00419e90(*(void **)(param_1 + 0xc),0,0,0,0,0,2,&DAT_00000002);
      FUN_00435790(pvVar8,(int)pvVar6);
    }
    puVar7 = (undefined4 *)FUN_004357b0(pvVar8,&local_2c);
    local_4._0_1_ = 3;
    FUN_004f26d0(&local_44,puVar7);
    local_4._0_1_ = 2;
    FUN_00619730();
    FUN_005f58b0(pvVar8);
    local_28 = (void *)0x80;
    local_24 = 0x90;
    if ((local_44 >> 0x18 < 0x80) || (0x8f < local_44 >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if ((bVar1) && (iVar5 = FUN_004e5260(local_18,(int *)&local_44), iVar5 == 0)) {
      local_2c = (void *)FUN_00618b70(0x28);
      local_4._0_1_ = 4;
      if (local_2c == (void *)0x0) {
        pvVar8 = (void *)0x0;
      }
      else {
        pvVar8 = FUN_004ecac0(local_2c,&local_44,5,0);
      }
      local_4._0_1_ = 2;
      FUN_005f5440(local_18,pvVar8);
      local_40 = (void *)0x1;
    }
  }
  if ((int)local_40 < (int)local_3c) {
    pvVar6 = FUN_00419a00(*(void **)(param_1 + 0xc),0,0,0x408,0,0,0x40000,0xc,&DAT_00000002);
    pvVar8 = (void *)(param_1 + 0x1c);
    FUN_00435790(pvVar8,(int)pvVar6);
    if (local_34 != 0) {
      pvVar6 = FUN_00419e90(*(void **)(param_1 + 0xc),0,0,0x408,0,0,0x40000,&DAT_00000002);
      FUN_00435790(pvVar8,(int)pvVar6);
      pvVar6 = FUN_00419e90(*(void **)(param_1 + 0xc),0,0,0x408,0,0,0x40000,&DAT_00000002);
      FUN_00435790(pvVar8,(int)pvVar6);
    }
    puVar7 = (undefined4 *)FUN_004357b0(pvVar8,&local_2c);
    local_4._0_1_ = 5;
    FUN_004f26d0(&local_44,puVar7);
    local_4._0_1_ = 2;
    FUN_00619730();
    local_28 = (void *)0x80;
    local_24 = 0x90;
    if ((local_44 >> 0x18 < 0x80) || (0x8f < local_44 >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    pvVar6 = local_40;
    if (!bVar1) {
      FUN_005f58b0(pvVar8);
      iVar5 = local_38;
      pvVar6 = FUN_00419a00(*(void **)(local_38 + 0xc),0,0,0x408,0,0,0,0x48,&DAT_00000002);
      FUN_00435790(pvVar8,(int)pvVar6);
      if (local_34 != 0) {
        pvVar6 = FUN_00419e90(*(void **)(iVar5 + 0xc),0,0,0x408,0,0,0,&DAT_00000002);
        FUN_00435790(pvVar8,(int)pvVar6);
        pvVar6 = FUN_00419e90(*(void **)(iVar5 + 0xc),0,0,0x408,0,0,0,&DAT_00000002);
        FUN_00435790(pvVar8,(int)pvVar6);
      }
      puVar7 = (undefined4 *)FUN_004357b0(pvVar8,&local_2c);
      local_4._0_1_ = 6;
      FUN_004f26d0(&local_44,puVar7);
      local_4._0_1_ = 2;
      FUN_00619730();
      pvVar6 = local_40;
    }
    while( true ) {
      local_28 = (void *)0x80;
      local_24 = 0x90;
      if (((local_44 >> 0x18 < 0x80) || (0x8f < local_44 >> 0x18)) || (bVar2)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (!bVar1) break;
      iVar5 = FUN_004e5260(local_18,(int *)&local_44);
      if (iVar5 == 0) {
        local_40 = (void *)FUN_00618b70(0x28);
        local_4._0_1_ = 7;
        if (local_40 == (void *)0x0) {
          pvVar9 = (void *)0x0;
        }
        else {
          pvVar9 = FUN_004ecac0(local_40,&local_44,6,0);
        }
        local_4._0_1_ = 2;
        FUN_005f5440(local_18,pvVar9);
        pvVar6 = (void *)((int)pvVar6 + 1);
        bVar2 = true;
      }
      puVar7 = (undefined4 *)FUN_004357b0(pvVar8,&local_2c);
      local_4._0_1_ = 8;
      FUN_004f26d0(&local_44,puVar7);
      local_4._0_1_ = 2;
      FUN_00619730();
    }
    local_40 = pvVar6;
    FUN_005f58b0(pvVar8);
    param_1 = local_38;
  }
  if ((int)local_40 < (int)local_3c) {
    pvVar6 = FUN_00419a00(*(void **)(param_1 + 0xc),0x40000000,0,4,0,0,0,0x26,(void *)0x1);
    pvVar8 = (void *)(param_1 + 0x1c);
    FUN_00435790(pvVar8,(int)pvVar6);
    if (local_34 != 0) {
      pvVar6 = FUN_00419e90(*(void **)(param_1 + 0xc),0x40000000,0,4,0,0,0,&DAT_00000002);
      FUN_00435790(pvVar8,(int)pvVar6);
      pvVar6 = FUN_00419e90(*(void **)(param_1 + 0xc),0x40000000,0,4,0,0,0,&DAT_00000002);
      FUN_00435790(pvVar8,(int)pvVar6);
    }
    puVar7 = (undefined4 *)FUN_004357b0(pvVar8,&local_2c);
    local_4._0_1_ = 9;
    FUN_004f26d0(&local_44,puVar7);
    while( true ) {
      local_4._0_1_ = 2;
      FUN_00619730();
      local_20 = 0x80;
      local_1c = 0x90;
      if (((local_44 >> 0x18 < 0x80) || (0x8f < local_44 >> 0x18)) ||
         ((int)local_3c <= (int)local_40)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (!bVar1) break;
      iVar5 = FUN_004e5260(local_18,(int *)&local_44);
      if (iVar5 == 0) {
        local_28 = (void *)FUN_00618b70(0x28);
        local_4._0_1_ = 10;
        if (local_28 == (void *)0x0) {
          pvVar6 = (void *)0x0;
        }
        else {
          pvVar6 = FUN_004ecac0(local_28,&local_44,3,0);
        }
        local_4._0_1_ = 2;
        FUN_005f5440(local_18,pvVar6);
        local_40 = (void *)((int)local_40 + 1);
      }
      puVar7 = (undefined4 *)FUN_004357b0(pvVar8,&local_2c);
      local_4._0_1_ = 0xb;
      FUN_004f26d0(&local_44,puVar7);
    }
    FUN_005f58b0(pvVar8);
    param_1 = local_38;
  }
  if ((int)local_40 < (int)local_3c) {
    pvVar6 = FUN_00419a00(*(void **)(param_1 + 0xc),0,0,0x16,0,0,8,0x26,(void *)0x1);
    pvVar8 = (void *)(param_1 + 0x1c);
    FUN_00435790(pvVar8,(int)pvVar6);
    if (local_34 != 0) {
      pvVar6 = FUN_00419e90(*(void **)(param_1 + 0xc),0,0,0x16,0,0,8,&DAT_00000002);
      FUN_00435790(pvVar8,(int)pvVar6);
      pvVar6 = FUN_00419e90(*(void **)(param_1 + 0xc),0,0,0x16,0,0,8,&DAT_00000002);
      FUN_00435790(pvVar8,(int)pvVar6);
    }
    puVar7 = (undefined4 *)FUN_004357b0(pvVar8,&local_2c);
    local_4._0_1_ = 0xc;
    FUN_004f26d0(&local_44,puVar7);
    while( true ) {
      local_4._0_1_ = 2;
      FUN_00619730();
      local_20 = 0x80;
      local_1c = 0x90;
      if (((local_44 >> 0x18 < 0x80) || (0x8f < local_44 >> 0x18)) ||
         ((int)local_3c <= (int)local_40)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (!bVar1) break;
      iVar5 = FUN_004e5260(local_18,(int *)&local_44);
      if (iVar5 == 0) {
        local_28 = (void *)FUN_00618b70(0x28);
        local_4._0_1_ = 0xd;
        if (local_28 == (void *)0x0) {
          pvVar6 = (void *)0x0;
        }
        else {
          pvVar6 = FUN_004ecac0(local_28,&local_44,3,0);
        }
        local_4._0_1_ = 2;
        FUN_005f5440(local_18,pvVar6);
        local_40 = (void *)((int)local_40 + 1);
      }
      puVar7 = (undefined4 *)FUN_004357b0(pvVar8,&local_2c);
      local_4._0_1_ = 0xe;
      FUN_004f26d0(&local_44,puVar7);
    }
    FUN_005f58b0(pvVar8);
    param_1 = local_38;
  }
  if ((int)local_40 < (int)local_3c) {
    pvVar6 = FUN_00419a00(*(void **)(param_1 + 0xc),0,0,6,0,0,0,0x26,(void *)0x1);
    pvVar8 = (void *)(param_1 + 0x1c);
    FUN_00435790(pvVar8,(int)pvVar6);
    if (local_34 != 0) {
      pvVar6 = FUN_00419e90(*(void **)(param_1 + 0xc),0,0,6,0,0,0,&DAT_00000002);
      FUN_00435790(pvVar8,(int)pvVar6);
      pvVar6 = FUN_00419e90(*(void **)(param_1 + 0xc),0,0,6,0,0,0,&DAT_00000002);
      FUN_00435790(pvVar8,(int)pvVar6);
    }
    puVar7 = (undefined4 *)FUN_004357b0(pvVar8,&local_2c);
    local_4._0_1_ = 0xf;
    FUN_004f26d0(&local_44,puVar7);
    while( true ) {
      local_4._0_1_ = 2;
      FUN_00619730();
      local_20 = 0x80;
      local_1c = 0x90;
      if (((local_44 >> 0x18 < 0x80) || (0x8f < local_44 >> 0x18)) ||
         ((int)local_3c <= (int)local_40)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (!bVar1) break;
      iVar5 = FUN_004e5260(local_18,(int *)&local_44);
      if (iVar5 == 0) {
        local_28 = (void *)FUN_00618b70(0x28);
        local_4._0_1_ = 0x10;
        if (local_28 == (void *)0x0) {
          pvVar6 = (void *)0x0;
        }
        else {
          pvVar6 = FUN_004ecac0(local_28,&local_44,3,0);
        }
        local_4._0_1_ = 2;
        FUN_005f5440(local_18,pvVar6);
        local_40 = (void *)((int)local_40 + 1);
      }
      puVar7 = (undefined4 *)FUN_004357b0(pvVar8,&local_2c);
      local_4._0_1_ = 0x11;
      FUN_004f26d0(&local_44,puVar7);
    }
    FUN_005f58b0(pvVar8);
    param_1 = local_38;
  }
  if ((int)local_40 < (int)local_3c) {
    pvVar6 = FUN_00419a00(*(void **)(param_1 + 0xc),0,0,0x80000,0,0,4,4,(void *)0x1);
    pvVar8 = (void *)(param_1 + 0x1c);
    FUN_00435790(pvVar8,(int)pvVar6);
    if (local_34 != 0) {
      pvVar6 = FUN_00419e90(*(void **)(param_1 + 0xc),0,0,0x80000,0,0,4,&DAT_00000002);
      FUN_00435790(pvVar8,(int)pvVar6);
      pvVar6 = FUN_00419e90(*(void **)(param_1 + 0xc),0,0,0x80000,0,0,4,&DAT_00000002);
      FUN_00435790(pvVar8,(int)pvVar6);
    }
    puVar7 = (undefined4 *)FUN_004357b0(pvVar8,&local_2c);
    local_4._0_1_ = 0x12;
    FUN_004f26d0(&local_44,puVar7);
    while( true ) {
      local_4._0_1_ = 2;
      FUN_00619730();
      local_20 = 0x80;
      local_1c = 0x90;
      if (((local_44 >> 0x18 < 0x80) || (0x8f < local_44 >> 0x18)) ||
         ((int)local_3c <= (int)local_40)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (!bVar1) break;
      iVar5 = FUN_004e5260(local_18,(int *)&local_44);
      if (iVar5 == 0) {
        local_28 = (void *)FUN_00618b70(0x28);
        local_4._0_1_ = 0x13;
        if (local_28 == (void *)0x0) {
          pvVar6 = (void *)0x0;
        }
        else {
          pvVar6 = FUN_004ecac0(local_28,&local_44,1,0);
        }
        local_4._0_1_ = 2;
        FUN_005f5440(local_18,pvVar6);
        local_40 = (void *)((int)local_40 + 1);
      }
      puVar7 = (undefined4 *)FUN_004357b0(pvVar8,&local_2c);
      local_4._0_1_ = 0x14;
      FUN_004f26d0(&local_44,puVar7);
    }
    FUN_005f58b0(pvVar8);
    param_1 = local_38;
  }
  if ((int)local_40 < (int)local_3c) {
    *(uint *)(param_1 + 0x10) = *(uint *)(param_1 + 0x10) | 1;
  }
  sVar4 = FUN_005f50e0((int)local_18);
  if (sVar4 != 0) {
    for (iVar5 = thunk_FUN_005f5060(*(int *)(param_1 + 0x2c)); iVar5 != 0;
        iVar5 = *(int *)(iVar5 + 0x10)) {
      pvVar8 = FUN_004f5940((void *)(*(int *)(local_38 + 0xc) + 0x44),(uint *)(iVar5 + 0x1c));
      if (pvVar8 != (void *)0x0) {
        *(uint *)((int)pvVar8 + 0x2c) = *(uint *)((int)pvVar8 + 0x2c) & 0xf7ffffff;
        FUN_004317b0((int)pvVar8);
        *(undefined4 *)((int)pvVar8 + 0x3c) = 0;
      }
    }
    (**(code **)(**(int **)(local_38 + 0x2c) + 4))();
    pvVar8 = (void *)thunk_FUN_005f5060((int)local_18);
    iVar5 = local_38;
    while (pvVar8 != (void *)0x0) {
      pvVar6 = FUN_004f5940((void *)(*(int *)(iVar5 + 0xc) + 0x44),(uint *)((int)pvVar8 + 0x1c));
      if (pvVar6 != (void *)0x0) {
        *(uint *)((int)pvVar6 + 0x2c) = *(uint *)((int)pvVar6 + 0x2c) | 0x8000000;
        FUN_00431790(pvVar6,*(undefined4 *)((int)pvVar8 + 0x20),*(undefined4 *)((int)pvVar8 + 0x24))
        ;
        *(undefined4 *)((int)pvVar6 + 0x3c) = *(undefined4 *)((int)pvVar8 + 0x18);
      }
      pvVar6 = *(void **)((int)pvVar8 + 0x10);
      FUN_005f54d0(local_18,*(uint *)((int)pvVar8 + 0x18));
      FUN_005f5440(*(void **)(iVar5 + 0x2c),pvVar8);
      pvVar8 = pvVar6;
    }
  }
  local_4._0_1_ = 1;
  FUN_004e5150(local_18);
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return;
}

