
undefined4 __thiscall FUN_00533850(int param_1,int param_2,int *param_3)

{
  undefined4 uVar1;
  
  uVar1 = 1;
  *param_3 = 0;
  if (param_2 == 0) {
    *param_3 = (int)*(short *)(param_1 + 0x5c);
    return uVar1;
  }
  if (param_2 != 1) {
    if (param_2 != 2) {
      return 0;
    }
    *param_3 = (int)*(short *)(param_1 + 0x5e);
    return uVar1;
  }
  *param_3 = (int)*(short *)(param_1 + 0x60);
  return uVar1;
}

