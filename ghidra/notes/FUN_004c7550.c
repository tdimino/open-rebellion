
void __thiscall FUN_004c7550(int param_1,undefined4 param_2,undefined4 *param_3)

{
  *(undefined4 *)(param_1 + 0x20) = 1;
  *(undefined4 *)(param_1 + 0x34) = 0;
  *(undefined4 *)(param_1 + 0x38) = 0;
  *(undefined4 *)(param_1 + 0x30) = 0;
  if (param_3 != (undefined4 *)0x0) {
    (**(code **)*param_3)(1);
  }
  return;
}

