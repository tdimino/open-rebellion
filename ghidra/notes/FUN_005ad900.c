
bool FUN_005ad900(uint param_1)

{
  void *this;
  int iVar1;
  
  this = (void *)FUN_00596bd0();
  iVar1 = FUN_005a6460(this,param_1);
  if (iVar1 == 0) {
    return false;
  }
  iVar1 = (**(code **)(*(int *)(iVar1 + 0x44) + 0x10))();
  return iVar1 != 0;
}

