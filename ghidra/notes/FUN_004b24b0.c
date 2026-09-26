
undefined4 __fastcall FUN_004b24b0(int param_1)

{
  int iVar1;
  bool bVar2;
  uint uVar3;
  void *pvVar4;
  void *pvVar5;
  undefined4 *puVar6;
  undefined4 uVar7;
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
  puStack_8 = &LAB_00639080;
  local_c = ExceptionList;
  local_20 = 0;
  ExceptionList = &local_c;
  FUN_004ece30(&local_24);
  local_1c = (undefined1 *)0x90;
  local_18 = 0x98;
  local_4 = 0;
  uVar3 = *(uint *)(*(int *)(param_1 + 0x10) + 0x40) >> 0x18;
  if ((uVar3 < 0x90) || (0x97 < uVar3)) {
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
    pvVar4 = FUN_00403d30((void *)(iVar1 + 0x2c));
    if (pvVar4 != (void *)0x0) {
      uVar3 = *(uint *)((int)pvVar4 + 0x30);
      if (((uVar3 & 1) == 0) || ((*(byte *)((int)pvVar4 + 0x28) & 2) != 0)) {
        *(uint *)((int)pvVar4 + 0x30) = *(uint *)((int)pvVar4 + 0x30) & 0xfff8ffff;
        FUN_00433e30((int)pvVar4);
        FUN_004ece80((undefined4 *)(*(int *)(param_1 + 0x10) + 0x40));
      }
      else {
        if ((uVar3 & 0x20000) == 0) {
          *(uint *)((int)pvVar4 + 0x30) = uVar3 | 0x60000;
          *(uint *)((int)pvVar4 + 0x30) = uVar3 & 0xfffeffff | 0x60000;
          FUN_00433e30((int)pvVar4);
        }
        local_20 = 1;
      }
    }
  }
  local_1c = (undefined1 *)0x90;
  local_18 = 0x98;
  uVar3 = *(uint *)(*(int *)(param_1 + 0x10) + 0x40) >> 0x18;
  if ((uVar3 < 0x90) || (0x97 < uVar3)) {
    bVar2 = false;
  }
  else {
    bVar2 = true;
  }
  FUN_00619730();
  uVar7 = local_20;
  if (!bVar2) {
    pvVar5 = FUN_0041a340(*(void **)(param_1 + 0xc),(undefined4 *)(*(int *)(param_1 + 0x10) + 0x34),
                          0x80000000,0,1,2,0,0x40000,0x13,(void *)0x1);
    pvVar4 = (void *)(param_1 + 0x18);
    FUN_00435790(pvVar4,(int)pvVar5);
    puVar6 = FUN_004357b0(pvVar4,&local_1c);
    local_4._0_1_ = 1;
    FUN_004f26d0(&local_24,puVar6);
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_00619730();
    FUN_005f58b0(pvVar4);
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
    uVar3 = *(uint *)(*(int *)(param_1 + 0x10) + 0x40) >> 0x18;
    if ((uVar3 < 0x90) || (0x97 < uVar3)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    FUN_00619730();
    if (!bVar2) {
      pvVar5 = FUN_0041a340(*(void **)(param_1 + 0xc),
                            (undefined4 *)(*(int *)(param_1 + 0x10) + 0x34),0,0,1,2,0,0x40000,0x13,
                            (void *)0x1);
      FUN_00435790(pvVar4,(int)pvVar5);
      puVar6 = FUN_004357b0(pvVar4,&local_1c);
      local_4._0_1_ = 2;
      FUN_004f26d0(&local_24,puVar6);
      local_4 = (uint)local_4._1_3_ << 8;
      FUN_00619730();
      FUN_005f58b0(pvVar4);
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
    uVar3 = *(uint *)(*(int *)(param_1 + 0x10) + 0x40) >> 0x18;
    if ((uVar3 < 0x90) || (0x97 < uVar3)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    FUN_00619730();
    if (!bVar2) {
      puVar6 = FUN_0041b2b0(*(void **)(param_1 + 0xc),&local_1c,
                            (uint *)(*(int *)(param_1 + 0x10) + 0x34),0,0,1,2,0,0x40000);
      local_4._0_1_ = 3;
      FUN_004f26d0(&local_24,puVar6);
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
    uVar3 = *(uint *)(*(int *)(param_1 + 0x10) + 0x40) >> 0x18;
    if ((uVar3 < 0x90) || (0x97 < uVar3)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    FUN_00619730();
    uVar7 = local_20;
    if (bVar2) {
      iVar1 = *(int *)(param_1 + 0xc);
      local_1c = &stack0xffffffc8;
      FUN_004f26d0(&stack0xffffffc8,(undefined4 *)(*(int *)(param_1 + 0x10) + 0x40));
      pvVar4 = FUN_00403d30((void *)(iVar1 + 0x2c));
      uVar7 = local_20;
      if (pvVar4 != (void *)0x0) {
        uVar3 = *(uint *)((int)pvVar4 + 0x30);
        *(uint *)((int)pvVar4 + 0x30) = uVar3 | 0x60000;
        *(uint *)((int)pvVar4 + 0x30) = uVar3 & 0xfffeffff | 0x60000;
        FUN_00433e30((int)pvVar4);
        uVar7 = 1;
      }
    }
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return uVar7;
}

