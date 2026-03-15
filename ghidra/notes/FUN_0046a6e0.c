
void __fastcall FUN_0046a6e0(int param_1)

{
  if (*(int *)(param_1 + 300) != 0) {
    FUN_00610c80(param_1 + 0x128U);
    if ((*(uint *)(param_1 + 0x138) >> 5 & 1) != 0) {
      FUN_00610c90(param_1 + 0x128U,1);
    }
  }
  if (*(int *)(param_1 + 0x144) != 0) {
    FUN_00610c80(param_1 + 0x140U);
    if ((*(uint *)(param_1 + 0x150) >> 5 & 1) != 0) {
      FUN_00610c90(param_1 + 0x140U,1);
    }
  }
  return;
}

