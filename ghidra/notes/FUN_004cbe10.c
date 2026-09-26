
void __fastcall FUN_004cbe10(int param_1)

{
  bool bVar1;
  ushort uVar2;
  void *pvVar3;
  undefined4 *puVar4;
  void *pvVar5;
  uint uVar6;
  int iVar7;
  uint local_4c;
  uint local_48;
  int local_44;
  int local_40;
  undefined1 local_3c [4];
  undefined1 local_38 [4];
  undefined1 local_34 [4];
  undefined1 *local_30;
  undefined4 local_2c;
  undefined4 local_28;
  undefined4 local_24;
  undefined4 local_20;
  undefined4 local_1c [4];
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063b52b;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004355d0(local_1c);
  iVar7 = 0;
  local_4 = 0;
  FUN_004ece30(&local_4c);
  local_4._0_1_ = 1;
  FUN_004ece30(&local_48);
  local_4._0_1_ = 2;
  local_44 = 0;
  FUN_004ece80((undefined4 *)(param_1 + 0x58));
  local_40 = FUN_004cc280(param_1);
  if (local_40 < 1) {
LAB_004cc08a:
    uVar6 = *(uint *)(param_1 + 0x58) >> 0x18;
    local_24 = 0x90;
    local_20 = 0x98;
    if ((uVar6 < 0x90) || (0x97 < uVar6)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (bVar1) {
      iVar7 = *(int *)(param_1 + 0x2c);
      local_30 = &stack0xffffffa0;
      FUN_004f26d0(&stack0xffffffa0,(undefined4 *)(param_1 + 0x58));
      pvVar3 = FUN_00403d30((void *)(iVar7 + 0x2c));
      if (pvVar3 != (void *)0x0) {
        local_30 = (undefined1 *)FUN_00618b70(0x1c);
        local_4._0_1_ = 7;
        if (local_30 == (undefined1 *)0x0) {
          pvVar5 = (void *)0x0;
        }
        else {
          pvVar5 = FUN_004ecc70(local_30,(undefined4 *)(param_1 + 0x58));
        }
        local_4._0_1_ = 2;
        if (pvVar5 != (void *)0x0) {
          FUN_004f57b0((void *)(param_1 + 0x78),pvVar5);
          *(uint *)((int)pvVar3 + 0x28) = *(uint *)((int)pvVar3 + 0x28) | 0x200000;
          FUN_00433e30((int)pvVar3);
        }
      }
    }
    else {
      bVar1 = false;
      uVar2 = FUN_005f50e0(param_1 + 0x78);
      if (uVar2 < 2) {
        pvVar3 = (void *)thunk_FUN_005f5060(param_1 + 0x78);
        if (pvVar3 != (void *)0x0) {
          puVar4 = FUN_00403040(pvVar3,local_34);
          iVar7 = *(int *)(param_1 + 0x2c);
          local_30 = &stack0xffffffa0;
          local_4._0_1_ = 8;
          FUN_004f26d0(&stack0xffffffa0,puVar4);
          pvVar3 = FUN_00403d30((void *)(iVar7 + 0x2c));
          local_4._0_1_ = 2;
          FUN_00619730();
          if ((pvVar3 != (void *)0x0) && (0 < *(int *)((int)pvVar3 + 0x58))) {
            bVar1 = true;
          }
        }
        if (!bVar1) {
          iVar7 = FUN_0041b230(*(void **)(param_1 + 0x2c),(uint *)(param_1 + 0x5c),(void *)0x2a,
                               0x4000,0x4000,2);
          if (iVar7 <= local_40) {
            *(uint *)(*(int *)(param_1 + 0x2c) + 8) = *(uint *)(*(int *)(param_1 + 0x2c) + 8) | 2;
          }
        }
      }
    }
    local_4._0_1_ = 1;
    FUN_00619730();
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_00619730();
    local_4 = 0xffffffff;
    FUN_00435610(local_1c);
    ExceptionList = local_c;
    return;
  }
  pvVar3 = FUN_00419a00(*(void **)(param_1 + 0x2c),0x20,0,0,0,0,0,0x11,(void *)0x1);
  FUN_00435790(local_1c,(int)pvVar3);
  puVar4 = FUN_004357b0(local_1c,local_3c);
  local_4._0_1_ = 3;
  FUN_004f26d0(&local_4c,puVar4);
  do {
    local_4._0_1_ = 2;
    FUN_00619730();
    while( true ) {
      local_2c = 0x80;
      local_28 = 0x90;
      if (((local_4c >> 0x18 < 0x80) || (0x8f < local_4c >> 0x18)) || (iVar7 != 0)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (!bVar1) {
        FUN_005f58b0(local_1c);
        goto LAB_004cc08a;
      }
      pvVar3 = FUN_004f5940((void *)(*(int *)(param_1 + 0x2c) + 0x44),&local_4c);
      if ((pvVar3 != (void *)0x0) && (0 < *(int *)((int)pvVar3 + 0xb4))) break;
      FUN_004ece80(&local_4c);
    }
    pvVar5 = FUN_0041a340(*(void **)(param_1 + 0x2c),&local_4c,0,0,1,0x200003,0,0x40000000,4,
                          (void *)0x1);
    pvVar3 = (void *)(param_1 + 0x40);
    FUN_00435790(pvVar3,(int)pvVar5);
    puVar4 = FUN_004357b0(pvVar3,local_3c);
    local_4._0_1_ = 4;
    FUN_004f26d0(&local_48,puVar4);
    local_4 = CONCAT31(local_4._1_3_,2);
    FUN_00619730();
    while( true ) {
      local_24 = 0x90;
      local_20 = 0x98;
      if (((local_48 >> 0x18 < 0x90) || (0x97 < local_48 >> 0x18)) || (iVar7 != 0)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (!bVar1) break;
      iVar7 = *(int *)(param_1 + 0x2c);
      local_30 = &stack0xffffffa0;
      FUN_004f26d0(&stack0xffffffa0,&local_48);
      pvVar5 = FUN_00403d30((void *)(iVar7 + 0x2c));
      if (((pvVar5 != (void *)0x0) && (0 < *(int *)((int)pvVar5 + 0x58))) &&
         (*(int *)((int)pvVar5 + 0xd0) <= local_40)) {
        FUN_004f26d0((void *)(param_1 + 0x58),&local_48);
        local_44 = 1;
      }
      puVar4 = FUN_004357b0(pvVar3,local_38);
      local_4._0_1_ = 5;
      FUN_004f26d0(&local_48,puVar4);
      local_4 = CONCAT31(local_4._1_3_,2);
      FUN_00619730();
      iVar7 = local_44;
    }
    FUN_005f58b0(pvVar3);
    puVar4 = FUN_004357b0(pvVar3,local_34);
    local_4._0_1_ = 6;
    FUN_004f26d0(&local_4c,puVar4);
  } while( true );
}

