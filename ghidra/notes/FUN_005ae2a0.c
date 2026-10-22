
undefined4 FUN_005ae2a0(int param_1)

{
  int iVar1;
  undefined4 uVar2;
  
  uVar2 = 1;
  if (param_1 != 0) {
    iVar1 = FUN_005ad870(param_1);
    if ((((iVar1 == 0) || (iVar1 = *(int *)(param_1 + 0x170), iVar1 == 8)) || (iVar1 == 6)) ||
       ((iVar1 == 7 || (iVar1 == 10)))) {
      uVar2 = 0;
    }
  }
  return uVar2;
}

