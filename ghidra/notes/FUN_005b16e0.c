
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

float10 __fastcall FUN_005b16e0(int param_1)

{
  float10 fVar1;
  
  if (*(float *)(param_1 + 0x3ec) == _DAT_0066c4f0) {
    return (float10)_DAT_0066c4f0;
  }
  fVar1 = FUN_005b1bc0(param_1);
  return fVar1 / (float10)*(float *)(param_1 + 0x3ec);
}

