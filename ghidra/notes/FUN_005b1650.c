
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

float10 __fastcall FUN_005b1650(int param_1)

{
  if (*(float *)(param_1 + 0x3bc) == _DAT_0066c4f0) {
    return (float10)_DAT_0066c4f0;
  }
  return (float10)*(float *)(param_1 + 0x3c0) / (float10)*(float *)(param_1 + 0x3bc);
}

