
bool FUN_005ad8d0(undefined4 param_1)

{
  int iVar1;
  
  FUN_00596bd0(param_1);
  iVar1 = FUN_005a6470(param_1);
  if (iVar1 == 0) {
    return false;
  }
  iVar1 = (**(code **)(*(int *)(iVar1 + 0x44) + 0x10))();
  return iVar1 != 0;
}

