
undefined4 __fastcall FUN_004d3ff0(int param_1)

{
  int iVar1;
  uint uVar2;
  bool bVar3;
  bool bVar4;
  void *pvVar5;
  undefined4 *puVar6;
  void *pvVar7;
  int *this;
  uint *puVar8;
  undefined4 local_2c;
  undefined1 local_28 [4];
  undefined1 local_24 [4];
  undefined1 local_20 [4];
  uint local_1c;
  undefined1 *local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063c3b8;
  local_c = ExceptionList;
  local_2c = 0;
  bVar4 = false;
  ExceptionList = &local_c;
  *(undefined4 *)(param_1 + 100) = 0;
  FUN_004ece80((undefined4 *)(*(int *)(param_1 + 0x2c) + 0x1c));
  pvVar5 = (void *)thunk_FUN_005f5060(param_1 + 0x6c);
  while (pvVar5 != (void *)0x0) {
    bVar3 = false;
    if (bVar4) goto LAB_004d4300;
    puVar6 = (undefined4 *)FUN_00403040(pvVar5,local_28);
    puVar8 = (uint *)(param_1 + 0x50);
    local_4 = 0;
    FUN_004f26d0(puVar8,puVar6);
    local_4 = 0xffffffff;
    FUN_00619730();
    local_14 = 0x90;
    local_10 = 0x98;
    if ((0x8f < *puVar8 >> 0x18) && (*puVar8 >> 0x18 < 0x98)) {
      bVar3 = true;
    }
    FUN_00619730();
    if (bVar3) {
      iVar1 = *(int *)(param_1 + 0x2c);
      local_18 = &stack0xffffffbc;
      FUN_004f26d0(&stack0xffffffbc,puVar8);
      pvVar7 = FUN_00403d30((void *)(iVar1 + 0x2c));
      if (pvVar7 == (void *)0x0) {
        pvVar7 = *(void **)((int)pvVar5 + 0x10);
        puVar8 = (uint *)FUN_00403040(pvVar5,local_20);
        local_4 = 2;
        FUN_004f58e0((void *)(param_1 + 0x6c),puVar8);
        goto LAB_004d41a7;
      }
      if (((*(byte *)((int)pvVar7 + 0x30) & 1) == 0) ||
         (uVar2 = *(uint *)((int)pvVar7 + 0x28), (uVar2 & 0x200000) == 0)) {
        *(uint *)((int)pvVar7 + 0x28) = *(uint *)((int)pvVar7 + 0x28) & 0xffdfffff;
        FUN_00433e30((int)pvVar7);
        pvVar7 = *(void **)((int)pvVar5 + 0x10);
        puVar8 = (uint *)FUN_00403040(pvVar5,local_24);
        local_4 = 1;
        FUN_004f58e0((void *)(param_1 + 0x6c),puVar8);
        goto LAB_004d41a7;
      }
      if ((0 < *(int *)((int)pvVar7 + 0x58)) && (((uVar2 & 0x200) != 0 && ((uVar2 & 3) == 0)))) {
        *(int *)(param_1 + 100) = *(int *)((int)pvVar7 + 0x58);
        FUN_004f26d0((void *)(*(int *)(param_1 + 0x2c) + 0x1c),puVar8);
        bVar4 = true;
      }
      pvVar5 = *(void **)((int)pvVar5 + 0x10);
    }
    else {
      pvVar7 = *(void **)((int)pvVar5 + 0x10);
      puVar8 = (uint *)FUN_00403040(pvVar5,&local_1c);
      local_4 = 3;
      FUN_004f58e0((void *)(param_1 + 0x6c),puVar8);
LAB_004d41a7:
      local_4 = 0xffffffff;
      FUN_00619730();
      pvVar5 = pvVar7;
    }
  }
  if (!bVar4) {
    puVar8 = (uint *)(param_1 + 0x50);
    FUN_004f26d0(puVar8,(undefined4 *)(*(int *)(param_1 + 0x2c) + 0x20));
    local_14 = 0x90;
    local_10 = 0x98;
    if ((*puVar8 >> 0x18 < 0x90) || (0x97 < *puVar8 >> 0x18)) {
      bVar3 = false;
    }
    else {
      bVar3 = true;
    }
    FUN_00619730();
    if (bVar3) {
      iVar1 = *(int *)(param_1 + 0x2c);
      local_18 = &stack0xffffffbc;
      FUN_004f26d0(&stack0xffffffbc,puVar8);
      pvVar5 = FUN_00403d30((void *)(iVar1 + 0x2c));
      if (((((pvVar5 != (void *)0x0) && ((*(byte *)((int)pvVar5 + 0x30) & 1) != 0)) &&
           (uVar2 = *(uint *)((int)pvVar5 + 0x28), (uVar2 & 0x200000) != 0)) &&
          ((0 < *(int *)((int)pvVar5 + 0x58) && ((uVar2 & 0x200) != 0)))) && ((uVar2 & 3) == 0)) {
        *(int *)(param_1 + 100) = *(int *)((int)pvVar5 + 0x58);
        bVar4 = true;
      }
    }
    if (!bVar4) {
      puVar8 = (uint *)(param_1 + 0x50);
      FUN_004f26d0(puVar8,(undefined4 *)(*(int *)(param_1 + 0x2c) + 0x24));
      local_14 = 0x90;
      local_10 = 0x98;
      if ((*puVar8 >> 0x18 < 0x90) || (0x97 < *puVar8 >> 0x18)) {
        bVar3 = false;
      }
      else {
        bVar3 = true;
      }
      FUN_00619730();
      if (bVar3) {
        iVar1 = *(int *)(param_1 + 0x2c);
        local_18 = &stack0xffffffbc;
        FUN_004f26d0(&stack0xffffffbc,puVar8);
        pvVar5 = FUN_00403d30((void *)(iVar1 + 0x2c));
        if ((((pvVar5 != (void *)0x0) && ((*(byte *)((int)pvVar5 + 0x30) & 1) != 0)) &&
            (uVar2 = *(uint *)((int)pvVar5 + 0x28), (uVar2 & 0x200000) != 0)) &&
           (((0 < *(int *)((int)pvVar5 + 0x58) && ((uVar2 & 0x200) != 0)) && ((uVar2 & 3) == 0)))) {
          *(int *)(param_1 + 100) = *(int *)((int)pvVar5 + 0x58);
          bVar4 = true;
        }
      }
      if (!bVar4) {
        FUN_004ece80((undefined4 *)(param_1 + 0x50));
        ExceptionList = local_c;
        return 0;
      }
    }
  }
LAB_004d4300:
  this = FUN_004f3220(*(int *)(param_1 + 0x30),(uint *)(param_1 + 0x50));
  if ((this != (int *)0x0) && (pvVar5 = (void *)FUN_00509670(this,0), pvVar5 != (void *)0x0)) {
    puVar8 = FUN_004025b0(pvVar5,&local_1c);
    local_4 = 4;
    FUN_004f26d0((uint *)(param_1 + 0x50),puVar8);
    local_4 = 0xffffffff;
    FUN_00619730();
    local_2c = 1;
  }
  ExceptionList = local_c;
  return local_2c;
}

