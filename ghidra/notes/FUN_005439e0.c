
undefined4 FUN_005439e0(int param_1)

{
  int iVar1;
  
  iVar1 = FUN_005439b0(param_1);
  if ((iVar1 != 0) && ((*(byte *)(param_1 + 0x78) & 0x80) == 0)) {
    return 1;
  }
  return 0;
}

