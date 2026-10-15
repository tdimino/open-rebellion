
uint __fastcall FUN_004f63d0(int param_1)

{
  uint uVar1;
  
  uVar1 = 0;
  if (*(int *)(param_1 + 0x2c) != 0) {
    uVar1 = *(uint *)(*(int *)(param_1 + 0x2c) + 0x30) & 0xfff;
  }
  return uVar1;
}

