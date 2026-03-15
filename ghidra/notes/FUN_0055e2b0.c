
int __cdecl FUN_0055e2b0(int param_1,int *param_2,int param_3,int param_4,int *param_5)

{
  int iVar1;
  int iVar2;
  
  *param_5 = 0;
  iVar1 = FUN_0040f340();
  if (iVar1 != 0) {
    iVar2 = FUN_0053e190(*param_2,DAT_006bb700);
    if (iVar2 == 0) {
      iVar2 = 1;
    }
    iVar2 = FUN_0053e160(param_2[1],iVar2);
    *param_5 = iVar2 + param_1;
    iVar2 = FUN_0053e190(iVar2 + param_1,param_3);
    *param_5 = iVar2;
    if (param_4 == 0) {
      iVar2 = FUN_0053e290(iVar2);
      *param_5 = iVar2 + *param_5 / 2;
    }
  }
  return iVar1;
}

