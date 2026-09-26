
int __cdecl FUN_0055d8c0(short *param_1,short *param_2,int param_3)

{
  int iVar1;
  int iVar2;
  
  iVar2 = 0;
  iVar1 = FUN_0055d860(param_1,param_2);
  if (iVar1 != 0) {
    iVar2 = FUN_0053e190(iVar1 / DAT_006bb6e8,param_3);
    if (iVar2 == 0) {
      iVar2 = 1;
    }
  }
  return iVar2;
}

