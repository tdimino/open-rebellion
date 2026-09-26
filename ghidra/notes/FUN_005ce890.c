
int __thiscall FUN_005ce890(void *this,undefined4 param_1)

{
  int iVar1;
  void **ppvVar2;
  bool bVar3;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  undefined **local_28;
  undefined4 local_24;
  undefined4 local_20;
  undefined4 local_1c;
  undefined4 local_18;
  undefined2 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_006544c8;
  local_28 = &PTR_FUN_0066d04c;
  local_24 = 0;
  local_20 = 0;
  local_1c = 0;
  local_18 = 0;
  local_14 = 0;
  iVar1 = *(int *)((int)this + 4);
  local_4 = 0;
  ppvVar2 = &local_c;
  local_c = ExceptionList;
  local_10 = param_1;
  while( true ) {
    ExceptionList = ppvVar2;
    if (iVar1 == 0) {
      local_4 = 0xffffffff;
      FUN_005ee360(&local_28);
      ExceptionList = local_c;
      return 0;
    }
    bVar3 = FUN_005ee3c0(&local_28,iVar1);
    if (CONCAT31(extraout_var,bVar3) != 0) break;
    bVar3 = FUN_005ee370(&local_28,iVar1);
    ppvVar2 = ExceptionList;
    if (CONCAT31(extraout_var_00,bVar3) == 0) {
      iVar1 = *(int *)(iVar1 + 8);
    }
    else {
      iVar1 = *(int *)(iVar1 + 4);
    }
  }
  local_4 = 0xffffffff;
  FUN_005ee360(&local_28);
  ExceptionList = local_c;
  return iVar1;
}

