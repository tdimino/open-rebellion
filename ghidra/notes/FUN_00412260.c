
undefined4 __thiscall FUN_00412260(int param_1,undefined4 param_2,undefined4 *param_3)

{
  *param_3 = 0;
  if (0 < *(int *)(param_1 + 0x50)) {
    *(int *)(param_1 + 0x50) = *(int *)(param_1 + 0x50) + -1;
    return 0;
  }
  return 1;
}

