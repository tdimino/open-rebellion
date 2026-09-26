
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

float10 __fastcall FUN_005b0400(void *param_1)

{
  float10 fVar1;
  
  fVar1 = FUN_005b1770(param_1,*(float *)((int)param_1 + 0x3e4));
  fVar1 = (float10)(float)fVar1 -
          (float10)*(float *)((int)param_1 + 0x3e4) * (float10)_DAT_0066c494 *
          (float10)*(uint *)((int)param_1 + 0x37c);
  if (fVar1 < (float10)_DAT_0066c484) {
    fVar1 = (float10)_DAT_0066c484;
  }
  return fVar1;
}

