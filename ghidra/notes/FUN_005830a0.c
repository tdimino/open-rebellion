
int __cdecl FUN_005830a0(int param_1,int param_2,int param_3,uint *param_4,void *param_5)

{
  uint uVar1;
  void *this;
  bool bVar2;
  void *pvVar3;
  uint *puVar4;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  undefined3 extraout_var_01;
  int iVar5;
  int *local_2c;
  undefined4 local_28;
  undefined4 local_24;
  undefined4 local_20 [5];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064f098;
  local_c = ExceptionList;
  iVar5 = 1;
  ExceptionList = &local_c;
  pvVar3 = (void *)FUN_00568fe0(local_20);
  local_4 = 0;
  FUN_00582b60(param_4,pvVar3);
  local_4 = 0xffffffff;
  FUN_00619730();
  this = param_5;
  local_28 = 1;
  local_24 = 0xffffffff;
  local_4 = 1;
  FUN_00520580(param_5,&local_28);
  local_4 = 0xffffffff;
  FUN_00619730();
  pvVar3 = (void *)thunk_FUN_005f5060(param_1);
  while (((pvVar3 != (void *)0x0 && (iVar5 != 0)) && (*(int *)((int)this + 4) == -1))) {
    puVar4 = FUN_00403040(pvVar3,&param_5);
    local_4 = 2;
    bVar2 = FUN_00582c30(puVar4,&local_2c);
    local_4 = 0xffffffff;
    FUN_00619730();
    iVar5 = 0;
    if (CONCAT31(extraout_var,bVar2) != 0) {
      iVar5 = FUN_00582fb0(local_2c,0,0,param_4,this);
    }
    pvVar3 = *(void **)((int)pvVar3 + 0x10);
  }
  pvVar3 = (void *)thunk_FUN_005f5060(param_2);
  while (((pvVar3 != (void *)0x0 && (iVar5 != 0)) && (*(int *)((int)this + 4) == -1))) {
    puVar4 = FUN_00403040(pvVar3,&param_5);
    local_4 = 3;
    bVar2 = FUN_00582c30(puVar4,&local_2c);
    local_4 = 0xffffffff;
    FUN_00619730();
    iVar5 = 0;
    if (CONCAT31(extraout_var_00,bVar2) != 0) {
      iVar5 = FUN_00582fb0(local_2c,1,0,param_4,this);
    }
    pvVar3 = *(void **)((int)pvVar3 + 0x10);
  }
  for (pvVar3 = (void *)thunk_FUN_005f5060(param_3); pvVar3 != (void *)0x0;
      pvVar3 = *(void **)((int)pvVar3 + 0x10)) {
    if (iVar5 == 0) {
      ExceptionList = local_c;
      return 0;
    }
    if (*(int *)((int)this + 4) != -1) break;
    puVar4 = FUN_00403040(pvVar3,&param_5);
    local_4 = 4;
    bVar2 = FUN_00582c30(puVar4,&local_2c);
    local_4 = 0xffffffff;
    FUN_00619730();
    iVar5 = 0;
    if (CONCAT31(extraout_var_01,bVar2) != 0) {
      iVar5 = FUN_00582fb0(local_2c,1,0,param_4,this);
    }
  }
  if (iVar5 == 0) {
    ExceptionList = local_c;
    return 0;
  }
  if (*(int *)((int)this + 4) != -1) {
    ExceptionList = local_c;
    return iVar5;
  }
  uVar1 = param_4[2];
  if (uVar1 != 0) {
    if (param_4[3] != 0) {
      local_28 = 1;
      local_24 = 0x14;
      local_4 = 5;
      FUN_00520580(this,&local_28);
      local_4 = 0xffffffff;
      FUN_00619730();
      ExceptionList = local_c;
      return iVar5;
    }
    if (uVar1 != 0) goto LAB_005832f8;
  }
  if (param_4[3] == 0) {
    local_28 = 1;
    local_24 = 0x16;
    local_4 = 6;
    FUN_00520580(this,&local_28);
    local_4 = 0xffffffff;
    FUN_00619730();
    ExceptionList = local_c;
    return iVar5;
  }
  if (uVar1 == 0) {
    param_4[4] = 2;
    ExceptionList = local_c;
    return iVar5;
  }
LAB_005832f8:
  param_4[4] = 1;
  ExceptionList = local_c;
  return iVar5;
}

