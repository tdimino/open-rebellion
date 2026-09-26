
void __fastcall FUN_004d6290(int param_1)

{
  uint *puVar1;
  bool bVar2;
  ushort uVar3;
  void *pvVar4;
  undefined4 *puVar5;
  void *pvVar6;
  uint uVar7;
  int iVar8;
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
  puStack_8 = &LAB_0063c78b;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004355d0(local_1c);
  iVar8 = 0;
  local_4 = 0;
  FUN_004ece30(&local_4c);
  local_4._0_1_ = 1;
  FUN_004ece30(&local_48);
  local_4._0_1_ = 2;
  local_44 = 0;
  FUN_004ece80((undefined4 *)(param_1 + 0x58));
  local_40 = FUN_004d6700(param_1);
  if (local_40 < 1) {
LAB_004d650a:
    uVar7 = *(uint *)(param_1 + 0x58) >> 0x18;
    local_24 = 0x90;
    local_20 = 0x98;
    if ((uVar7 < 0x90) || (0x97 < uVar7)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    FUN_00619730();
    if (bVar2) {
      iVar8 = *(int *)(param_1 + 0x2c);
      local_30 = &stack0xffffffa0;
      FUN_004f26d0(&stack0xffffffa0,(undefined4 *)(param_1 + 0x58));
      pvVar4 = FUN_00403d30((void *)(iVar8 + 0x2c));
      if (pvVar4 != (void *)0x0) {
        local_30 = (undefined1 *)FUN_00618b70(0x1c);
        local_4._0_1_ = 7;
        if (local_30 == (undefined1 *)0x0) {
          pvVar6 = (void *)0x0;
        }
        else {
          pvVar6 = FUN_004ecc70(local_30,(undefined4 *)(param_1 + 0x58));
        }
        local_4._0_1_ = 2;
        if (pvVar6 != (void *)0x0) {
          FUN_004f57b0((void *)(param_1 + 0x6c),pvVar6);
          *(uint *)((int)pvVar4 + 0x28) = *(uint *)((int)pvVar4 + 0x28) | 0x200000;
          FUN_00433e30((int)pvVar4);
        }
      }
    }
    else {
      bVar2 = false;
      uVar3 = FUN_005f50e0(param_1 + 0x6c);
      if (uVar3 < 2) {
        pvVar4 = (void *)thunk_FUN_005f5060(param_1 + 0x6c);
        if (pvVar4 != (void *)0x0) {
          puVar5 = FUN_00403040(pvVar4,local_34);
          iVar8 = *(int *)(param_1 + 0x2c);
          local_30 = &stack0xffffffa0;
          local_4._0_1_ = 8;
          FUN_004f26d0(&stack0xffffffa0,puVar5);
          pvVar4 = FUN_00403d30((void *)(iVar8 + 0x2c));
          local_4._0_1_ = 2;
          FUN_00619730();
          if ((pvVar4 != (void *)0x0) && (0 < *(int *)((int)pvVar4 + 0x58))) {
            bVar2 = true;
          }
        }
        if (!bVar2) {
          iVar8 = FUN_0041b230(*(void **)(param_1 + 0x2c),(uint *)(param_1 + 0x5c),(void *)0x2a,
                               0x4000,0x4000,2);
          if (iVar8 <= local_40) {
            puVar1 = (uint *)(*(int *)(param_1 + 0x2c) + 8);
            *puVar1 = *puVar1 | 8;
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
  pvVar4 = FUN_00419a00(*(void **)(param_1 + 0x2c),0x20,0,0,0,0,0,0x11,(void *)0x1);
  FUN_00435790(local_1c,(int)pvVar4);
  puVar5 = FUN_004357b0(local_1c,local_3c);
  local_4._0_1_ = 3;
  FUN_004f26d0(&local_4c,puVar5);
  do {
    local_4._0_1_ = 2;
    FUN_00619730();
    while( true ) {
      local_2c = 0x80;
      local_28 = 0x90;
      if (((local_4c >> 0x18 < 0x80) || (0x8f < local_4c >> 0x18)) || (iVar8 != 0)) {
        bVar2 = false;
      }
      else {
        bVar2 = true;
      }
      FUN_00619730();
      if (!bVar2) {
        FUN_005f58b0(local_1c);
        goto LAB_004d650a;
      }
      pvVar4 = FUN_004f5940((void *)(*(int *)(param_1 + 0x2c) + 0x44),&local_4c);
      if ((pvVar4 != (void *)0x0) && (0 < *(int *)((int)pvVar4 + 0xb4))) break;
      FUN_004ece80(&local_4c);
    }
    pvVar6 = FUN_0041a340(*(void **)(param_1 + 0x2c),&local_4c,0,0,1,0x200003,0,0x40000000,4,
                          (void *)0x1);
    pvVar4 = (void *)(param_1 + 0x40);
    FUN_00435790(pvVar4,(int)pvVar6);
    puVar5 = FUN_004357b0(pvVar4,local_3c);
    local_4._0_1_ = 4;
    FUN_004f26d0(&local_48,puVar5);
    local_4 = CONCAT31(local_4._1_3_,2);
    FUN_00619730();
    while( true ) {
      local_24 = 0x90;
      local_20 = 0x98;
      if (((local_48 >> 0x18 < 0x90) || (0x97 < local_48 >> 0x18)) || (iVar8 != 0)) {
        bVar2 = false;
      }
      else {
        bVar2 = true;
      }
      FUN_00619730();
      if (!bVar2) break;
      iVar8 = *(int *)(param_1 + 0x2c);
      local_30 = &stack0xffffffa0;
      FUN_004f26d0(&stack0xffffffa0,&local_48);
      pvVar6 = FUN_00403d30((void *)(iVar8 + 0x2c));
      if (((pvVar6 != (void *)0x0) && (0 < *(int *)((int)pvVar6 + 0x58))) &&
         (*(int *)((int)pvVar6 + 0xd0) <= local_40)) {
        FUN_004f26d0((void *)(param_1 + 0x58),&local_48);
        local_44 = 1;
      }
      puVar5 = FUN_004357b0(pvVar4,local_38);
      local_4._0_1_ = 5;
      FUN_004f26d0(&local_48,puVar5);
      local_4 = CONCAT31(local_4._1_3_,2);
      FUN_00619730();
      iVar8 = local_44;
    }
    FUN_005f58b0(pvVar4);
    puVar5 = FUN_004357b0(pvVar4,local_34);
    local_4._0_1_ = 6;
    FUN_004f26d0(&local_4c,puVar5);
  } while( true );
}

