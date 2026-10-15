
void __fastcall FUN_00479a00(int param_1)

{
  if ((*(uint *)(param_1 + 100) & 0x4ff) == 0) {
    *(uint *)(param_1 + 0x60) = *(uint *)(param_1 + 0x60) | 0x1c0000;
  }
  else {
    if ((*(uint *)(param_1 + 100) & 0x9fe00000) == 0) {
      *(uint *)(param_1 + 0x60) = *(uint *)(param_1 + 0x60) | 0x10000;
      return;
    }
    *(uint *)(param_1 + 0x60) = *(uint *)(param_1 + 0x60) | 0x100000;
    if ((*(byte *)(param_1 + 100) & 0x1f) == 0) {
      *(uint *)(param_1 + 0x60) = *(uint *)(param_1 + 0x60) | 0x80000;
      return;
    }
  }
  return;
}

