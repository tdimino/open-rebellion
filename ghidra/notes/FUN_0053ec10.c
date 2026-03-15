
int __cdecl FUN_0053ec10(int param_1,int param_2,int param_3)

{
  bool bVar1;
  int iVar2;
  int iVar3;
  int iVar4;
  
  iVar4 = 0;
  if ((param_1 < 1) || (3 < param_1)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  if ((bVar1) && (param_1 != 3)) {
    iVar4 = param_3;
    iVar2 = param_2;
    iVar3 = DAT_006b9094;
    if ((param_1 != 1) && (iVar2 = param_3, iVar3 = param_3, param_1 == 2)) {
      iVar4 = param_2;
      iVar3 = DAT_006b9078;
    }
    iVar4 = (iVar4 - iVar2) / iVar3 + iVar4 / iVar3;
  }
  return iVar4;
}

