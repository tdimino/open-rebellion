
undefined4 __cdecl FUN_0056c460(int *param_1,undefined4 *param_2)

{
  int iVar1;
  
  *param_2 = 0;
  iVar1 = *param_1;
  FUN_00619730();
  if (iVar1 == 0x53000020) {
    *param_2 = 0;
    return 1;
  }
  iVar1 = *param_1;
  FUN_00619730();
  if (iVar1 == 0x53000021) {
    *param_2 = 1;
    return 1;
  }
  iVar1 = *param_1;
  FUN_00619730();
  if (iVar1 == 0x53000022) {
    *param_2 = 2;
    return 1;
  }
  return 0;
}

