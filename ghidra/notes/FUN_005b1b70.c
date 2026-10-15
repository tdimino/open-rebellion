
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __fastcall FUN_005b1b70(int *param_1)

{
  float10 fVar1;
  
  fVar1 = FUN_005b1bc0((int)param_1);
  if (fVar1 != (float10)_DAT_0066c4f0) {
    fVar1 = (float10)(**(code **)(*param_1 + 0x90))();
    if (fVar1 != (float10)_DAT_0066c4f0) {
      param_1[0x5b] = 1;
      return;
    }
  }
  param_1[0x5b] = 0;
  return;
}

