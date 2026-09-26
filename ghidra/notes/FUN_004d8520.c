
int * __fastcall FUN_004d8520(int param_1)

{
  bool bVar1;
  void *pvVar2;
  undefined4 *puVar3;
  int *piVar4;
  void *pvVar5;
  uint *puVar6;
  int iVar7;
  void *local_44;
  uint local_40 [2];
  int *local_38;
  undefined4 local_34 [2];
  undefined4 local_2c [7];
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063cc26;
  pvStack_c = ExceptionList;
  local_38 = (int *)0x0;
  ExceptionList = &pvStack_c;
  FUN_004f4340(local_34);
  local_4 = 0;
  local_44 = (void *)0x0;
  pvVar2 = FUN_00419a00(*(void **)(param_1 + 0x2c),0x10000,0,0,0,0,0,0x1d,(void *)0x1);
  pvVar5 = (void *)(param_1 + 0x40);
  FUN_00435790(pvVar5,(int)pvVar2);
  puVar3 = FUN_004357b0(pvVar5,local_40);
  puVar6 = (uint *)(param_1 + 0x54);
  local_4._0_1_ = 1;
  FUN_004f26d0(puVar6,puVar3);
  local_4._0_1_ = 0;
  FUN_00619730();
  FUN_005f58b0(pvVar5);
  pvVar2 = FUN_0041a340(*(void **)(param_1 + 0x2c),puVar6,0,0,1,0,0,0,0x25,(void *)0x1);
  FUN_00435790(pvVar5,(int)pvVar2);
  puVar3 = FUN_004357b0(pvVar5,local_40);
  local_4._0_1_ = 2;
  FUN_004f26d0(puVar6,puVar3);
  local_4._0_1_ = 0;
  FUN_00619730();
  FUN_005f58b0(pvVar5);
  local_40[0] = 0x90;
  local_40[1] = 0x98;
  if ((*puVar6 >> 0x18 < 0x90) || (0x97 < *puVar6 >> 0x18)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_00619730();
  if (!bVar1) {
    pvVar2 = FUN_00419a00(*(void **)(param_1 + 0x2c),0x200e0,0,0,0,0,0,3,(void *)0x1);
    FUN_00435790(pvVar5,(int)pvVar2);
    puVar3 = FUN_004357b0(pvVar5,local_40);
    local_4._0_1_ = 3;
    FUN_004f26d0(puVar6,puVar3);
    local_4._0_1_ = 0;
    FUN_00619730();
    FUN_005f58b0(pvVar5);
    pvVar2 = FUN_0041a340(*(void **)(param_1 + 0x2c),puVar6,0,0,1,0x3e00000,0,0x40000000,7,
                          (void *)0x1);
    FUN_00435790(pvVar5,(int)pvVar2);
    pvVar2 = FUN_0041a340(*(void **)(param_1 + 0x2c),puVar6,0,0,1,0x3e00000,0,0x40000000,0xb,
                          (void *)0x1);
    FUN_00435790(pvVar5,(int)pvVar2);
    puVar3 = FUN_004357b0(pvVar5,local_40);
    local_4._0_1_ = 4;
    FUN_004f26d0(puVar6,puVar3);
    local_4._0_1_ = 0;
    FUN_00619730();
    FUN_005f58b0(pvVar5);
  }
  local_40[0] = 0x90;
  local_40[1] = 0x98;
  if ((*puVar6 >> 0x18 < 0x90) || (0x97 < *puVar6 >> 0x18)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_00619730();
  if (!bVar1) {
    pvVar2 = FUN_00419a00(*(void **)(param_1 + 0x2c),0x20000,0,0,0,0,0,3,(void *)0x1);
    FUN_00435790(pvVar5,(int)pvVar2);
    puVar3 = FUN_004357b0(pvVar5,local_40);
    local_4._0_1_ = 5;
    FUN_004f26d0(puVar6,puVar3);
    local_4._0_1_ = 0;
    FUN_00619730();
    FUN_005f58b0(pvVar5);
    pvVar2 = FUN_0041a340(*(void **)(param_1 + 0x2c),puVar6,0,0,1,0,0,0x40000000,7,(void *)0x1);
    FUN_00435790(pvVar5,(int)pvVar2);
    pvVar2 = FUN_0041a340(*(void **)(param_1 + 0x2c),puVar6,0,0,1,0,0,0x40000000,0xb,(void *)0x1);
    FUN_00435790(pvVar5,(int)pvVar2);
    puVar3 = FUN_004357b0(pvVar5,local_40);
    local_4._0_1_ = 6;
    FUN_004f26d0(puVar6,puVar3);
    local_4._0_1_ = 0;
    FUN_00619730();
    FUN_005f58b0(pvVar5);
  }
  local_40[0] = 0x90;
  local_40[1] = 0x98;
  if ((*puVar6 >> 0x18 < 0x90) || (0x97 < *puVar6 >> 0x18)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_00619730();
  if ((bVar1) && (piVar4 = FUN_004f3220(*(int *)(param_1 + 0x30),puVar6), piVar4 != (int *)0x0)) {
    FUN_00527550(local_2c,piVar4,1);
    local_4._0_1_ = 7;
    FUN_00513120((int)local_2c);
    if (local_10 != 0) {
      pvVar5 = (void *)FUN_00618b70(0x20);
      local_4._0_1_ = 8;
      if (pvVar5 == (void *)0x0) {
        local_44 = (void *)0x0;
      }
      else {
        puVar6 = local_40;
        pvVar2 = (void *)FUN_0052bed0((int)local_2c);
        puVar6 = FUN_004025b0(pvVar2,puVar6);
        local_4 = CONCAT31(local_4._1_3_,9);
        local_44 = FUN_004f5b10(pvVar5,puVar6,0);
      }
      local_4 = 8;
      if (pvVar5 != (void *)0x0) {
        FUN_00619730();
      }
      local_4._0_1_ = 7;
      if (local_44 != (void *)0x0) {
        FUN_004f57b0(local_34,local_44);
      }
    }
    local_4._0_1_ = 0;
    FUN_00527650(local_2c);
    if (local_44 == (void *)0x0) {
      FUN_005272e0(local_2c,piVar4,1);
      local_4._0_1_ = 10;
      FUN_00513120((int)local_2c);
      if (local_10 != 0) {
        pvVar5 = (void *)FUN_00618b70(0x20);
        local_4._0_1_ = 0xb;
        if (pvVar5 == (void *)0x0) {
          local_44 = (void *)0x0;
        }
        else {
          puVar6 = local_40;
          pvVar2 = (void *)FUN_0052bed0((int)local_2c);
          puVar6 = FUN_004025b0(pvVar2,puVar6);
          local_4 = CONCAT31(local_4._1_3_,0xc);
          local_44 = FUN_004f5b10(pvVar5,puVar6,0);
        }
        local_4 = 0xb;
        if (pvVar5 != (void *)0x0) {
          FUN_00619730();
        }
        local_4._0_1_ = 10;
        if (local_44 != (void *)0x0) {
          FUN_004f57b0(local_34,local_44);
        }
      }
      local_4._0_1_ = 0;
      FUN_00527360(local_2c);
      if (local_44 == (void *)0x0) {
        FUN_005278c0(local_2c,piVar4,1);
        local_4._0_1_ = 0xd;
        FUN_00513120((int)local_2c);
        if (local_10 != 0) {
          pvVar5 = (void *)FUN_00618b70(0x20);
          local_4._0_1_ = 0xe;
          if (pvVar5 == (void *)0x0) {
            pvVar2 = (void *)0x0;
          }
          else {
            puVar6 = local_40;
            pvVar2 = (void *)FUN_0052bed0((int)local_2c);
            puVar6 = FUN_004025b0(pvVar2,puVar6);
            local_4 = CONCAT31(local_4._1_3_,0xf);
            pvVar2 = FUN_004f5b10(pvVar5,puVar6,0);
          }
          local_4 = 0xe;
          if (pvVar5 != (void *)0x0) {
            FUN_00619730();
          }
          local_4._0_1_ = 0xd;
          if (pvVar2 != (void *)0x0) {
            FUN_004f57b0(local_34,pvVar2);
          }
        }
        local_4._0_1_ = 0;
        FUN_005279c0(local_2c);
      }
    }
  }
  iVar7 = thunk_FUN_005f5060((int)local_34);
  piVar4 = local_38;
  if ((iVar7 != 0) && (piVar4 = (int *)FUN_004f5cd0(0x200), piVar4 != (int *)0x0)) {
    piVar4[8] = *(int *)(param_1 + 0x30);
    (**(code **)(*piVar4 + 0x24))(local_34);
  }
  local_4 = 0xffffffff;
  FUN_004f4380(local_34);
  ExceptionList = pvStack_c;
  return piVar4;
}

