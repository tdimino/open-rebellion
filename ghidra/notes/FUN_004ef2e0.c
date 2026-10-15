
undefined4 __thiscall FUN_004ef2e0(int param_1,undefined4 param_2,undefined4 param_3)

{
  undefined4 uVar1;
  int iStack_4;
  
  uVar1 = 1;
  if (((*(byte *)(param_1 + 0x78) & 0x80) == 0) && ((*(byte *)(param_1 + 0x50) & 8) == 0)) {
    iStack_4 = 0;
    FUN_004f6b50(&iStack_4);
    if ((iStack_4 != 0) && (*(char *)(iStack_4 + 0x40) == '\x14')) {
      uVar1 = FUN_004f78e0(1,param_3);
      return uVar1;
    }
    uVar1 = FUN_00534790(1,param_3);
  }
  return uVar1;
}

