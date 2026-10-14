
undefined4 __thiscall FUN_004ed310(int param_1,undefined4 *param_2)

{
  int iVar1;
  
  iVar1 = FUN_004ed220();
  if (iVar1 != 0) {
    iVar1 = FUN_0053eb90((int)*(short *)(param_1 + 0x8c));
    if (iVar1 != 0) {
      *param_2 = 1;
      return 1;
    }
  }
  *param_2 = 0;
  return 1;
}

