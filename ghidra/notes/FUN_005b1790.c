
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

float10 __fastcall FUN_005b1790(void *param_1)

{
  float10 fVar1;
  
  fVar1 = FUN_005b1770(param_1,*(float *)((int)param_1 + 0x3b8));
  fVar1 = (float10)(float)fVar1 -
          (float10)*(float *)((int)param_1 + 0x3b8) * (float10)_DAT_0066c4f4 *
          (float10)*(uint *)((int)param_1 + 0x378);
  if (fVar1 < (float10)_DAT_0066c4f0) {
    fVar1 = (float10)_DAT_0066c4f0;
  }
  return fVar1;
}

