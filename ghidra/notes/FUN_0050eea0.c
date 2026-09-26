
undefined4 FUN_0050eea0(void *param_1,void *param_2)

{
  void *pvVar1;
  void *pvVar2;
  uint *puVar3;
  int iVar4;
  void *pvVar5;
  void *this;
  uint local_50;
  undefined4 local_4c [7];
  int local_30;
  undefined4 local_2c [7];
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  pvVar1 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00642040;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  puVar3 = FUN_004025b0(param_1,(uint *)&param_1);
  local_4 = 0;
  iVar4 = FUN_00504dc0(puVar3);
  local_4 = 0xffffffff;
  FUN_00619730();
  pvVar2 = param_2;
  if (iVar4 != 0) {
    FUN_004f25c0(local_4c,iVar4,3);
    local_4 = 1;
    FUN_00513120((int)local_4c);
    while (local_30 != 0) {
      iVar4 = FUN_0052bed0((int)local_4c);
      if (*(short *)(iVar4 + 0x96) != 0) {
        pvVar5 = (void *)FUN_0052bed0((int)local_4c);
        FUN_004f9510(pvVar5,*(uint *)((int)pvVar1 + 0x24) >> 4 & 3,pvVar2);
      }
      FUN_005130d0((int)local_4c);
    }
    local_4 = 0xffffffff;
    FUN_004f26c0(local_4c);
  }
  FUN_004f25c0(local_2c,pvVar1,3);
  local_4._0_1_ = 2;
  local_4._1_3_ = 0;
  FUN_00513120((int)local_2c);
  while (local_10 != 0) {
    pvVar5 = (void *)FUN_0052bed0((int)local_2c);
    FUN_005130d0((int)local_2c);
    if (*(short *)((int)pvVar5 + 0x96) != 0) {
      puVar3 = FUN_004025b0(pvVar5,&local_50);
      local_4._0_1_ = 3;
      this = (void *)FUN_00505190(puVar3);
      local_4._0_1_ = 2;
      FUN_00619730();
      if (this != (void *)0x0) {
        FUN_004ece30(&param_1);
        local_4._0_1_ = 4;
        FUN_004ece30(&param_2);
        local_4 = CONCAT31(local_4._1_3_,5);
        FUN_004f6b70(this,(uint *)&param_1);
        FUN_004f6b70(pvVar5,(uint *)&param_2);
        iVar4 = FUN_004ece60((uint *)&param_1);
        if ((iVar4 == 0) || (param_1 != param_2)) {
          FUN_004f7f20(pvVar5,pvVar2);
        }
        else {
          FUN_004f9510(pvVar5,*(uint *)((int)pvVar1 + 0x24) >> 4 & 3,pvVar2);
        }
        local_4._0_1_ = 4;
        FUN_00619730();
        local_4._0_1_ = 2;
        FUN_00619730();
      }
    }
  }
  local_4 = 0xffffffff;
  FUN_004f26c0(local_2c);
  ExceptionList = pvStack_c;
  return 1;
}

