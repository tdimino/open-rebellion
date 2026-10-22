
undefined4 FUN_005ae260(int param_1)

{
  void *this;
  int iVar1;
  uint uVar2;
  
  if (param_1 != 0) {
    uVar2 = *(uint *)(param_1 + 0x174);
    this = (void *)FUN_00596bd0();
    iVar1 = FUN_005a6480(this,uVar2);
    if ((iVar1 != 0) && ((*(int *)(iVar1 + 100) == 1 || (*(int *)(iVar1 + 0x68) == 1)))) {
      return 0;
    }
  }
  return 1;
}

