
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

float10 __fastcall FUN_005b1bc0(int param_1)

{
  if ((float10)*(uint *)(param_1 + 0x374) < (float10)*(float *)(param_1 + 0x3ec)) {
    return (float10)*(float *)(param_1 + 0x3ec) - (float10)*(uint *)(param_1 + 0x374);
  }
  return (float10)_DAT_0066c4f0;
}

