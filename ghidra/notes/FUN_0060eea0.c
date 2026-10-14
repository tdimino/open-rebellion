
undefined4 __thiscall FUN_0060eea0(int param_1,uint param_2)

{
  undefined4 uVar1;
  
  uVar1 = 0;
  if (param_2 < 0xe) {
    if (*(int *)(param_1 + param_2 * 4) == 0) {
      FUN_0060eed0(param_2);
    }
    uVar1 = *(undefined4 *)(param_1 + param_2 * 4);
  }
  return uVar1;
}

