
int * __fastcall FUN_004d0610(int param_1)

{
  bool bVar1;
  int iVar2;
  undefined4 *puVar3;
  void *pvVar4;
  int *piVar5;
  uint local_28;
  void *local_24;
  int *local_20;
  undefined4 local_1c [2];
  undefined4 local_14;
  undefined4 local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063bd56;
  pvStack_c = ExceptionList;
  local_20 = (int *)0x0;
  ExceptionList = &pvStack_c;
  FUN_004f4340(local_1c);
  local_4 = 0;
  FUN_004ece30(&local_28);
  local_4._0_1_ = 1;
  iVar2 = FUN_0041b0d0(*(void **)(param_1 + 0x2c),0x802000,0,0xa0,0,9,0,1000,(void *)0x1);
  pvVar4 = (void *)(param_1 + 0x68);
  FUN_00435790(pvVar4,iVar2);
  puVar3 = FUN_004357b0(pvVar4,&local_24);
  local_4._0_1_ = 2;
  FUN_004f26d0(&local_28,puVar3);
  local_4._0_1_ = 1;
  FUN_00619730();
  FUN_005f58b0(pvVar4);
  local_14 = 0x3c;
  local_10 = 0x40;
  if ((local_28 >> 0x18 < 0x3c) || (local_24 = (void *)0x1, 0x3f < local_28 >> 0x18)) {
    local_24 = (void *)0x0;
  }
  FUN_00619730();
  if (local_24 == (void *)0x0) {
    iVar2 = FUN_0041b0d0(*(void **)(param_1 + 0x2c),0x2000,0,0xa0,0,9,0,1000,(void *)0x1);
    FUN_00435790(pvVar4,iVar2);
    puVar3 = FUN_004357b0(pvVar4,&local_24);
    local_4._0_1_ = 4;
    FUN_004f26d0(&local_28,puVar3);
    local_4._0_1_ = 1;
    FUN_00619730();
    FUN_005f58b0(pvVar4);
    local_14 = 0x3c;
    local_10 = 0x40;
    if ((local_28 >> 0x18 < 0x3c) || (0x3f < local_28 >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (!bVar1) goto LAB_004d07e7;
    local_24 = (void *)FUN_00618b70(0x20);
    local_4._0_1_ = 5;
    if (local_24 == (void *)0x0) {
      pvVar4 = (void *)0x0;
    }
    else {
      pvVar4 = FUN_004f5b10(local_24,&local_28,0);
    }
  }
  else {
    local_24 = (void *)FUN_00618b70(0x20);
    local_4._0_1_ = 3;
    if (local_24 == (void *)0x0) {
      pvVar4 = (void *)0x0;
    }
    else {
      pvVar4 = FUN_004f5b10(local_24,&local_28,0);
    }
  }
  local_4._0_1_ = 1;
  if (pvVar4 != (void *)0x0) {
    local_4._0_1_ = 1;
    FUN_004f57b0(local_1c,pvVar4);
  }
LAB_004d07e7:
  iVar2 = thunk_FUN_005f5060((int)local_1c);
  piVar5 = local_20;
  if ((iVar2 != 0) && (piVar5 = (int *)FUN_004f5cd0(0x242), piVar5 != (int *)0x0)) {
    piVar5[8] = *(int *)(param_1 + 0x30);
    (**(code **)(*piVar5 + 0x24))(local_1c);
  }
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_004f4380(local_1c);
  ExceptionList = pvStack_c;
  return piVar5;
}

