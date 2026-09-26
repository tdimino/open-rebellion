
undefined4 FUN_00561bb0(uint *param_1,int param_2,void *param_3)

{
  bool bVar1;
  bool bVar2;
  int *this;
  uint uVar3;
  int iVar4;
  uint *puVar5;
  uint *puVar6;
  uint uVar7;
  void *pvStack_30;
  uint local_2c;
  void *pvStack_28;
  void *pvStack_24;
  undefined4 uStack_20;
  int iStack_1c;
  uint local_18;
  uint local_14;
  uint local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064a32e;
  local_c = ExceptionList;
  bVar2 = false;
  local_2c = 0;
  ExceptionList = &local_c;
  this = (int *)FUN_00504dc0(param_1);
  if (this == (int *)0x0) {
    ExceptionList = local_c;
    return 0;
  }
  local_14 = 0x30;
  local_10 = 0x3c;
  local_4 = 0;
  uVar3 = (**(code **)(*this + 4))();
  if ((local_14 <= uVar3) && (uVar3 < local_10)) {
    bVar2 = true;
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  local_14 = 0x30;
  local_10 = 0x3c;
  uVar7 = 1;
  local_4 = 1;
  uVar3 = (**(code **)(*this + 4))();
  if ((uVar3 < local_14) || (local_10 <= uVar3)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  if (!bVar1) {
    ExceptionList = local_c;
    return 0;
  }
  param_1 = (uint *)0x0;
  iVar4 = FUN_0053ebb0(*(undefined4 *)(param_2 + 0xc),&param_1);
  if ((iVar4 == 0) || (!bVar2)) {
    bVar2 = false;
  }
  else {
    bVar2 = true;
  }
  if ((uint *)(int)*(short *)((int)this + 0x96) == param_1) {
    param_1 = (uint *)0x0;
  }
  FUN_004ece30(&uStack_20);
  local_4 = 2;
  if (param_1 == (uint *)0x0) goto LAB_00561e4b;
  iStack_1c = 0;
  pvStack_24 = (void *)0x0;
  pvStack_28 = (void *)0x0;
  pvStack_30 = (void *)0x0;
  iVar4 = FUN_0055ed30(this,&iStack_1c,(int *)&pvStack_28,&pvStack_24);
  if ((iVar4 == 0) || (!bVar2)) {
    bVar2 = false;
  }
  else {
    bVar2 = true;
  }
  if ((pvStack_24 == (void *)0x0) || (!bVar2)) {
    bVar2 = false;
  }
  else {
    bVar2 = true;
  }
  if (pvStack_24 != (void *)0x0) {
    if (pvStack_28 == (void *)0x0) {
      puVar5 = FUN_004025b0(pvStack_24,&local_18);
      uVar7 = 2;
      local_2c = 2;
      local_4 = 4;
    }
    else {
      puVar5 = FUN_004025b0(pvStack_28,&local_14);
      local_2c = 1;
      local_4 = CONCAT31(local_4._1_3_,3);
    }
    FUN_004f26d0(&uStack_20,puVar5);
    local_4 = 3;
    if ((uVar7 & 2) != 0) {
      uVar7 = uVar7 & 0xfffffffd;
      local_2c = uVar7;
      FUN_00619730();
    }
    local_4 = 2;
    if ((uVar7 & 1) != 0) {
      FUN_00619730();
    }
    if (pvStack_28 == (void *)0x0) {
      if (pvStack_24 != (void *)0x0) {
        iVar4 = FUN_00509330(pvStack_24,(int)param_1,(int *)&pvStack_30);
        if ((iVar4 == 0) || (!bVar2)) goto LAB_00561de2;
        local_2c = 1;
      }
    }
    else {
      iVar4 = FUN_004fd790(pvStack_28,(int)param_1,(int *)&pvStack_30);
      if ((iVar4 == 0) || (!bVar2)) {
LAB_00561de2:
        local_2c = 0;
      }
      else {
        local_2c = 1;
      }
    }
  }
  if (pvStack_30 != (void *)0x0) {
    puVar5 = FUN_004025b0(this,&local_18);
    local_4._0_1_ = 5;
    puVar6 = FUN_004025b0(pvStack_30,&local_14);
    uVar3 = *puVar6;
    uVar7 = *puVar5;
    FUN_00619730();
    local_4 = CONCAT31(local_4._1_3_,2);
    FUN_00619730();
    if (uVar3 != uVar7) {
      FUN_004ee350(pvStack_30,0,param_3);
    }
  }
LAB_00561e4b:
  (**(code **)(*this + 0x2f4))(param_2,&uStack_20,param_3);
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return 0;
}

