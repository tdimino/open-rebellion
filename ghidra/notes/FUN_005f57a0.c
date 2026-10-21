
int FUN_005f57a0(int param_1)

{
  int iVar1;
  
  iVar1 = param_1 * 0xa18e + (param_1 / 0xcad4) * -0x7fffffff;
  if (iVar1 < 0) {
    iVar1 = iVar1 + 0x7fffffff;
  }
  return iVar1;
}

