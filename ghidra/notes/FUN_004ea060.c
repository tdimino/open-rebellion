
void __fastcall FUN_004ea060(int param_1)

{
  bool bVar1;
  ushort uVar2;
  short sVar3;
  int iVar4;
  void *pvVar5;
  undefined4 *puVar6;
  void *pvVar7;
  uint uVar8;
  uint local_40;
  int local_3c;
  uint local_38;
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
  puStack_8 = &LAB_0063e8af;
  pvStack_c = ExceptionList;
  local_3c = 0;
  ExceptionList = &pvStack_c;
  FUN_004ece30(&local_30);
  local_4 = 0;
  FUN_004ece30(&local_40);
  local_4._0_1_ = 1;
  FUN_004e5110(local_18);
  local_4._0_1_ = 2;
  local_34 = 0;
  uVar2 = FUN_005f50e0(*(int *)(param_1 + 0x2c));
  uVar8 = (uint)uVar2;
  if (uVar8 != 0) {
    local_34 = 1;
  }
  if (*(int *)(param_1 + 0x14) == 0) {
    if (*(int *)(param_1 + 0x18) != 0) {
      uVar8 = uVar8 - 1;
    }
  }
  else {
    uVar8 = uVar8 + 1;
  }
  local_38 = uVar8;
  iVar4 = thunk_FUN_005f5060(*(int *)(param_1 + 0x2c));
  if (iVar4 != 0) {
    FUN_004f26d0(&local_30,(undefined4 *)(iVar4 + 0x1c));
  }
  if (0 < (int)uVar8) {
    pvVar5 = FUN_00419a00(*(void **)(param_1 + 0xc),0,0,0x40000,0,0,0,0x48,(void *)0x1);
    pvVar7 = (void *)(param_1 + 0x1c);
    FUN_00435790(pvVar7,(int)pvVar5);
    local_28 = (void *)0x80;
    local_24 = 0x90;
    if ((local_30 >> 0x18 < 0x80) || (0x8f < local_30 >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (bVar1) {
      puVar6 = FUN_00419b80(*(void **)(param_1 + 0xc),&local_30,0,0,0x40000,0,0,0,2);
      FUN_00435790(pvVar7,(int)puVar6);
    }
    if (local_34 != 0) {
      pvVar5 = FUN_00419e90(*(void **)(param_1 + 0xc),0,0,0x40000,0,0,0,&DAT_00000002);
      FUN_00435790(pvVar7,(int)pvVar5);
      pvVar5 = FUN_00419e90(*(void **)(param_1 + 0xc),0,0,0x40000,0,0,0,&DAT_00000002);
      FUN_00435790(pvVar7,(int)pvVar5);
    }
    puVar6 = (undefined4 *)FUN_004357b0(pvVar7,&local_2c);
    local_4._0_1_ = 3;
    FUN_004f26d0(&local_40,puVar6);
    local_4._0_1_ = 2;
    FUN_00619730();
    FUN_005f58b0(pvVar7);
    local_28 = (void *)0x80;
    local_24 = 0x90;
    if ((local_40 >> 0x18 < 0x80) || (0x8f < local_40 >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (!bVar1) {
      pvVar5 = FUN_0041a060(*(void **)(param_1 + 0xc),0,0,8,0,0,0x20000000,(void *)0x1);
      FUN_00435790(pvVar7,(int)pvVar5);
      local_28 = (void *)0x80;
      local_24 = 0x90;
      if ((local_30 >> 0x18 < 0x80) || (0x8f < local_30 >> 0x18)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (bVar1) {
        puVar6 = FUN_00419b80(*(void **)(param_1 + 0xc),&local_30,0,0,8,0,0,0x20000000,2);
        FUN_00435790(pvVar7,(int)puVar6);
      }
      if (local_34 != 0) {
        pvVar5 = FUN_00419e90(*(void **)(param_1 + 0xc),0,0,8,0,0,0x20000000,&DAT_00000002);
        FUN_00435790(pvVar7,(int)pvVar5);
        pvVar5 = FUN_00419e90(*(void **)(param_1 + 0xc),0,0,8,0,0,0x20000000,&DAT_00000002);
        FUN_00435790(pvVar7,(int)pvVar5);
      }
      puVar6 = (undefined4 *)FUN_004357b0(pvVar7,&local_2c);
      local_4._0_1_ = 4;
      FUN_004f26d0(&local_40,puVar6);
      local_4._0_1_ = 2;
      FUN_00619730();
      FUN_005f58b0(pvVar7);
    }
    local_28 = (void *)0x80;
    local_24 = 0x90;
    if ((local_40 >> 0x18 < 0x80) || (0x8f < local_40 >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if ((bVar1) && (iVar4 = FUN_004e5260(local_18,(int *)&local_40), iVar4 == 0)) {
      local_2c = (void *)FUN_00618b70(0x28);
      local_4._0_1_ = 5;
      if (local_2c == (void *)0x0) {
        pvVar7 = (void *)0x0;
      }
      else {
        pvVar7 = FUN_004ecac0(local_2c,&local_40,2,0);
      }
      local_4._0_1_ = 2;
      FUN_005f5440(local_18,pvVar7);
      local_3c = 1;
    }
  }
  if (local_3c < (int)local_38) {
    pvVar5 = FUN_00419a00(*(void **)(param_1 + 0xc),0x40000000,0,4,0,0,0,0x26,(void *)0x1);
    pvVar7 = (void *)(param_1 + 0x1c);
    FUN_00435790(pvVar7,(int)pvVar5);
    if (local_34 != 0) {
      pvVar5 = FUN_00419e90(*(void **)(param_1 + 0xc),0x40000000,0,4,0,0,0,&DAT_00000002);
      FUN_00435790(pvVar7,(int)pvVar5);
      pvVar5 = FUN_00419e90(*(void **)(param_1 + 0xc),0x40000000,0,4,0,0,0,&DAT_00000002);
      FUN_00435790(pvVar7,(int)pvVar5);
    }
    puVar6 = (undefined4 *)FUN_004357b0(pvVar7,&local_2c);
    local_4._0_1_ = 6;
    FUN_004f26d0(&local_40,puVar6);
    while( true ) {
      local_4._0_1_ = 2;
      FUN_00619730();
      local_20 = 0x80;
      local_1c = 0x90;
      if (((local_40 >> 0x18 < 0x80) || (0x8f < local_40 >> 0x18)) || ((int)local_38 <= local_3c)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (!bVar1) break;
      iVar4 = FUN_004e5260(local_18,(int *)&local_40);
      if (iVar4 == 0) {
        local_28 = (void *)FUN_00618b70(0x28);
        local_4._0_1_ = 7;
        if (local_28 == (void *)0x0) {
          pvVar5 = (void *)0x0;
        }
        else {
          pvVar5 = FUN_004ecac0(local_28,&local_40,3,0);
        }
        local_4._0_1_ = 2;
        FUN_005f5440(local_18,pvVar5);
        local_3c = local_3c + 1;
      }
      puVar6 = (undefined4 *)FUN_004357b0(pvVar7,&local_2c);
      local_4._0_1_ = 8;
      FUN_004f26d0(&local_40,puVar6);
    }
    FUN_005f58b0(pvVar7);
  }
  if (local_3c < (int)local_38) {
    pvVar5 = FUN_00419a00(*(void **)(param_1 + 0xc),0,0,0x16,0,0,8,0x26,(void *)0x1);
    pvVar7 = (void *)(param_1 + 0x1c);
    FUN_00435790(pvVar7,(int)pvVar5);
    if (local_34 != 0) {
      pvVar5 = FUN_00419e90(*(void **)(param_1 + 0xc),0,0,0x16,0,0,8,&DAT_00000002);
      FUN_00435790(pvVar7,(int)pvVar5);
      pvVar5 = FUN_00419e90(*(void **)(param_1 + 0xc),0,0,0x16,0,0,8,&DAT_00000002);
      FUN_00435790(pvVar7,(int)pvVar5);
    }
    puVar6 = (undefined4 *)FUN_004357b0(pvVar7,&local_2c);
    local_4._0_1_ = 9;
    FUN_004f26d0(&local_40,puVar6);
    while( true ) {
      local_4._0_1_ = 2;
      FUN_00619730();
      local_20 = 0x80;
      local_1c = 0x90;
      if (((local_40 >> 0x18 < 0x80) || (0x8f < local_40 >> 0x18)) || ((int)local_38 <= local_3c)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (!bVar1) break;
      iVar4 = FUN_004e5260(local_18,(int *)&local_40);
      if (iVar4 == 0) {
        local_28 = (void *)FUN_00618b70(0x28);
        local_4._0_1_ = 10;
        if (local_28 == (void *)0x0) {
          pvVar5 = (void *)0x0;
        }
        else {
          pvVar5 = FUN_004ecac0(local_28,&local_40,3,0);
        }
        local_4._0_1_ = 2;
        FUN_005f5440(local_18,pvVar5);
        local_3c = local_3c + 1;
      }
      puVar6 = (undefined4 *)FUN_004357b0(pvVar7,&local_2c);
      local_4._0_1_ = 0xb;
      FUN_004f26d0(&local_40,puVar6);
    }
    FUN_005f58b0(pvVar7);
  }
  if (local_3c < (int)local_38) {
    pvVar5 = FUN_00419a00(*(void **)(param_1 + 0xc),0,0,6,0,0,0,0x26,(void *)0x1);
    pvVar7 = (void *)(param_1 + 0x1c);
    FUN_00435790(pvVar7,(int)pvVar5);
    if (local_34 != 0) {
      pvVar5 = FUN_00419e90(*(void **)(param_1 + 0xc),0,0,6,0,0,0,&DAT_00000002);
      FUN_00435790(pvVar7,(int)pvVar5);
      pvVar5 = FUN_00419e90(*(void **)(param_1 + 0xc),0,0,6,0,0,0,&DAT_00000002);
      FUN_00435790(pvVar7,(int)pvVar5);
    }
    puVar6 = (undefined4 *)FUN_004357b0(pvVar7,&local_2c);
    local_4._0_1_ = 0xc;
    FUN_004f26d0(&local_40,puVar6);
    while( true ) {
      local_4._0_1_ = 2;
      FUN_00619730();
      local_20 = 0x80;
      local_1c = 0x90;
      if (((local_40 >> 0x18 < 0x80) || (0x8f < local_40 >> 0x18)) || ((int)local_38 <= local_3c)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (!bVar1) break;
      iVar4 = FUN_004e5260(local_18,(int *)&local_40);
      if (iVar4 == 0) {
        local_28 = (void *)FUN_00618b70(0x28);
        local_4._0_1_ = 0xd;
        if (local_28 == (void *)0x0) {
          pvVar5 = (void *)0x0;
        }
        else {
          pvVar5 = FUN_004ecac0(local_28,&local_40,3,0);
        }
        local_4._0_1_ = 2;
        FUN_005f5440(local_18,pvVar5);
        local_3c = local_3c + 1;
      }
      puVar6 = (undefined4 *)FUN_004357b0(pvVar7,&local_2c);
      local_4._0_1_ = 0xe;
      FUN_004f26d0(&local_40,puVar6);
    }
    FUN_005f58b0(pvVar7);
  }
  if (local_3c < (int)local_38) {
    pvVar5 = FUN_00419a00(*(void **)(param_1 + 0xc),0,0,0x80000,0,0,0,4,(void *)0x1);
    pvVar7 = (void *)(param_1 + 0x1c);
    FUN_00435790(pvVar7,(int)pvVar5);
    if (local_34 != 0) {
      pvVar5 = FUN_00419e90(*(void **)(param_1 + 0xc),0,0,0x80000,0,0,0,&DAT_00000002);
      FUN_00435790(pvVar7,(int)pvVar5);
      pvVar5 = FUN_00419e90(*(void **)(param_1 + 0xc),0,0,0x80000,0,0,0,&DAT_00000002);
      FUN_00435790(pvVar7,(int)pvVar5);
    }
    puVar6 = (undefined4 *)FUN_004357b0(pvVar7,&local_2c);
    local_4._0_1_ = 0xf;
    FUN_004f26d0(&local_40,puVar6);
    while( true ) {
      local_4._0_1_ = 2;
      FUN_00619730();
      local_20 = 0x80;
      local_1c = 0x90;
      if (((local_40 >> 0x18 < 0x80) || (0x8f < local_40 >> 0x18)) || ((int)local_38 <= local_3c)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (!bVar1) break;
      iVar4 = FUN_004e5260(local_18,(int *)&local_40);
      if (iVar4 == 0) {
        local_28 = (void *)FUN_00618b70(0x28);
        local_4._0_1_ = 0x10;
        if (local_28 == (void *)0x0) {
          pvVar5 = (void *)0x0;
        }
        else {
          pvVar5 = FUN_004ecac0(local_28,&local_40,1,0);
        }
        local_4._0_1_ = 2;
        FUN_005f5440(local_18,pvVar5);
        local_3c = local_3c + 1;
      }
      puVar6 = (undefined4 *)FUN_004357b0(pvVar7,&local_2c);
      local_4._0_1_ = 0x11;
      FUN_004f26d0(&local_40,puVar6);
    }
    FUN_005f58b0(pvVar7);
  }
  if (local_3c < (int)local_38) {
    *(uint *)(param_1 + 0x10) = *(uint *)(param_1 + 0x10) | 1;
  }
  sVar3 = FUN_005f50e0((int)local_18);
  if (sVar3 != 0) {
    for (iVar4 = thunk_FUN_005f5060(*(int *)(param_1 + 0x2c)); iVar4 != 0;
        iVar4 = *(int *)(iVar4 + 0x10)) {
      pvVar7 = FUN_004f5940((void *)(*(int *)(param_1 + 0xc) + 0x44),(uint *)(iVar4 + 0x1c));
      if (pvVar7 != (void *)0x0) {
        *(uint *)((int)pvVar7 + 0x2c) = *(uint *)((int)pvVar7 + 0x2c) & 0xf7ffffff;
        FUN_004317b0((int)pvVar7);
        *(undefined4 *)((int)pvVar7 + 0x3c) = 0;
      }
    }
    (**(code **)(**(int **)(param_1 + 0x2c) + 4))();
    pvVar7 = (void *)thunk_FUN_005f5060((int)local_18);
    while (pvVar7 != (void *)0x0) {
      pvVar5 = FUN_004f5940((void *)(*(int *)(param_1 + 0xc) + 0x44),(uint *)((int)pvVar7 + 0x1c));
      if (pvVar5 != (void *)0x0) {
        *(uint *)((int)pvVar5 + 0x2c) = *(uint *)((int)pvVar5 + 0x2c) | 0x8000000;
        FUN_00431790(pvVar5,*(undefined4 *)((int)pvVar7 + 0x20),*(undefined4 *)((int)pvVar7 + 0x24))
        ;
        *(undefined4 *)((int)pvVar5 + 0x3c) = *(undefined4 *)((int)pvVar7 + 0x18);
      }
      pvVar5 = *(void **)((int)pvVar7 + 0x10);
      FUN_005f54d0(local_18,*(uint *)((int)pvVar7 + 0x18));
      FUN_005f5440(*(void **)(param_1 + 0x2c),pvVar7);
      pvVar7 = pvVar5;
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

