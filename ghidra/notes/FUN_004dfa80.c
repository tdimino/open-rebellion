
int * __fastcall FUN_004dfa80(int param_1)

{
  bool bVar1;
  void *pvVar2;
  undefined4 *puVar3;
  int *piVar4;
  int iVar5;
  uint *puVar6;
  int iVar7;
  void *pvVar8;
  uint local_44;
  int *local_40;
  void *local_3c;
  undefined4 local_38;
  undefined4 local_34 [2];
  undefined4 local_2c [7];
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063d882;
  pvStack_c = ExceptionList;
  local_40 = (int *)0x0;
  ExceptionList = &pvStack_c;
  FUN_004f4340(local_34);
  local_4 = 0;
  pvVar2 = FUN_00419a00(*(void **)(param_1 + 0x2c),0x80000,0,0,0,0,0,0x1f,&DAT_00000002);
  pvVar8 = (void *)(param_1 + 0x40);
  FUN_00435790(pvVar8,(int)pvVar2);
  puVar3 = FUN_004357b0(pvVar8,&local_44);
  puVar6 = (uint *)(param_1 + 0x54);
  local_4._0_1_ = 1;
  FUN_004f26d0(puVar6,puVar3);
  local_4._0_1_ = 0;
  FUN_00619730();
  FUN_005f58b0(pvVar8);
  pvVar2 = FUN_0041a340(*(void **)(param_1 + 0x2c),puVar6,0x10000,0,1,0,0,0,0x13,&DAT_00000002);
  FUN_00435790(pvVar8,(int)pvVar2);
  puVar3 = FUN_004357b0(pvVar8,&local_44);
  local_4._0_1_ = 2;
  FUN_004f26d0(puVar6,puVar3);
  local_4._0_1_ = 0;
  FUN_00619730();
  FUN_005f58b0(pvVar8);
  local_3c = (void *)0x90;
  local_38 = 0x98;
  if ((*puVar6 >> 0x18 < 0x90) || (0x97 < *puVar6 >> 0x18)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_00619730();
  if ((bVar1) && (piVar4 = FUN_004f3220(*(int *)(param_1 + 0x30),puVar6), piVar4 != (int *)0x0)) {
    iVar7 = 100000;
    pvVar8 = (void *)0x0;
    FUN_00504c40(local_2c,piVar4,1);
    local_4._0_1_ = 3;
    FUN_00513120((int)local_2c);
    while (local_10 != 0) {
      piVar4 = (int *)FUN_0052bed0((int)local_2c);
      iVar5 = (**(code **)(*piVar4 + 0x1e0))();
      if (iVar5 < iVar7) {
        piVar4 = (int *)FUN_0052bed0((int)local_2c);
        iVar7 = (**(code **)(*piVar4 + 0x1e0))();
        pvVar8 = (void *)FUN_0052bed0((int)local_2c);
      }
      FUN_005130d0((int)local_2c);
    }
    if (pvVar8 != (void *)0x0) {
      pvVar2 = (void *)FUN_00618b70(0x20);
      local_4._0_1_ = 4;
      local_3c = pvVar2;
      if (pvVar2 == (void *)0x0) {
        pvVar8 = (void *)0x0;
      }
      else {
        puVar6 = FUN_004025b0(pvVar8,&local_44);
        local_4 = CONCAT31(local_4._1_3_,5);
        pvVar8 = FUN_004f5b10(pvVar2,puVar6,0);
      }
      local_4 = 4;
      if (pvVar2 != (void *)0x0) {
        FUN_00619730();
      }
      local_4._0_1_ = 3;
      if (pvVar8 != (void *)0x0) {
        FUN_004f57b0(local_34,pvVar8);
      }
    }
    local_4._0_1_ = 0;
    FUN_00504d40(local_2c);
  }
  iVar7 = thunk_FUN_005f5060((int)local_34);
  piVar4 = local_40;
  if ((iVar7 != 0) && (piVar4 = (int *)FUN_004f5cd0(0x200), piVar4 != (int *)0x0)) {
    piVar4[8] = *(int *)(param_1 + 0x30);
    (**(code **)(*piVar4 + 0x24))(local_34);
  }
  local_4 = 0xffffffff;
  FUN_004f4380(local_34);
  ExceptionList = pvStack_c;
  return piVar4;
}

