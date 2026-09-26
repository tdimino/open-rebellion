
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

float10 __fastcall FUN_005b16b0(int *param_1)

{
  float10 fVar1;
  
  if ((float)param_1[0xfe] == _DAT_0066c4f0) {
    return (float10)_DAT_0066c4f0;
  }
  fVar1 = (float10)(**(code **)(*param_1 + 0x90))();
  return fVar1 / (float10)(float)param_1[0xfe];
}

