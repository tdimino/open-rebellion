
void __fastcall FUN_004e9690(void *param_1)

{
  bool bVar1;
  ushort uVar2;
  short sVar3;
  undefined4 uVar4;
  int iVar5;
  void *pvVar6;
  void *pvVar7;
  undefined4 *puVar8;
  uint uVar9;
  uint local_3c;
  int local_38;
  uint local_34;
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
  puStack_8 = &LAB_0063e7d7;
  pvStack_c = ExceptionList;
  local_38 = 0;
  ExceptionList = &pvStack_c;
  FUN_004ece30(&local_3c);
  local_4 = 0;
  FUN_004e5110(local_18);
  local_4._0_1_ = 1;
  local_30 = 0;
  uVar2 = FUN_005f50e0(*(int *)((int)param_1 + 0x2c));
  local_34 = (uint)uVar2;
  if (local_34 != 0) {
    local_30 = 1;
  }
  uVar9 = (uint)(local_34 != 0);
  if (*(int *)((int)param_1 + 0x14) == 0) {
    if (*(int *)((int)param_1 + 0x18) != 0) {
      local_34 = local_34 - 1;
    }
  }
  else {
    local_34 = local_34 + 1;
  }
  if (0 < (int)local_34) {
    uVar4 = FUN_004e5b10(param_1,uVar9,&local_3c);
    local_28 = (void *)0x80;
    local_24 = 0x90;
    if ((local_3c >> 0x18 < 0x80) || (0x8f < local_3c >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if ((bVar1) && (iVar5 = FUN_004e5260(local_18,(int *)&local_3c), iVar5 == 0)) {
      local_2c = (void *)FUN_00618b70(0x28);
      local_4._0_1_ = 2;
      if (local_2c == (void *)0x0) {
        pvVar6 = (void *)0x0;
      }
      else {
        pvVar6 = FUN_004ecac0(local_2c,&local_3c,4,uVar4);
      }
      local_4._0_1_ = 1;
      FUN_005f5440(local_18,pvVar6);
      local_38 = 1;
    }
  }
  if (local_38 < (int)local_34) {
    pvVar7 = FUN_00419a00(*(void **)((int)param_1 + 0xc),0x40000000,0,4,0,0,0,0x26,(void *)0x1);
    pvVar6 = (void *)((int)param_1 + 0x1c);
    FUN_00435790(pvVar6,(int)pvVar7);
    if (uVar9 != 0) {
      pvVar7 = FUN_00419e90(*(void **)((int)param_1 + 0xc),0x40000000,0,4,0,0,0,&DAT_00000002);
      FUN_00435790(pvVar6,(int)pvVar7);
      pvVar7 = FUN_00419e90(*(void **)((int)param_1 + 0xc),0x40000000,0,4,0,0,0,&DAT_00000002);
      FUN_00435790(pvVar6,(int)pvVar7);
    }
    puVar8 = FUN_004357b0(pvVar6,&local_2c);
    local_4._0_1_ = 3;
    FUN_004f26d0(&local_3c,puVar8);
    while( true ) {
      local_4._0_1_ = 1;
      FUN_00619730();
      local_20 = 0x80;
      local_1c = 0x90;
      if (((local_3c >> 0x18 < 0x80) || (0x8f < local_3c >> 0x18)) || ((int)local_34 <= local_38)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (!bVar1) break;
      iVar5 = FUN_004e5260(local_18,(int *)&local_3c);
      if (iVar5 == 0) {
        local_28 = (void *)FUN_00618b70(0x28);
        local_4._0_1_ = 4;
        if (local_28 == (void *)0x0) {
          pvVar7 = (void *)0x0;
        }
        else {
          pvVar7 = FUN_004ecac0(local_28,&local_3c,3,0);
        }
        local_4._0_1_ = 1;
        FUN_005f5440(local_18,pvVar7);
        local_38 = local_38 + 1;
      }
      puVar8 = FUN_004357b0(pvVar6,&local_2c);
      local_4._0_1_ = 5;
      FUN_004f26d0(&local_3c,puVar8);
    }
    FUN_005f58b0(pvVar6);
    uVar9 = local_30;
  }
  if (local_38 < (int)local_34) {
    pvVar7 = FUN_00419a00(*(void **)((int)param_1 + 0xc),0,0,0x16,0,0,8,0x26,(void *)0x1);
    pvVar6 = (void *)((int)param_1 + 0x1c);
    FUN_00435790(pvVar6,(int)pvVar7);
    if (uVar9 != 0) {
      pvVar7 = FUN_00419e90(*(void **)((int)param_1 + 0xc),0,0,0x16,0,0,8,&DAT_00000002);
      FUN_00435790(pvVar6,(int)pvVar7);
      pvVar7 = FUN_00419e90(*(void **)((int)param_1 + 0xc),0,0,0x16,0,0,8,&DAT_00000002);
      FUN_00435790(pvVar6,(int)pvVar7);
    }
    puVar8 = FUN_004357b0(pvVar6,&local_2c);
    local_4._0_1_ = 6;
    FUN_004f26d0(&local_3c,puVar8);
    while( true ) {
      local_4._0_1_ = 1;
      FUN_00619730();
      local_20 = 0x80;
      local_1c = 0x90;
      if (((local_3c >> 0x18 < 0x80) || (0x8f < local_3c >> 0x18)) || ((int)local_34 <= local_38)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (!bVar1) break;
      iVar5 = FUN_004e5260(local_18,(int *)&local_3c);
      if (iVar5 == 0) {
        local_28 = (void *)FUN_00618b70(0x28);
        local_4._0_1_ = 7;
        if (local_28 == (void *)0x0) {
          pvVar7 = (void *)0x0;
        }
        else {
          pvVar7 = FUN_004ecac0(local_28,&local_3c,3,0);
        }
        local_4._0_1_ = 1;
        FUN_005f5440(local_18,pvVar7);
        local_38 = local_38 + 1;
      }
      puVar8 = FUN_004357b0(pvVar6,&local_2c);
      local_4._0_1_ = 8;
      FUN_004f26d0(&local_3c,puVar8);
    }
    FUN_005f58b0(pvVar6);
    uVar9 = local_30;
  }
  if (local_38 < (int)local_34) {
    pvVar7 = FUN_00419a00(*(void **)((int)param_1 + 0xc),0,0,6,0,0,0,0x26,(void *)0x1);
    pvVar6 = (void *)((int)param_1 + 0x1c);
    FUN_00435790(pvVar6,(int)pvVar7);
    if (uVar9 != 0) {
      pvVar7 = FUN_00419e90(*(void **)((int)param_1 + 0xc),0,0,6,0,0,0,&DAT_00000002);
      FUN_00435790(pvVar6,(int)pvVar7);
      pvVar7 = FUN_00419e90(*(void **)((int)param_1 + 0xc),0,0,6,0,0,0,&DAT_00000002);
      FUN_00435790(pvVar6,(int)pvVar7);
    }
    puVar8 = FUN_004357b0(pvVar6,&local_2c);
    local_4._0_1_ = 9;
    FUN_004f26d0(&local_3c,puVar8);
    while( true ) {
      local_4._0_1_ = 1;
      FUN_00619730();
      local_20 = 0x80;
      local_1c = 0x90;
      if (((local_3c >> 0x18 < 0x80) || (0x8f < local_3c >> 0x18)) || ((int)local_34 <= local_38)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (!bVar1) break;
      iVar5 = FUN_004e5260(local_18,(int *)&local_3c);
      if (iVar5 == 0) {
        local_28 = (void *)FUN_00618b70(0x28);
        local_4._0_1_ = 10;
        if (local_28 == (void *)0x0) {
          pvVar7 = (void *)0x0;
        }
        else {
          pvVar7 = FUN_004ecac0(local_28,&local_3c,3,0);
        }
        local_4._0_1_ = 1;
        FUN_005f5440(local_18,pvVar7);
        local_38 = local_38 + 1;
      }
      puVar8 = FUN_004357b0(pvVar6,&local_2c);
      local_4._0_1_ = 0xb;
      FUN_004f26d0(&local_3c,puVar8);
    }
    FUN_005f58b0(pvVar6);
    uVar9 = local_30;
  }
  if (local_38 < (int)local_34) {
    pvVar7 = FUN_00419a00(*(void **)((int)param_1 + 0xc),0,0,0x80000,0,0,0,4,(void *)0x1);
    pvVar6 = (void *)((int)param_1 + 0x1c);
    FUN_00435790(pvVar6,(int)pvVar7);
    if (uVar9 != 0) {
      pvVar7 = FUN_00419e90(*(void **)((int)param_1 + 0xc),0,0,0x80000,0,0,0,&DAT_00000002);
      FUN_00435790(pvVar6,(int)pvVar7);
      pvVar7 = FUN_00419e90(*(void **)((int)param_1 + 0xc),0,0,0x80000,0,0,0,&DAT_00000002);
      FUN_00435790(pvVar6,(int)pvVar7);
    }
    puVar8 = FUN_004357b0(pvVar6,&local_2c);
    local_4._0_1_ = 0xc;
    FUN_004f26d0(&local_3c,puVar8);
    while( true ) {
      local_4._0_1_ = 1;
      FUN_00619730();
      local_20 = 0x80;
      local_1c = 0x90;
      if (((local_3c >> 0x18 < 0x80) || (0x8f < local_3c >> 0x18)) || ((int)local_34 <= local_38)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (!bVar1) break;
      iVar5 = FUN_004e5260(local_18,(int *)&local_3c);
      if (iVar5 == 0) {
        local_28 = (void *)FUN_00618b70(0x28);
        local_4._0_1_ = 0xd;
        if (local_28 == (void *)0x0) {
          pvVar7 = (void *)0x0;
        }
        else {
          pvVar7 = FUN_004ecac0(local_28,&local_3c,1,0);
        }
        local_4._0_1_ = 1;
        FUN_005f5440(local_18,pvVar7);
        local_38 = local_38 + 1;
      }
      puVar8 = FUN_004357b0(pvVar6,&local_2c);
      local_4._0_1_ = 0xe;
      FUN_004f26d0(&local_3c,puVar8);
    }
    FUN_005f58b0(pvVar6);
  }
  if (local_38 < (int)local_34) {
    *(uint *)((int)param_1 + 0x10) = *(uint *)((int)param_1 + 0x10) | 1;
  }
  sVar3 = FUN_005f50e0((int)local_18);
  if (sVar3 != 0) {
    for (iVar5 = thunk_FUN_005f5060(*(int *)((int)param_1 + 0x2c)); iVar5 != 0;
        iVar5 = *(int *)(iVar5 + 0x10)) {
      pvVar6 = FUN_004f5940((void *)(*(int *)((int)param_1 + 0xc) + 0x44),(uint *)(iVar5 + 0x1c));
      if (pvVar6 != (void *)0x0) {
        *(uint *)((int)pvVar6 + 0x2c) = *(uint *)((int)pvVar6 + 0x2c) & 0xf7ffffff;
        FUN_004317b0((int)pvVar6);
        *(undefined4 *)((int)pvVar6 + 0x3c) = 0;
      }
    }
    (**(code **)(**(int **)((int)param_1 + 0x2c) + 4))();
    pvVar6 = (void *)thunk_FUN_005f5060((int)local_18);
    while (pvVar6 != (void *)0x0) {
      pvVar7 = FUN_004f5940((void *)(*(int *)((int)param_1 + 0xc) + 0x44),
                            (uint *)((int)pvVar6 + 0x1c));
      if (pvVar7 != (void *)0x0) {
        *(uint *)((int)pvVar7 + 0x2c) = *(uint *)((int)pvVar7 + 0x2c) | 0x8000000;
        FUN_00431790(pvVar7,*(undefined4 *)((int)pvVar6 + 0x20),*(undefined4 *)((int)pvVar6 + 0x24))
        ;
        *(undefined4 *)((int)pvVar7 + 0x3c) = *(undefined4 *)((int)pvVar6 + 0x18);
      }
      pvVar7 = *(void **)((int)pvVar6 + 0x10);
      FUN_005f54d0(local_18,*(uint *)((int)pvVar6 + 0x18));
      FUN_005f5440(*(void **)((int)param_1 + 0x2c),pvVar6);
      pvVar6 = pvVar7;
    }
  }
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_004e5150(local_18);
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return;
}

