
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __thiscall FUN_005b5700(int param_1,float param_2)

{
  float fVar1;
  
  if (*(float *)(param_1 + 0x3cc) <= _DAT_0066c4f0) {
    *(undefined4 *)(param_1 + 0x3cc) = 0;
    return;
  }
  param_2 = *(float *)(param_1 + 0x3cc) - param_2;
  *(undefined4 *)(param_1 + 0x3c0) = 0;
  fVar1 = _DAT_0066c4f0;
  *(float *)(param_1 + 0x3cc) = param_2;
  if (param_2 <= fVar1) {
    *(undefined4 *)(param_1 + 0x3cc) = 0;
  }
  return;
}

