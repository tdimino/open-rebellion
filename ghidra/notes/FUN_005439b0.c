
undefined4 FUN_005439b0(int param_1)

{
  if (((((*(byte *)(param_1 + 0xac) & 1) == 0) && ((*(uint *)(param_1 + 0x50) & 1) != 0)) &&
      ((*(uint *)(param_1 + 0x50) & 0x1000) == 0)) && ((*(byte *)(param_1 + 0x78) & 0x40) == 0)) {
    return 1;
  }
  return 0;
}

