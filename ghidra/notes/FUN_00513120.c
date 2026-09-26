
int __fastcall FUN_00513120(int param_1)

{
  int iVar1;
  
  FUN_00513170(param_1);
  if (*(int *)(param_1 + 4) == 0) {
    *(undefined4 *)(param_1 + 0x1c) = 0;
  }
  else {
    iVar1 = FUN_00539f70(*(int *)(param_1 + 4));
    *(int *)(param_1 + 0x1c) = iVar1;
  }
  if (*(int *)(param_1 + 0x1c) == 0) {
    FUN_00513170(param_1);
    return param_1;
  }
  iVar1 = FUN_005131d0(param_1);
  if (iVar1 == 0) {
    FUN_005130d0(param_1);
  }
  return param_1;
}

