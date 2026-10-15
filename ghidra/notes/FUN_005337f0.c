
undefined4 __thiscall FUN_005337f0(int *param_1,int param_2,undefined4 *param_3)

{
  undefined4 uVar1;
  
  *param_3 = 0;
  if (param_2 == 0) {
    uVar1 = (**(code **)(*param_1 + 0x1e4))();
    *param_3 = uVar1;
    return 1;
  }
  if (param_2 != 1) {
    if (param_2 != 2) {
      return 0;
    }
    uVar1 = (**(code **)(*param_1 + 0x1e8))();
    *param_3 = uVar1;
    return 1;
  }
  uVar1 = (**(code **)(*param_1 + 0x1ec))();
  *param_3 = uVar1;
  return 1;
}

