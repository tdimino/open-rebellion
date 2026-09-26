
int FUN_0054c200(undefined4 param_1,int param_2,int param_3,int param_4,void *param_5)

{
  void *pvVar1;
  bool bVar2;
  void *pvVar3;
  uint *puVar4;
  undefined3 extraout_var;
  int iVar5;
  undefined3 extraout_var_00;
  undefined3 extraout_var_01;
  int iVar6;
  int *local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  pvVar1 = param_5;
  puStack_8 = &LAB_00648050;
  pvStack_c = ExceptionList;
  iVar6 = 1;
  local_14 = 1;
  local_10 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &pvStack_c;
  FUN_00520580(param_5,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  local_18 = (int *)0x0;
  pvVar3 = (void *)thunk_FUN_005f5060(param_2);
  while ((pvVar3 != (void *)0x0 && (*(int *)((int)pvVar1 + 4) == -1))) {
    local_18 = (int *)0x0;
    puVar4 = FUN_00403040(pvVar3,&param_2);
    local_4 = 1;
    bVar2 = FUN_00582c30(puVar4,&local_18);
    if ((CONCAT31(extraout_var,bVar2) == 0) || (iVar6 == 0)) {
      iVar6 = 0;
    }
    else {
      iVar6 = 1;
    }
    local_4 = 0xffffffff;
    FUN_00619730();
    if (local_18 != (int *)0x0) {
      iVar5 = (**(code **)(*local_18 + 0x1cc))(param_1,0,pvVar1);
      if ((iVar5 == 0) || (iVar6 == 0)) {
        iVar6 = 0;
      }
      else {
        iVar6 = 1;
      }
    }
    pvVar3 = *(void **)((int)pvVar3 + 0x10);
  }
  pvVar3 = (void *)thunk_FUN_005f5060(param_3);
  while ((pvVar3 != (void *)0x0 && (*(int *)((int)param_5 + 4) == -1))) {
    local_18 = (int *)0x0;
    puVar4 = FUN_00403040(pvVar3,&param_2);
    local_4 = 2;
    bVar2 = FUN_00582c30(puVar4,&local_18);
    if ((CONCAT31(extraout_var_00,bVar2) == 0) || (iVar6 == 0)) {
      iVar6 = 0;
    }
    else {
      iVar6 = 1;
    }
    local_4 = 0xffffffff;
    FUN_00619730();
    if (local_18 != (int *)0x0) {
      iVar5 = (**(code **)(*local_18 + 0x1cc))(param_1,1,param_5);
      if ((iVar5 == 0) || (iVar6 == 0)) {
        iVar6 = 0;
      }
      else {
        iVar6 = 1;
      }
    }
    pvVar3 = *(void **)((int)pvVar3 + 0x10);
  }
  pvVar3 = (void *)thunk_FUN_005f5060(param_4);
  while ((pvVar3 != (void *)0x0 && (*(int *)((int)param_5 + 4) == -1))) {
    local_18 = (int *)0x0;
    puVar4 = FUN_00403040(pvVar3,&param_2);
    local_4 = 3;
    bVar2 = FUN_00582c30(puVar4,&local_18);
    if ((CONCAT31(extraout_var_01,bVar2) == 0) || (iVar6 == 0)) {
      iVar6 = 0;
    }
    else {
      iVar6 = 1;
    }
    local_4 = 0xffffffff;
    FUN_00619730();
    if (local_18 != (int *)0x0) {
      iVar5 = (**(code **)(*local_18 + 0x1cc))(param_1,0,param_5);
      if ((iVar5 == 0) || (iVar6 == 0)) {
        iVar6 = 0;
      }
      else {
        iVar6 = 1;
      }
    }
    pvVar3 = *(void **)((int)pvVar3 + 0x10);
  }
  ExceptionList = pvStack_c;
  return iVar6;
}

