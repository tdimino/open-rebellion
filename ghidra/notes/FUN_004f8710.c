
void __thiscall FUN_004f8710(int param_1,undefined4 param_2,undefined4 param_3)

{
  uint uVar1;
  int iStack_4;
  
  iStack_4 = param_1;
  FUN_004f6b50(&iStack_4);
  uVar1 = *(uint *)(param_1 + 0x50);
  if (((uVar1 & 4) != 0) && ((uVar1 & 0x20) == 0)) {
    FUN_004f7ed0(param_3);
    return;
  }
  if ((uVar1 & 8) == 0) {
    FUN_004f78e0(1,param_3);
  }
  return;
}

