
int __fastcall FUN_0043a250(int *param_1)

{
  int iVar1;
  
  if (0 < param_1[100]) {
    param_1[100] = param_1[100] + -1;
    iVar1 = FUN_0041cd80(6);
    return iVar1 / 2 + 1;
  }
  param_1[100] = 3;
  iVar1 = (**(code **)(*param_1 + 0x1c))();
  return iVar1;
}

