
undefined4 __thiscall FUN_00473250(int param_1,uint *param_2)

{
  uint uVar1;
  undefined4 uVar2;
  
  uVar2 = 0;
  uVar1 = *param_2;
  if ((uVar1 & 0x10000) != 0) {
    return *(undefined4 *)(param_1 + 0x34);
  }
  if ((uVar1 & 0x40000) != 0) {
    return *(undefined4 *)(param_1 + 0x30);
  }
  if ((uVar1 & 1) != 0) {
    return *(undefined4 *)(param_1 + 0x20);
  }
  if ((uVar1 & 4) != 0) {
    uVar2 = *(undefined4 *)(param_1 + 0x1c);
  }
  return uVar2;
}

