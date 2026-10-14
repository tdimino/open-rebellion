
undefined4 FUN_005888a0(int param_1,undefined4 *param_2)

{
  bool bVar1;
  undefined3 extraout_var;
  
  bVar1 = FUN_0055e570((int)*(short *)(param_1 + 0x8c));
  if (((CONCAT31(extraout_var,bVar1) != 0) && ((*(byte *)(param_1 + 0x78) & 0xc) == 0)) &&
     ((*(byte *)(param_1 + 0xac) & 1) == 0)) {
    *param_2 = 1;
    return 1;
  }
  *param_2 = 0;
  return 1;
}

