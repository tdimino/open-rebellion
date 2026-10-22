
void __fastcall FUN_005fc4d0(int *param_1)

{
  int iVar1;
  
  if (*param_1 == 0) {
    param_1[1] = 0;
    return;
  }
  iVar1 = FUN_005fc120(param_1);
  param_1[1] = iVar1 + param_1[4] * 4 + *param_1;
  return;
}

