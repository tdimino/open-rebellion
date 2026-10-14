
undefined4 FUN_0055ca20(int param_1,int param_2,int *param_3)

{
  undefined4 uVar1;
  int iVar2;
  
  *param_3 = param_1;
  if (param_1 < param_2) {
    uVar1 = FUN_0053e190(param_2 - param_1,DAT_006bb528);
    iVar2 = FUN_0053e290(uVar1);
    *param_3 = *param_3 + iVar2;
  }
  return 1;
}

