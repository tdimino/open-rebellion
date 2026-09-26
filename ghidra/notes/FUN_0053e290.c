
int __cdecl FUN_0053e290(int param_1)

{
  int iVar1;
  
  iVar1 = 0;
  if (DAT_006b9054 != (void *)0x0) {
    if (-1 < param_1) {
      iVar1 = FUN_005f5700(DAT_006b9054,param_1 + 1);
      return iVar1;
    }
    iVar1 = FUN_005f5700(DAT_006b9054,1 - param_1);
    iVar1 = -iVar1;
  }
  return iVar1;
}

