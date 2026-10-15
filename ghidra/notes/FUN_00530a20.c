
undefined4 FUN_00530a20(uint *param_1)

{
  if ((*param_1 & 0xffffff) == 0xf) {
    return 1;
  }
  if ((*param_1 & 0xffffff) != 0x10) {
    return 0;
  }
  return 2;
}

