
uint FUN_005425e0(int *param_1,uint *param_2,uint param_3,uint param_4,int *param_5,void *param_6)

{
  void *pvVar1;
  bool bVar2;
  undefined3 extraout_var;
  int iVar3;
  void *pvVar4;
  uint *puVar5;
  undefined3 extraout_var_00;
  void *pvVar6;
  uint local_40;
  int local_3c;
  undefined4 local_38;
  int local_34 [2];
  int local_2c [2];
  int local_24 [2];
  int local_1c [2];
  undefined4 local_14;
  undefined4 local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  pvVar1 = param_6;
  puStack_8 = &LAB_006470f0;
  pvStack_c = ExceptionList;
  local_3c = 1;
  local_38 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &pvStack_c;
  FUN_00520580(param_6,&local_3c);
  local_4 = 0xffffffff;
  FUN_00619730();
  FUN_004f4340(local_24);
  local_4 = 1;
  FUN_004f4340(local_2c);
  local_4._0_1_ = 2;
  FUN_004f4340(local_1c);
  local_4._0_1_ = 3;
  bVar2 = FUN_0054bb90(param_1,param_3,param_4,local_24,local_2c,local_1c,param_5,pvVar1);
  pvVar4 = (void *)(uint)(CONCAT31(extraout_var,bVar2) != 0);
  param_6 = pvVar4;
  FUN_004ece30(&local_40);
  local_4._0_1_ = 4;
  pvVar6 = pvVar4;
  if ((*(int *)((int)pvVar1 + 4) == -1) &&
     ((iVar3 = FUN_0054bf00((int)param_5,&local_40,pvVar1), iVar3 == 0 ||
      (param_6 = (void *)0x1, pvVar6 = param_6, pvVar4 == (void *)0x0)))) {
    param_6 = (void *)0x0;
    pvVar6 = param_6;
  }
  local_34[0] = 0;
  if (*(int *)((int)pvVar1 + 4) != -1) goto LAB_00542922;
  param_3 = 0x33000243;
  local_4._0_1_ = 5;
  pvVar4 = FUN_004f5940(local_24,&param_3);
  if (pvVar4 == (void *)0x0) {
    param_4 = 0x33000243;
    local_4._0_1_ = 6;
    pvVar4 = FUN_004f5940(local_2c,&param_4);
    local_4._0_1_ = 5;
    FUN_00619730();
    param_1 = (int *)0x0;
    if (pvVar4 != (void *)0x0) goto LAB_00542740;
  }
  else {
LAB_00542740:
    param_1 = (int *)0x1;
  }
  local_4._0_1_ = 4;
  FUN_00619730();
  iVar3 = 0;
  pvVar4 = (void *)thunk_FUN_005f5060((int)local_24);
  while ((pvVar4 != (void *)0x0 && (iVar3 == 0))) {
    puVar5 = (uint *)FUN_00403040(pvVar4,&param_4);
    local_14 = 0x3c;
    local_10 = 0x40;
    local_4._0_1_ = 7;
    if ((*puVar5 >> 0x18 < 0x3c) || (0x3f < *puVar5 >> 0x18)) {
      iVar3 = 0;
    }
    else {
      iVar3 = 1;
    }
    FUN_00619730();
    local_4._0_1_ = 4;
    FUN_00619730();
    pvVar4 = *(void **)((int)pvVar4 + 0x10);
  }
  pvVar4 = (void *)thunk_FUN_005f5060((int)local_2c);
  while ((pvVar4 != (void *)0x0 && (iVar3 == 0))) {
    puVar5 = (uint *)FUN_00403040(pvVar4,&param_4);
    local_14 = 0x3c;
    local_10 = 0x40;
    local_4._0_1_ = 8;
    if ((*puVar5 >> 0x18 < 0x3c) || (0x3f < *puVar5 >> 0x18)) {
      iVar3 = 0;
    }
    else {
      iVar3 = 1;
    }
    FUN_00619730();
    local_4._0_1_ = 4;
    FUN_00619730();
    pvVar4 = *(void **)((int)pvVar4 + 0x10);
  }
  pvVar4 = (void *)thunk_FUN_005f5060((int)local_1c);
  while ((pvVar4 != (void *)0x0 && (iVar3 == 0))) {
    puVar5 = (uint *)FUN_00403040(pvVar4,&param_4);
    local_14 = 0x3c;
    local_10 = 0x40;
    local_4._0_1_ = 9;
    if ((*puVar5 >> 0x18 < 0x3c) || (0x3f < *puVar5 >> 0x18)) {
      iVar3 = 0;
    }
    else {
      iVar3 = 1;
    }
    FUN_00619730();
    local_4._0_1_ = 4;
    FUN_00619730();
    pvVar4 = *(void **)((int)pvVar4 + 0x10);
  }
  local_3c = DAT_006b9050;
  bVar2 = FUN_00542990((int)param_1,iVar3,&local_3c);
  if ((CONCAT31(extraout_var_00,bVar2) == 0) || (param_6 == (void *)0x0)) {
    bVar2 = false;
  }
  else {
    bVar2 = true;
  }
  iVar3 = FUN_00506e60();
  if ((iVar3 == 0) || (param_6 = (void *)0x1, !bVar2)) {
    param_6 = (void *)0x0;
  }
  pvVar6 = param_6;
  if ((iVar3 != 0) &&
     ((iVar3 = FUN_00555b30(local_3c,0,&local_40,param_2,local_34), pvVar4 = param_6, iVar3 == 0 ||
      (param_6 = (void *)0x1, pvVar6 = param_6, pvVar4 == (void *)0x0)))) {
    param_6 = (void *)0x0;
    pvVar6 = param_6;
  }
LAB_00542922:
  for (iVar3 = thunk_FUN_005f5060((int)param_5); iVar3 != 0; iVar3 = *(int *)(iVar3 + 0x10)) {
    *(int *)(iVar3 + 0x1c) = local_34[0];
  }
  local_4._0_1_ = 3;
  FUN_00619730();
  local_4._0_1_ = 2;
  FUN_004f4380(local_1c);
  local_4 = CONCAT31(local_4._1_3_,1);
  FUN_004f4380(local_2c);
  local_4 = 0xffffffff;
  FUN_004f4380(local_24);
  ExceptionList = pvStack_c;
  return (uint)pvVar6;
}

