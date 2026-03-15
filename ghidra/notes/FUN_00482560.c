
undefined4 __fastcall FUN_00482560(int *param_1)

{
  int *piVar1;
  uint *puVar2;
  bool bVar3;
  short sVar4;
  void *pvVar5;
  undefined4 *puVar6;
  void *pvVar7;
  int iVar8;
  uint local_30;
  undefined4 local_2c;
  undefined1 local_28 [4];
  int local_24;
  undefined1 *local_20;
  undefined1 *local_1c;
  undefined4 local_18;
  undefined4 local_14 [2];
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063430b;
  pvStack_c = ExceptionList;
  bVar3 = false;
  local_2c = 0;
  ExceptionList = &pvStack_c;
  FUN_004f5510(local_14);
  local_4 = 0;
  FUN_004ece30(&local_30);
  piVar1 = param_1 + 0xd;
  local_4._0_1_ = 1;
  FUN_004ece80(piVar1);
  puVar2 = (uint *)(param_1 + 0xc);
  FUN_004ece80(puVar2);
  param_1[8] = param_1[8] & 0xffffffef;
  local_24 = FUN_004bd000((int)param_1);
  local_1c = (undefined1 *)0x18;
  local_18 = 0x1c;
  if ((0x17 < *puVar2 >> 0x18) && (*puVar2 >> 0x18 < 0x1c)) {
    bVar3 = true;
  }
  FUN_00619730();
  if (bVar3) {
    pvVar5 = FUN_004f5940((void *)(param_1[0x1a] + 0x58),puVar2);
    if (pvVar5 == (void *)0x0) {
      FUN_004ece80(piVar1);
      FUN_004ece80(puVar2);
    }
    else {
      iVar8 = param_1[0x1a];
      local_20 = &stack0xffffffbc;
      FUN_004f26d0(&stack0xffffffbc,(undefined4 *)((int)pvVar5 + 0x2c));
      pvVar5 = FUN_00419e40((void *)(iVar8 + 0x78));
      if (pvVar5 != (void *)0x0) {
        FUN_004f26d0(piVar1,(undefined4 *)((int)pvVar5 + 0x28));
      }
    }
  }
  local_1c = (undefined1 *)0x18;
  local_18 = 0x1c;
  if ((*puVar2 >> 0x18 < 0x18) || (0x1b < *puVar2 >> 0x18)) {
    bVar3 = false;
  }
  else {
    bVar3 = true;
  }
  FUN_00619730();
  if (!bVar3) {
    pvVar5 = FUN_0041a4c0((void *)param_1[0x1a],0x10,1,0x15,&DAT_00000002);
    piVar1 = param_1 + 0x15;
    FUN_00435790(piVar1,(int)pvVar5);
    puVar6 = (undefined4 *)FUN_004357b0(piVar1,local_28);
    local_4._0_1_ = 2;
    FUN_004f26d0(&local_30,puVar6);
    local_4._0_1_ = 1;
    FUN_00619730();
    while( true ) {
      local_1c = (undefined1 *)0x8;
      local_18 = 0x10;
      if ((local_30 >> 0x18 < 8) || (0xf < local_30 >> 0x18)) {
        bVar3 = false;
      }
      else {
        bVar3 = true;
      }
      FUN_00619730();
      if (!bVar3) break;
      iVar8 = param_1[0x1a];
      local_20 = &stack0xffffffbc;
      FUN_004f26d0(&stack0xffffffbc,&local_30);
      pvVar5 = FUN_00419e40((void *)(iVar8 + 0x78));
      if (pvVar5 != (void *)0x0) {
        local_20 = (undefined1 *)FUN_00618b70(0x1c);
        local_4._0_1_ = 3;
        if (local_20 == (undefined1 *)0x0) {
          pvVar5 = (void *)0x0;
        }
        else {
          pvVar5 = FUN_004ecc70(local_20,(undefined4 *)((int)pvVar5 + 0x28));
        }
        local_4._0_1_ = 1;
        if (pvVar5 != (void *)0x0) {
          FUN_004f57b0(local_14,pvVar5);
        }
      }
      puVar6 = (undefined4 *)FUN_004357b0(piVar1,local_28);
      local_4._0_1_ = 4;
      FUN_004f26d0(&local_30,puVar6);
      local_4._0_1_ = 1;
      FUN_00619730();
    }
    FUN_005f58b0(piVar1);
    sVar4 = FUN_005f50e0((int)local_14);
    if (sVar4 != 0) {
      bVar3 = false;
      pvVar5 = (void *)thunk_FUN_005f5060((int)local_14);
      while ((pvVar5 != (void *)0x0 && (!bVar3))) {
        puVar6 = (undefined4 *)FUN_00403040(pvVar5,local_28);
        iVar8 = param_1[0x1a];
        local_1c = &stack0xffffffbc;
        local_4._0_1_ = 5;
        FUN_004f26d0(&stack0xffffffbc,puVar6);
        pvVar7 = FUN_00403d30((void *)(iVar8 + 0x2c));
        local_4._0_1_ = 1;
        FUN_00619730();
        if ((pvVar7 != (void *)0x0) &&
           ((iVar8 = (**(code **)(*param_1 + 0x44))(), local_24 == iVar8 ||
            (*(int *)((int)pvVar7 + 0xbc) / 2 <= local_24)))) {
          puVar6 = (undefined4 *)FUN_00403040(pvVar5,&local_20);
          local_4._0_1_ = 6;
          FUN_004f26d0(param_1 + 0xd,puVar6);
          local_4._0_1_ = 1;
          FUN_00619730();
          bVar3 = true;
        }
        pvVar5 = *(void **)((int)pvVar5 + 0x10);
      }
      local_1c = (undefined1 *)0x90;
      local_18 = 0x98;
      if (((uint)param_1[0xd] >> 0x18 < 0x90) || (0x97 < (uint)param_1[0xd] >> 0x18)) {
        bVar3 = false;
      }
      else {
        bVar3 = true;
      }
      FUN_00619730();
      if (bVar3) {
        FUN_00482910((int)param_1);
      }
    }
  }
  bVar3 = true;
  local_1c = (undefined1 *)0x1;
  local_18 = 0xff;
  if (((uint)param_1[0xc] >> 0x18 == 0) || (0xfe < (uint)param_1[0xc] >> 0x18)) {
    bVar3 = false;
  }
  FUN_00619730();
  if (bVar3) {
    local_2c = 1;
    param_1[8] = param_1[8] | 0x10;
  }
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_004f55c0(local_14);
  ExceptionList = pvStack_c;
  return local_2c;
}

