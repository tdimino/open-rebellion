
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

float10 __fastcall FUN_005b1680(int *param_1)

{
  float10 fVar1;
  
  if ((float)param_1[0xf9] == _DAT_0066c4f0) {
    return (float10)_DAT_0066c4f0;
  }
  fVar1 = (float10)(**(code **)(*param_1 + 0x60))();
  return fVar1 / (float10)(float)param_1[0xf9];
}

