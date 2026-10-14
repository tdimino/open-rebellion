
undefined4 FUN_0058d020(int *param_1,undefined4 param_2)

{
  undefined4 uVar1;
  
  uVar1 = 1;
  if (*param_1 != 0) {
    uVar1 = FUN_0058ce70(*param_1,param_2);
    return uVar1;
  }
  if (param_1[1] != 0) {
    uVar1 = FUN_0058ce70(param_1[1],param_2);
    return uVar1;
  }
  if (param_1[2] != 0) {
    uVar1 = FUN_0058cf20(param_2);
    return uVar1;
  }
  if (param_1[3] != 0) {
    uVar1 = FUN_0058cf50(param_2);
  }
  return uVar1;
}

