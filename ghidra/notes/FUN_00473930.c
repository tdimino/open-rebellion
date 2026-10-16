
undefined4 __thiscall FUN_00473930(int param_1,uint *param_2)

{
  uint uVar1;
  undefined4 uVar2;
  
  uVar2 = 0;
  uVar1 = *param_2;
  if ((uVar1 & 0x200) != 0) {
    return *(undefined4 *)(param_1 + 0x30);
  }
  if ((uVar1 & 0x80000) != 0) {
    return *(undefined4 *)(param_1 + 0x58);
  }
  if ((uVar1 & 0x400) != 0) {
    return *(undefined4 *)(param_1 + 0x34);
  }
  if ((uVar1 & 0x800) != 0) {
    return *(undefined4 *)(param_1 + 0x38);
  }
  if ((uVar1 & 0x80) != 0) {
    return *(undefined4 *)(param_1 + 0x44);
  }
  if ((uVar1 & 0x100) != 0) {
    return *(undefined4 *)(param_1 + 0x48);
  }
  if ((uVar1 & 1) != 0) {
    return *(undefined4 *)(param_1 + 0x20);
  }
  if ((uVar1 & 2) != 0) {
    return *(undefined4 *)(param_1 + 0x4c);
  }
  if ((uVar1 & 4) != 0) {
    return *(undefined4 *)(param_1 + 0x1c);
  }
  if ((uVar1 & 0x10000) != 0) {
    uVar2 = *(undefined4 *)(param_1 + 0x3c);
  }
  return uVar2;
}

