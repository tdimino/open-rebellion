
undefined4 __thiscall FUN_00503e20(int param_1,int param_2,undefined4 param_3)

{
  undefined4 uVar1;
  
  uVar1 = 1;
  if ((((*(uint *)(param_1 + 0x50) & 0x20) == 0) && (param_2 != 0)) &&
     ((*(uint *)(param_1 + 0x50) & 8) == 0)) {
    uVar1 = FUN_004f7ed0(param_3);
  }
  if ((((*(byte *)(param_1 + 0x78) & 0x80) == 0) && ((*(byte *)(param_1 + 0x50) & 8) == 0)) &&
     (param_2 == 0)) {
    uVar1 = FUN_004f78e0(1,param_3);
  }
  return uVar1;
}

