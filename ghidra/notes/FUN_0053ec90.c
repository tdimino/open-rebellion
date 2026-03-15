
int __cdecl FUN_0053ec90(int param_1,int param_2)

{
  bool bVar1;
  int iVar2;
  
  iVar2 = 0;
  if ((param_1 < 1) || (3 < param_1)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  if (((bVar1) && (param_1 != 3)) && (param_2 != 0)) {
    if (param_1 == 1) {
      iVar2 = FUN_0053e290(DAT_006b90a0);
      return iVar2 + DAT_006b90b4;
    }
    if (param_1 == 2) {
      iVar2 = FUN_0053e290(DAT_006b90b0);
      iVar2 = iVar2 + DAT_006b906c;
    }
  }
  return iVar2;
}

