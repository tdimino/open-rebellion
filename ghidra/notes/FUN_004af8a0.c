
undefined4 __fastcall FUN_004af8a0(int param_1)

{
  int iVar1;
  bool bVar2;
  bool bVar3;
  uint uVar4;
  void *pvVar5;
  void *pvVar6;
  undefined4 *puVar7;
  uint local_24;
  undefined4 local_20;
  undefined1 *local_1c;
  undefined4 local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00638c70;
  local_c = ExceptionList;
  bVar3 = false;
  local_20 = 0;
  ExceptionList = &local_c;
  FUN_004ece30(&local_24);
  local_1c = (undefined1 *)0x90;
  local_18 = 0x98;
  local_4 = 0;
  uVar4 = *(uint *)(*(int *)(param_1 + 0x10) + 0x40) >> 0x18;
  bVar2 = true;
  if ((0x8f < uVar4) && (uVar4 < 0x98)) {
    bVar3 = true;
  }
  FUN_00619730();
  if (bVar3) {
    iVar1 = *(int *)(param_1 + 0xc);
    local_1c = &stack0xffffffc8;
    FUN_004f26d0(&stack0xffffffc8,(undefined4 *)(*(int *)(param_1 + 0x10) + 0x40));
    pvVar5 = FUN_00403d30((void *)(iVar1 + 0x2c));
    if (pvVar5 != (void *)0x0) {
      pvVar6 = FUN_004f5940((void *)(*(int *)(param_1 + 0xc) + 0x44),
                            (uint *)(*(int *)(param_1 + 0x10) + 0x34));
      if (((pvVar6 != (void *)0x0) && ((*(uint *)((int)pvVar6 + 0x24) & 0x40000000) != 0)) &&
         ((*(uint *)((int)pvVar5 + 0x28) & 0x80000000) == 0)) {
        bVar2 = false;
      }
      if (((bVar2) && (uVar4 = *(uint *)((int)pvVar5 + 0x30), (uVar4 & 1) != 0)) &&
         ((*(byte *)((int)pvVar5 + 0x28) & 2) == 0)) {
        if ((uVar4 & 0x30000) != 0) {
          *(uint *)((int)pvVar5 + 0x30) = uVar4 & 0xfffcffff;
          FUN_00433e30((int)pvVar5);
        }
        local_20 = 1;
      }
      else {
        *(uint *)((int)pvVar5 + 0x30) = *(uint *)((int)pvVar5 + 0x30) & 0xfff8ffff;
        FUN_00433e30((int)pvVar5);
        FUN_004ece80((undefined4 *)(*(int *)(param_1 + 0x10) + 0x40));
      }
    }
  }
  local_1c = (undefined1 *)0x90;
  local_18 = 0x98;
  uVar4 = *(uint *)(*(int *)(param_1 + 0x10) + 0x40) >> 0x18;
  if ((uVar4 < 0x90) || (0x97 < uVar4)) {
    bVar2 = false;
  }
  else {
    bVar2 = true;
  }
  FUN_00619730();
  if (!bVar2) {
    pvVar6 = FUN_0041a340(*(void **)(param_1 + 0xc),(undefined4 *)(*(int *)(param_1 + 0x10) + 0x34),
                          0x80000000,0,1,2,0,0x40000,0x26,(void *)0x1);
    pvVar5 = (void *)(param_1 + 0x18);
    FUN_00435790(pvVar5,(int)pvVar6);
    puVar7 = FUN_004357b0(pvVar5,&local_1c);
    local_4._0_1_ = 1;
    FUN_004f26d0(&local_24,puVar7);
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_00619730();
    FUN_005f58b0(pvVar5);
    local_1c = (undefined1 *)0x90;
    local_18 = 0x98;
    if ((local_24 >> 0x18 < 0x90) || (0x97 < local_24 >> 0x18)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    FUN_00619730();
    if (bVar2) {
      FUN_004f26d0((void *)(*(int *)(param_1 + 0x10) + 0x40),&local_24);
    }
    local_1c = (undefined1 *)0x90;
    local_18 = 0x98;
    uVar4 = *(uint *)(*(int *)(param_1 + 0x10) + 0x40) >> 0x18;
    if ((uVar4 < 0x90) || (0x97 < uVar4)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    FUN_00619730();
    if (!bVar2) {
      pvVar6 = FUN_0041a340(*(void **)(param_1 + 0xc),
                            (undefined4 *)(*(int *)(param_1 + 0x10) + 0x34),0,0,1,2,0,0x40000,0x26,
                            (void *)0x1);
      FUN_00435790(pvVar5,(int)pvVar6);
      puVar7 = FUN_004357b0(pvVar5,&local_1c);
      local_4._0_1_ = 2;
      FUN_004f26d0(&local_24,puVar7);
      local_4 = (uint)local_4._1_3_ << 8;
      FUN_00619730();
      FUN_005f58b0(pvVar5);
      local_1c = (undefined1 *)0x90;
      local_18 = 0x98;
      if ((local_24 >> 0x18 < 0x90) || (0x97 < local_24 >> 0x18)) {
        bVar2 = false;
      }
      else {
        bVar2 = true;
      }
      FUN_00619730();
      if (bVar2) {
        FUN_004f26d0((void *)(*(int *)(param_1 + 0x10) + 0x40),&local_24);
      }
    }
    local_1c = (undefined1 *)0x90;
    local_18 = 0x98;
    uVar4 = *(uint *)(*(int *)(param_1 + 0x10) + 0x40) >> 0x18;
    if ((uVar4 < 0x90) || (0x97 < uVar4)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    FUN_00619730();
    if (!bVar2) {
      puVar7 = FUN_0041b2b0(*(void **)(param_1 + 0xc),&local_1c,
                            (uint *)(*(int *)(param_1 + 0x10) + 0x34),0,0,1,2,0,0x40000);
      local_4._0_1_ = 3;
      FUN_004f26d0(&local_24,puVar7);
      local_4 = (uint)local_4._1_3_ << 8;
      FUN_00619730();
      local_1c = (undefined1 *)0x90;
      local_18 = 0x98;
      if ((local_24 >> 0x18 < 0x90) || (0x97 < local_24 >> 0x18)) {
        bVar2 = false;
      }
      else {
        bVar2 = true;
      }
      FUN_00619730();
      if (bVar2) {
        FUN_004f26d0((void *)(*(int *)(param_1 + 0x10) + 0x40),&local_24);
      }
    }
    local_14 = 0x90;
    local_10 = 0x98;
    uVar4 = *(uint *)(*(int *)(param_1 + 0x10) + 0x40) >> 0x18;
    if ((uVar4 < 0x90) || (0x97 < uVar4)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    FUN_00619730();
    if (bVar2) {
      iVar1 = *(int *)(param_1 + 0xc);
      local_1c = &stack0xffffffc8;
      FUN_004f26d0(&stack0xffffffc8,(undefined4 *)(*(int *)(param_1 + 0x10) + 0x40));
      pvVar5 = FUN_00403d30((void *)(iVar1 + 0x2c));
      if (pvVar5 != (void *)0x0) {
        uVar4 = *(uint *)((int)pvVar5 + 0x30) & 0xfffcffff;
        *(uint *)((int)pvVar5 + 0x30) = uVar4;
        *(uint *)((int)pvVar5 + 0x30) = uVar4 | 0x40000;
        FUN_00433e30((int)pvVar5);
        local_20 = 1;
      }
    }
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return local_20;
}

