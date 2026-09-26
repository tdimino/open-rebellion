
int __fastcall FUN_004744b0(int param_1)

{
  uint *puVar1;
  int iVar2;
  bool bVar3;
  uint uVar4;
  void *pvVar5;
  void *pvVar6;
  undefined4 *puVar7;
  uint local_4c;
  uint local_48;
  int local_44;
  void *local_40;
  undefined1 local_3c [4];
  undefined1 *local_38;
  undefined1 *local_34;
  undefined4 local_30;
  undefined4 local_2c;
  undefined4 local_28;
  undefined4 local_24;
  undefined4 local_20;
  undefined4 local_1c [4];
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00632ab0;
  local_c = ExceptionList;
  bVar3 = false;
  local_44 = 0;
  ExceptionList = &local_c;
  FUN_004355d0(local_1c);
  local_4 = 0;
  FUN_004ece30(&local_48);
  local_4._0_1_ = 1;
  FUN_004ece30(&local_4c);
  puVar1 = (uint *)(param_1 + 0x30);
  local_4._0_1_ = 2;
  uVar4 = *(uint *)(param_1 + 0x30) >> 0x18;
  local_34 = (undefined1 *)0x90;
  local_30 = 0x98;
  if ((0x8f < uVar4) && (uVar4 < 0x98)) {
    bVar3 = true;
  }
  FUN_00619730();
  if (bVar3) {
    iVar2 = *(int *)(param_1 + 0x4c);
    local_38 = &stack0xffffffa0;
    FUN_004f26d0(&stack0xffffffa0,puVar1);
    pvVar5 = FUN_00403d30((void *)(iVar2 + 0x2c));
    if (pvVar5 != (void *)0x0) {
      if ((((*(byte *)((int)pvVar5 + 0x30) & 1) == 0) ||
          ((*(uint *)((int)pvVar5 + 0x28) & 0x80000000) == 0)) ||
         ((*(uint *)((int)pvVar5 + 0x28) & 2) != 0)) {
        FUN_004ece80(puVar1);
        *(uint *)((int)pvVar5 + 0x30) = *(uint *)((int)pvVar5 + 0x30) & 0xfff7ffff;
      }
      else {
        local_44 = 1;
        *(uint *)((int)pvVar5 + 0x30) = *(uint *)((int)pvVar5 + 0x30) | 0x80000;
      }
    }
  }
  local_34 = (undefined1 *)0x90;
  local_30 = 0x98;
  if ((*puVar1 >> 0x18 < 0x90) || (0x97 < *puVar1 >> 0x18)) {
    bVar3 = false;
  }
  else {
    bVar3 = true;
  }
  FUN_00619730();
  if (!bVar3) {
    pvVar6 = FUN_0041a1d0(*(void **)(param_1 + 0x4c),0x40000000,0,0,0,0,0,(void *)0x1);
    pvVar5 = (void *)(param_1 + 0x3c);
    local_40 = pvVar5;
    FUN_00435790(pvVar5,(int)pvVar6);
    puVar7 = FUN_004357b0(pvVar5,local_3c);
    local_4._0_1_ = 3;
    FUN_004f26d0(&local_48,puVar7);
    while( true ) {
      local_4._0_1_ = 2;
      FUN_00619730();
      local_2c = 0x80;
      local_28 = 0x90;
      if (((local_48 >> 0x18 < 0x80) || (0x8f < local_48 >> 0x18)) || (local_44 != 0)) {
        bVar3 = false;
      }
      else {
        bVar3 = true;
      }
      FUN_00619730();
      if (!bVar3) break;
      pvVar5 = FUN_0041a460(*(void **)(param_1 + 0x4c),&local_48,0x80000000,0,1,2,0,0x80000,
                            (void *)0x1);
      FUN_00435790(local_1c,(int)pvVar5);
      puVar7 = FUN_004357b0(local_1c,local_3c);
      local_4._0_1_ = 4;
      FUN_004f26d0(&local_4c,puVar7);
      local_4 = CONCAT31(local_4._1_3_,2);
      FUN_00619730();
      FUN_005f58b0(local_1c);
      local_24 = 0x90;
      local_20 = 0x98;
      if ((local_4c >> 0x18 < 0x90) || (0x97 < local_4c >> 0x18)) {
        bVar3 = false;
      }
      else {
        bVar3 = true;
      }
      FUN_00619730();
      if (bVar3) {
        iVar2 = *(int *)(param_1 + 0x4c);
        local_34 = &stack0xffffffa0;
        FUN_004f26d0(&stack0xffffffa0,&local_4c);
        pvVar5 = FUN_00403d30((void *)(iVar2 + 0x2c));
        if (pvVar5 != (void *)0x0) {
          FUN_004f26d0((void *)(param_1 + 0x30),&local_4c);
          local_44 = 1;
          *(uint *)((int)pvVar5 + 0x30) = *(uint *)((int)pvVar5 + 0x30) | 0x80000;
        }
      }
      puVar7 = FUN_004357b0(local_40,&local_38);
      local_4._0_1_ = 5;
      FUN_004f26d0(&local_48,puVar7);
    }
    FUN_005f58b0(local_40);
  }
  local_4._0_1_ = 1;
  FUN_00619730();
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_00435610(local_1c);
  ExceptionList = local_c;
  return local_44;
}

