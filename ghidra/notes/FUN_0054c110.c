
int FUN_0054c110(undefined4 param_1,int param_2,int *param_3)

{
  int *piVar1;
  bool bVar2;
  void *this;
  uint *puVar3;
  undefined3 extraout_var;
  int iVar4;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  piVar1 = param_3;
  puStack_8 = &LAB_00648020;
  local_c = ExceptionList;
  iVar4 = 1;
  local_14 = 1;
  local_10 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_3,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  this = (void *)thunk_FUN_005f5060(param_2);
  while ((this != (void *)0x0 && (piVar1[1] == -1))) {
    param_3 = (int *)0x0;
    puVar3 = FUN_00403040(this,&param_2);
    local_4 = 1;
    bVar2 = FUN_00582c30(puVar3,&param_3);
    if ((CONCAT31(extraout_var,bVar2) == 0) || (iVar4 == 0)) {
      iVar4 = 0;
    }
    else {
      iVar4 = 1;
    }
    local_4 = 0xffffffff;
    FUN_00619730();
    if (param_3 != (int *)0x0) {
      iVar4 = (**(code **)(*param_3 + 0x1c8))(param_1,piVar1);
    }
    this = *(void **)((int)this + 0x10);
  }
  ExceptionList = local_c;
  return iVar4;
}

