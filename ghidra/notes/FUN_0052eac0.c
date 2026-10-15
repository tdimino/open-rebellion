
undefined4 __thiscall FUN_0052eac0(int param_1,int param_2,undefined4 *param_3)

{
  undefined4 uVar1;
  
  uVar1 = 1;
  *param_3 = 0;
  if (param_2 == 0) {
    *param_3 = *(undefined4 *)(param_1 + 0x9c);
    return uVar1;
  }
  if (param_2 != 1) {
    if (param_2 != 2) {
      return 0;
    }
    *param_3 = *(undefined4 *)(param_1 + 0xa0);
    return uVar1;
  }
  *param_3 = *(undefined4 *)(param_1 + 0xa4);
  return uVar1;
}

