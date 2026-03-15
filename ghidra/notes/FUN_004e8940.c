
void __fastcall FUN_004e8940(int param_1)

{
  bool bVar1;
  ushort uVar2;
  short sVar3;
  int iVar4;
  void *pvVar5;
  undefined4 *puVar6;
  void *pvVar7;
  void *pvVar8;
  int iVar9;
  undefined4 uVar10;
  uint uVar11;
  uint local_40;
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
  puStack_8 = &LAB_0063e717;
  pvStack_c = ExceptionList;
  local_2c = (void *)0x0;
  ExceptionList = &pvStack_c;
  local_34 = param_1;
  FUN_004ece30(&local_30);
  local_4 = 0;
  FUN_004ece30(&local_40);
  local_4._0_1_ = 1;
  FUN_004e5110(local_18);
  local_4._0_1_ = 2;
  local_38 = 0;
  uVar2 = FUN_005f50e0(*(int *)(param_1 + 0x2c));
  uVar11 = (uint)uVar2;
  if (uVar11 != 0) {
    local_38 = 1;
  }
  if (*(int *)(param_1 + 0x14) == 0) {
    if (*(int *)(param_1 + 0x18) != 0) {
      uVar11 = uVar11 - 1;
    }
  }
  else {
    uVar11 = uVar11 + 1;
  }
  local_3c = uVar11;
  iVar4 = thunk_FUN_005f5060(*(int *)(param_1 + 0x2c));
  if (iVar4 != 0) {
    FUN_004f26d0(&local_30,(undefined4 *)(iVar4 + 0x1c));
  }
  iVar4 = local_34;
  if (0 < (int)uVar11) {
    pvVar5 = FUN_00419a00(*(void **)(local_34 + 0xc),0,0,0x408,0,0,0x40000,0xc,&DAT_00000002);
    pvVar7 = (void *)(iVar4 + 0x1c);
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
      puVar6 = FUN_00419b80(*(void **)(iVar4 + 0xc),&local_30,0,0,0x408,0,0,0x40000,2);
      FUN_00435790(pvVar7,(int)puVar6);
    }
    if (local_38 != 0) {
      pvVar5 = FUN_00419e90(*(void **)(iVar4 + 0xc),0,0,0x408,0,0,0x40000,&DAT_00000002);
      FUN_00435790(pvVar7,(int)pvVar5);
      pvVar5 = FUN_00419e90(*(void **)(iVar4 + 0xc),0,0,0x408,0,0,0x40000,&DAT_00000002);
      FUN_00435790(pvVar7,(int)pvVar5);
    }
    puVar6 = (undefined4 *)FUN_004357b0(pvVar7,&local_28);
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
    iVar4 = local_34;
    if (!bVar1) {
      pvVar5 = FUN_00419a00(*(void **)(local_34 + 0xc),0,0,0x408,0,0,0,0x48,&DAT_00000002);
      FUN_00435790(pvVar7,(int)pvVar5);
      if (local_38 != 0) {
        pvVar5 = FUN_00419e90(*(void **)(iVar4 + 0xc),0,0,0x408,0,0,0,&DAT_00000002);
        FUN_00435790(pvVar7,(int)pvVar5);
        pvVar5 = FUN_00419e90(*(void **)(iVar4 + 0xc),0,0,0x408,0,0,0,&DAT_00000002);
        FUN_00435790(pvVar7,(int)pvVar5);
      }
      puVar6 = (undefined4 *)FUN_004357b0(pvVar7,&local_28);
      local_4._0_1_ = 4;
      FUN_004f26d0(&local_40,puVar6);
      local_4._0_1_ = 2;
      FUN_00619730();
      FUN_005f58b0(pvVar7);
    }
    uVar10 = 6;
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
      uVar10 = 5;
      pvVar5 = FUN_00419a00(*(void **)(iVar4 + 0xc),0,0,0,0,0,2,3,(void *)0x1);
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
      iVar4 = local_34;
      if (bVar1) {
        puVar6 = FUN_00419b80(*(void **)(local_34 + 0xc),&local_30,0,0,0,0,0,2,2);
        FUN_00435790(pvVar7,(int)puVar6);
      }
      if (local_38 != 0) {
        pvVar5 = FUN_00419e90(*(void **)(iVar4 + 0xc),0,0,0,0,0,2,&DAT_00000002);
        FUN_00435790(pvVar7,(int)pvVar5);
        pvVar5 = FUN_00419e90(*(void **)(iVar4 + 0xc),0,0,0,0,0,2,&DAT_00000002);
        FUN_00435790(pvVar7,(int)pvVar5);
      }
      puVar6 = (undefined4 *)FUN_004357b0(pvVar7,&local_28);
      local_4._0_1_ = 5;
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
      local_28 = (void *)FUN_00618b70(0x28);
      local_4._0_1_ = 6;
      if (local_28 == (void *)0x0) {
        pvVar7 = (void *)0x0;
      }
      else {
        pvVar7 = FUN_004ecac0(local_28,&local_40,uVar10,0);
      }
      local_4._0_1_ = 2;
      FUN_005f5440(local_18,pvVar7);
      local_2c = (void *)0x1;
    }
  }
  pvVar7 = local_2c;
  iVar4 = local_34;
  if ((int)local_2c < (int)local_3c) {
    pvVar8 = FUN_00419a00(*(void **)(local_34 + 0xc),0x40000000,0,4,0,0,0,0x26,(void *)0x1);
    pvVar5 = (void *)(iVar4 + 0x1c);
    FUN_00435790(pvVar5,(int)pvVar8);
    if (local_38 != 0) {
      pvVar8 = FUN_00419e90(*(void **)(iVar4 + 0xc),0x40000000,0,4,0,0,0,&DAT_00000002);
      FUN_00435790(pvVar5,(int)pvVar8);
      pvVar8 = FUN_00419e90(*(void **)(iVar4 + 0xc),0x40000000,0,4,0,0,0,&DAT_00000002);
      FUN_00435790(pvVar5,(int)pvVar8);
    }
    puVar6 = (undefined4 *)FUN_004357b0(pvVar5,&local_28);
    local_4._0_1_ = 7;
    FUN_004f26d0(&local_40,puVar6);
    while( true ) {
      local_4._0_1_ = 2;
      FUN_00619730();
      local_20 = 0x80;
      local_1c = 0x90;
      if (((local_40 >> 0x18 < 0x80) || (0x8f < local_40 >> 0x18)) || ((int)local_3c <= (int)pvVar7)
         ) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (!bVar1) break;
      iVar9 = FUN_004e5260(local_18,(int *)&local_40);
      if (iVar9 == 0) {
        local_2c = (void *)FUN_00618b70(0x28);
        local_4._0_1_ = 8;
        if (local_2c == (void *)0x0) {
          pvVar8 = (void *)0x0;
        }
        else {
          pvVar8 = FUN_004ecac0(local_2c,&local_40,3,0);
        }
        local_4._0_1_ = 2;
        FUN_005f5440(local_18,pvVar8);
        pvVar7 = (void *)((int)pvVar7 + 1);
      }
      puVar6 = (undefined4 *)FUN_004357b0(pvVar5,&local_28);
      local_4._0_1_ = 9;
      FUN_004f26d0(&local_40,puVar6);
    }
    FUN_005f58b0(pvVar5);
  }
  if ((int)pvVar7 < (int)local_3c) {
    pvVar8 = FUN_00419a00(*(void **)(iVar4 + 0xc),0,0,0x16,0,0,8,0x26,(void *)0x1);
    pvVar5 = (void *)(iVar4 + 0x1c);
    FUN_00435790(pvVar5,(int)pvVar8);
    if (local_38 != 0) {
      pvVar8 = FUN_00419e90(*(void **)(iVar4 + 0xc),0,0,0x16,0,0,8,&DAT_00000002);
      FUN_00435790(pvVar5,(int)pvVar8);
      pvVar8 = FUN_00419e90(*(void **)(iVar4 + 0xc),0,0,0x16,0,0,8,&DAT_00000002);
      FUN_00435790(pvVar5,(int)pvVar8);
    }
    puVar6 = (undefined4 *)FUN_004357b0(pvVar5,&local_28);
    local_4._0_1_ = 10;
    FUN_004f26d0(&local_40,puVar6);
    while( true ) {
      local_4._0_1_ = 2;
      FUN_00619730();
      local_20 = 0x80;
      local_1c = 0x90;
      if (((local_40 >> 0x18 < 0x80) || (0x8f < local_40 >> 0x18)) || ((int)local_3c <= (int)pvVar7)
         ) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (!bVar1) break;
      iVar9 = FUN_004e5260(local_18,(int *)&local_40);
      if (iVar9 == 0) {
        local_2c = (void *)FUN_00618b70(0x28);
        local_4._0_1_ = 0xb;
        if (local_2c == (void *)0x0) {
          pvVar8 = (void *)0x0;
        }
        else {
          pvVar8 = FUN_004ecac0(local_2c,&local_40,3,0);
        }
        local_4._0_1_ = 2;
        FUN_005f5440(local_18,pvVar8);
        pvVar7 = (void *)((int)pvVar7 + 1);
      }
      puVar6 = (undefined4 *)FUN_004357b0(pvVar5,&local_28);
      local_4._0_1_ = 0xc;
      FUN_004f26d0(&local_40,puVar6);
    }
    FUN_005f58b0(pvVar5);
    if ((int)pvVar7 < (int)local_3c) {
      pvVar8 = FUN_00419a00(*(void **)(iVar4 + 0xc),0,0,6,0,0,0,0x26,(void *)0x1);
      pvVar5 = (void *)(iVar4 + 0x1c);
      FUN_00435790(pvVar5,(int)pvVar8);
      if (local_38 != 0) {
        pvVar8 = FUN_00419e90(*(void **)(iVar4 + 0xc),0,0,6,0,0,0,&DAT_00000002);
        FUN_00435790(pvVar5,(int)pvVar8);
        pvVar8 = FUN_00419e90(*(void **)(iVar4 + 0xc),0,0,6,0,0,0,&DAT_00000002);
        FUN_00435790(pvVar5,(int)pvVar8);
      }
      puVar6 = (undefined4 *)FUN_004357b0(pvVar5,&local_28);
      local_4._0_1_ = 0xd;
      FUN_004f26d0(&local_40,puVar6);
      while( true ) {
        local_4._0_1_ = 2;
        FUN_00619730();
        local_20 = 0x80;
        local_1c = 0x90;
        if (((local_40 >> 0x18 < 0x80) || (0x8f < local_40 >> 0x18)) ||
           ((int)local_3c <= (int)pvVar7)) {
          bVar1 = false;
        }
        else {
          bVar1 = true;
        }
        FUN_00619730();
        if (!bVar1) break;
        iVar9 = FUN_004e5260(local_18,(int *)&local_40);
        if (iVar9 == 0) {
          local_2c = (void *)FUN_00618b70(0x28);
          local_4._0_1_ = 0xe;
          if (local_2c == (void *)0x0) {
            pvVar8 = (void *)0x0;
          }
          else {
            pvVar8 = FUN_004ecac0(local_2c,&local_40,3,0);
          }
          local_4._0_1_ = 2;
          FUN_005f5440(local_18,pvVar8);
          pvVar7 = (void *)((int)pvVar7 + 1);
        }
        puVar6 = (undefined4 *)FUN_004357b0(pvVar5,&local_28);
        local_4._0_1_ = 0xf;
        FUN_004f26d0(&local_40,puVar6);
      }
      FUN_005f58b0(pvVar5);
      if ((int)pvVar7 < (int)local_3c) {
        pvVar8 = FUN_00419a00(*(void **)(iVar4 + 0xc),0,0,0x80000,0,0,0,4,(void *)0x1);
        pvVar5 = (void *)(iVar4 + 0x1c);
        FUN_00435790(pvVar5,(int)pvVar8);
        if (local_38 != 0) {
          pvVar8 = FUN_00419e90(*(void **)(iVar4 + 0xc),0,0,0x80000,0,0,0,&DAT_00000002);
          FUN_00435790(pvVar5,(int)pvVar8);
          pvVar8 = FUN_00419e90(*(void **)(iVar4 + 0xc),0,0,0x80000,0,0,0,&DAT_00000002);
          FUN_00435790(pvVar5,(int)pvVar8);
        }
        puVar6 = (undefined4 *)FUN_004357b0(pvVar5,&local_28);
        local_4._0_1_ = 0x10;
        FUN_004f26d0(&local_40,puVar6);
        while( true ) {
          local_4._0_1_ = 2;
          FUN_00619730();
          local_20 = 0x80;
          local_1c = 0x90;
          if (((local_40 >> 0x18 < 0x80) || (0x8f < local_40 >> 0x18)) ||
             ((int)local_3c <= (int)pvVar7)) {
            bVar1 = false;
          }
          else {
            bVar1 = true;
          }
          FUN_00619730();
          if (!bVar1) break;
          iVar9 = FUN_004e5260(local_18,(int *)&local_40);
          if (iVar9 == 0) {
            local_2c = (void *)FUN_00618b70(0x28);
            local_4._0_1_ = 0x11;
            if (local_2c == (void *)0x0) {
              pvVar8 = (void *)0x0;
            }
            else {
              pvVar8 = FUN_004ecac0(local_2c,&local_40,1,0);
            }
            local_4._0_1_ = 2;
            FUN_005f5440(local_18,pvVar8);
            pvVar7 = (void *)((int)pvVar7 + 1);
          }
          puVar6 = (undefined4 *)FUN_004357b0(pvVar5,&local_28);
          local_4._0_1_ = 0x12;
          FUN_004f26d0(&local_40,puVar6);
        }
        FUN_005f58b0(pvVar5);
        if ((int)pvVar7 < (int)local_3c) {
          *(uint *)(iVar4 + 0x10) = *(uint *)(iVar4 + 0x10) | 1;
        }
      }
    }
  }
  sVar3 = FUN_005f50e0((int)local_18);
  iVar4 = local_34;
  if (sVar3 != 0) {
    for (iVar9 = thunk_FUN_005f5060(*(int *)(local_34 + 0x2c)); iVar9 != 0;
        iVar9 = *(int *)(iVar9 + 0x10)) {
      pvVar7 = FUN_004f5940((void *)(*(int *)(iVar4 + 0xc) + 0x44),(uint *)(iVar9 + 0x1c));
      if (pvVar7 != (void *)0x0) {
        *(uint *)((int)pvVar7 + 0x2c) = *(uint *)((int)pvVar7 + 0x2c) & 0xf7ffffff;
        FUN_004317b0((int)pvVar7);
        *(undefined4 *)((int)pvVar7 + 0x3c) = 0;
      }
    }
    (**(code **)(**(int **)(iVar4 + 0x2c) + 4))();
    pvVar7 = (void *)thunk_FUN_005f5060((int)local_18);
    while (pvVar7 != (void *)0x0) {
      pvVar5 = FUN_004f5940((void *)(*(int *)(iVar4 + 0xc) + 0x44),(uint *)((int)pvVar7 + 0x1c));
      if (pvVar5 != (void *)0x0) {
        *(uint *)((int)pvVar5 + 0x2c) = *(uint *)((int)pvVar5 + 0x2c) | 0x8000000;
        FUN_00431790(pvVar5,*(undefined4 *)((int)pvVar7 + 0x20),*(undefined4 *)((int)pvVar7 + 0x24))
        ;
        *(undefined4 *)((int)pvVar5 + 0x3c) = *(undefined4 *)((int)pvVar7 + 0x18);
      }
      pvVar5 = *(void **)((int)pvVar7 + 0x10);
      FUN_005f54d0(local_18,*(uint *)((int)pvVar7 + 0x18));
      FUN_005f5440(*(void **)(iVar4 + 0x2c),pvVar7);
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

