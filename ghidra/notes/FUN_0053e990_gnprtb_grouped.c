
undefined4 __cdecl FUN_0053e990(int param_1,int *param_2)

{
  bool bVar1;
  int iVar2;
  undefined3 extraout_var;
  int iVar3;
  
  *param_2 = 0;
  iVar2 = FUN_0053e130(DAT_006b9080,100 - param_1);
  bVar1 = FUN_0053e2f0(iVar2);
  iVar3 = DAT_006b90d8;
  if (CONCAT31(extraout_var,bVar1) != 0) {
    iVar2 = FUN_0053e290(iVar2);
    iVar3 = FUN_0053e290(iVar3);
    *param_2 = iVar3 + iVar2 + DAT_006b908c;
  }
  return 1;
}

