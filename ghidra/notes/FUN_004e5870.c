
void __fastcall FUN_004e5870(int param_1)

{
  int iVar1;
  bool bVar2;
  void *pvVar3;
  void *pvVar4;
  undefined4 *puVar5;
  uint uVar6;
  uint local_20;
  undefined1 *local_1c;
  undefined4 local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063e378;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004ece30(&local_20);
  iVar1 = *(int *)(param_1 + 0xc);
  local_1c = &stack0xffffffd0;
  local_4 = 0;
  local_20 = 0x90000109;
  FUN_004f26d0(&stack0xffffffd0,&local_20);
  pvVar3 = FUN_00403d30((void *)(iVar1 + 0x2c));
  if (pvVar3 != (void *)0x0) {
    if ((*(byte *)((int)pvVar3 + 0x30) & 1) == 0) {
      uVar6 = *(uint *)((int)pvVar3 + 0x30) & 0xefffffff;
    }
    else {
      uVar6 = *(uint *)((int)pvVar3 + 0x30) | 0x10000000;
    }
    *(uint *)((int)pvVar3 + 0x30) = uVar6;
  }
  bVar2 = true;
  if (*(int *)(param_1 + 4) == 1) {
    local_1c = (undefined1 *)0x90;
    local_18 = 0x98;
    uVar6 = *(uint *)(*(int *)(param_1 + 0xc) + 0x28) >> 0x18;
    if ((uVar6 < 0x90) || (0x97 < uVar6)) {
      bVar2 = false;
    }
    FUN_00619730();
    if (bVar2) {
      iVar1 = *(int *)(param_1 + 0xc);
      local_1c = &stack0xffffffd0;
      FUN_004f26d0(&stack0xffffffd0,(undefined4 *)(iVar1 + 0x28));
      pvVar3 = FUN_00403d30((void *)(iVar1 + 0x2c));
      if (pvVar3 != (void *)0x0) {
        uVar6 = *(uint *)((int)pvVar3 + 0x30);
        if (((uVar6 & 1) == 0) || ((uVar6 & 0x80000000) == 0)) {
          *(uint *)((int)pvVar3 + 0x30) = *(uint *)((int)pvVar3 + 0x30) & 0xefffffff;
          FUN_004ece80((undefined4 *)(*(int *)(param_1 + 0xc) + 0x28));
        }
        else {
          *(uint *)((int)pvVar3 + 0x30) = uVar6 | 0x10000000;
        }
      }
    }
    local_1c = (undefined1 *)0x90;
    local_18 = 0x98;
    uVar6 = *(uint *)(*(int *)(param_1 + 0xc) + 0x28) >> 0x18;
    if ((uVar6 < 0x90) || (0x97 < uVar6)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    FUN_00619730();
    if (!bVar2) {
      pvVar4 = FUN_00419a00(*(void **)(param_1 + 0xc),0,0,0x80000000,0,0,0,3,(void *)0x1);
      pvVar3 = (void *)(param_1 + 0x1c);
      FUN_00435790(pvVar3,(int)pvVar4);
      puVar5 = FUN_004357b0(pvVar3,&local_1c);
      local_4._0_1_ = 1;
      FUN_004f26d0(&local_20,puVar5);
      local_4._0_1_ = 0;
      FUN_00619730();
      FUN_005f58b0(pvVar3);
      pvVar4 = FUN_0041a340(*(void **)(param_1 + 0xc),&local_20,0,0,0x80000000,0,0,0,0,(void *)0x1);
      FUN_00435790(pvVar3,(int)pvVar4);
      puVar5 = FUN_004357b0(pvVar3,&local_1c);
      local_4._0_1_ = 2;
      FUN_004f26d0(&local_20,puVar5);
      local_4 = (uint)local_4._1_3_ << 8;
      FUN_00619730();
      FUN_005f58b0(pvVar3);
      local_14 = 0x90;
      local_10 = 0x98;
      if ((local_20 >> 0x18 < 0x90) || (0x97 < local_20 >> 0x18)) {
        bVar2 = false;
      }
      else {
        bVar2 = true;
      }
      FUN_00619730();
      if (bVar2) {
        iVar1 = *(int *)(param_1 + 0xc);
        local_1c = &stack0xffffffd0;
        FUN_004f26d0(&stack0xffffffd0,&local_20);
        pvVar3 = FUN_00403d30((void *)(iVar1 + 0x2c));
        if (pvVar3 != (void *)0x0) {
          if ((*(byte *)((int)pvVar3 + 0x30) & 1) == 0) {
            *(uint *)((int)pvVar3 + 0x30) = *(uint *)((int)pvVar3 + 0x30) & 0xefffffff;
          }
          else {
            *(uint *)((int)pvVar3 + 0x30) = *(uint *)((int)pvVar3 + 0x30) | 0x10000000;
            FUN_004f26d0((void *)(*(int *)(param_1 + 0xc) + 0x28),&local_20);
          }
        }
      }
    }
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return;
}

