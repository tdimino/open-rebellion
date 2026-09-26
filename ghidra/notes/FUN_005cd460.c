
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

float10 __thiscall FUN_005cd460(void *param_1,float param_2,float param_3)

{
  float fVar1;
  float10 fVar2;
  float10 fVar3;
  float10 fVar4;
  
  if (param_2 == _DAT_0066d028) {
    return (float10)param_2;
  }
  fVar2 = (float10)param_2;
  if (-*(float *)((int)param_1 + 0x18) * *(float *)((int)param_1 + 0x20) +
      *(float *)((int)param_1 + 0x28) * *(float *)((int)param_1 + 0x10) +
      *(float *)((int)param_1 + 0x24) * *(float *)((int)param_1 + 0x14) < _DAT_0066d028) {
    fVar2 = (float10)param_2 * (float10)_DAT_0066d024;
  }
  if (*(int *)((int)param_1 + 0xa8) == -0x40800000) {
    *(float *)((int)param_1 + 0xa8) =
         SQRT(*(float *)((int)param_1 + 0x9c) * *(float *)((int)param_1 + 0x9c) +
              *(float *)((int)param_1 + 0xa4) * *(float *)((int)param_1 + 0xa4) +
              *(float *)((int)param_1 + 0xa0) * *(float *)((int)param_1 + 0xa0));
  }
  fVar4 = ((float10)*(float *)((int)param_1 + 0xa8) * (float10)param_3) /
          ((float10)param_2 * (float10)_DAT_0066d02c);
  if (fVar4 < (float10)_DAT_0066d020) {
    fVar4 = fVar4 * fVar2;
    *(undefined4 *)((int)param_1 + 0x1c) = 0xbf800000;
    fVar2 = (float10)fcos(fVar4);
    fVar3 = (float10)fsin(fVar4);
    fVar1 = *(float *)((int)param_1 + 0x18);
    *(float *)((int)param_1 + 0x18) =
         (float)((float10)*(float *)((int)param_1 + 0x10) * (float10)(float)fVar3 +
                fVar2 * (float10)*(float *)((int)param_1 + 0x18));
    *(float *)((int)param_1 + 0x10) =
         *(float *)((int)param_1 + 0x10) * (float)fVar2 - (float)fVar3 * fVar1;
    *(float *)((int)param_1 + 0x1c) =
         SQRT(*(float *)((int)param_1 + 0x10) * *(float *)((int)param_1 + 0x10) +
              *(float *)((int)param_1 + 0x18) * *(float *)((int)param_1 + 0x18) +
              *(float *)((int)param_1 + 0x14) * *(float *)((int)param_1 + 0x14));
    if (_DAT_0066d028 < *(float *)((int)param_1 + 0x1c)) {
      fVar1 = _DAT_0066d020 / *(float *)((int)param_1 + 0x1c);
      *(undefined4 *)((int)param_1 + 0x1c) = 0xbf800000;
      *(float *)((int)param_1 + 0x10) = *(float *)((int)param_1 + 0x10) * fVar1;
      *(float *)((int)param_1 + 0x14) = fVar1 * *(float *)((int)param_1 + 0x14);
      *(float *)((int)param_1 + 0x18) = fVar1 * *(float *)((int)param_1 + 0x18);
    }
    *(undefined4 *)((int)param_1 + 0x1c) = 0x3f800000;
    return fVar4;
  }
  *(undefined4 *)((int)param_1 + 0x10) = *(undefined4 *)((int)param_1 + 0x20);
  *(undefined4 *)((int)param_1 + 0x14) = *(undefined4 *)((int)param_1 + 0x24);
  *(undefined4 *)((int)param_1 + 0x18) = *(undefined4 *)((int)param_1 + 0x28);
  *(undefined4 *)((int)param_1 + 0x1c) = *(undefined4 *)((int)param_1 + 0x2c);
  return fVar2;
}

