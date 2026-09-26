
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

float * __thiscall FUN_005eb040(int param_1,float *param_2,int param_3)

{
  float fVar1;
  int iVar2;
  float fStack_10;
  float fStack_8;
  
  iVar2 = FUN_005c61d0((void *)(param_1 + 0xec),*(int *)(param_3 + 0xec));
  if (iVar2 != 0) {
    *param_2 = *(float *)(iVar2 + 0x50);
    param_2[1] = *(float *)(iVar2 + 0x54);
    param_2[2] = *(float *)(iVar2 + 0x58);
    param_2[3] = *(float *)(iVar2 + 0x5c);
    return param_2;
  }
  iVar2 = FUN_005c61d0((void *)(param_1 + 0xe0),*(int *)(param_3 + 0xec));
  fStack_10 = 0.0;
  fStack_8 = 0.0;
  fVar1 = _DAT_0066d874;
  if (iVar2 != 0) {
    fVar1 = (*(float *)(param_1 + 0x40) - *(float *)(iVar2 + 0x54)) - *(float *)(param_3 + 0x34);
    fStack_8 = (*(float *)(param_1 + 0x44) - *(float *)(iVar2 + 0x58)) - *(float *)(param_3 + 0x38);
    fStack_10 = (*(float *)(param_1 + 0x3c) - *(float *)(iVar2 + 0x50)) - *(float *)(param_3 + 0x30)
    ;
    if (fStack_10 * fStack_10 + fStack_8 * fStack_8 + fVar1 * fVar1 <= _DAT_0066d850) {
      *param_2 = DAT_006bc940;
      param_2[1] = DAT_006bc944;
      param_2[2] = DAT_006bc948;
      param_2[3] = DAT_006bc94c;
      return param_2;
    }
  }
  *param_2 = fStack_10;
  param_2[2] = fStack_8;
  param_2[3] = -1.0;
  param_2[1] = fVar1;
  return param_2;
}

