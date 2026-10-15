
uint __fastcall FUN_005092f0(int param_1)

{
  if (*(int *)(param_1 + 0x54) != 0) {
    return *(uint *)(*(int *)(param_1 + 0x54) + 0x80) & DAT_0066274c;
  }
  return 0;
}

