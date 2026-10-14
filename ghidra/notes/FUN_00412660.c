
undefined4 __thiscall FUN_00412660(int param_1,undefined4 param_2,undefined4 *param_3)

{
  *param_3 = 0;
  if (*(int *)(param_1 + 0x54) != 0) {
    *(int *)(param_1 + 0x54) = *(int *)(param_1 + 0x54) + -1;
    return 0;
  }
  return 1;
}

