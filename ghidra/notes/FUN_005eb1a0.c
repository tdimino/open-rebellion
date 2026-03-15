
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

float __thiscall FUN_005eb1a0(void *param_1,undefined4 *param_2,float param_3,float param_4)

{
  float *this;
  float *this_00;
  float fVar1;
  float fVar2;
  float fVar3;
  float fVar4;
  int iVar5;
  float10 fVar6;
  float fStack_20;
  float fStack_1c;
  float fStack_18;
  float fStack_14;
  undefined4 uStack_10;
  undefined4 uStack_c;
  undefined4 uStack_8;
  undefined4 uStack_4;
  
  fVar6 = (float10)(**(code **)(*(int *)((int)param_1 + -0xc) + 0x34))();
  fVar1 = (float)(fVar6 * (float10)_DAT_0066d84c);
  FUN_005a33b0(&fStack_20,param_2);
  fStack_20 = fStack_20 - *(float *)((int)param_1 + 0x30);
  fStack_14 = -1.0;
  fStack_1c = fStack_1c - *(float *)((int)param_1 + 0x34);
  fStack_18 = fStack_18 - *(float *)((int)param_1 + 0x38);
  iVar5 = FUN_005b39e0(&fStack_20,(float *)&DAT_006bc940);
  if (iVar5 == 0) {
    if (fStack_14 == _DAT_0066d878) {
      fStack_14 = SQRT(fStack_20 * fStack_20 + fStack_18 * fStack_18 + fStack_1c * fStack_1c);
    }
    if (param_4 < fStack_14) {
      FUN_005a33b0(&uStack_10,&fStack_20);
      if ((fStack_14 != _DAT_0066d890) &&
         (fVar6 = FUN_005b50b0(&fStack_20), (float10)_DAT_0066d874 < fVar6)) {
        FUN_005ba080(&fStack_20,(float)((float10)_DAT_0066d890 / fVar6));
      }
      fVar3 = param_3 * _DAT_0066d894;
      fVar4 = fStack_1c * fVar1 * _DAT_0066d844;
      fVar2 = fStack_18 * fVar1;
      fStack_1c = fVar3 * fVar4;
      fStack_18 = fVar3 * fVar2;
      *(float *)((int)param_1 + 0x6c) = fStack_20 * fVar1;
      fStack_20 = fVar3 * fStack_20 * fVar1;
      *(float *)((int)param_1 + 0x70) = fVar4;
      *(float *)((int)param_1 + 0x74) = fVar2;
      *(undefined4 *)((int)param_1 + 0x78) = 0xbf800000;
      *(undefined4 *)((int)param_1 + 0x3c) = 0xbf800000;
      *(float *)((int)param_1 + 0x30) = *(float *)((int)param_1 + 0x30) + fStack_20;
      *(float *)((int)param_1 + 0x34) = fStack_1c + *(float *)((int)param_1 + 0x34);
      *(float *)((int)param_1 + 0x38) = fStack_18 + *(float *)((int)param_1 + 0x38);
      *(undefined4 *)((int)param_1 + 0x5c) = uStack_10;
      *(undefined4 *)((int)param_1 + 0x60) = uStack_c;
      *(undefined4 *)((int)param_1 + 100) = uStack_8;
      fStack_14 = -1.0;
      param_4 = 1.4013e-45;
      *(undefined4 *)((int)param_1 + 0x68) = uStack_4;
      goto LAB_005eb3ad;
    }
  }
  param_4 = 0.0;
  *(undefined4 *)((int)param_1 + 0x6c) = DAT_006bc940;
  *(undefined4 *)((int)param_1 + 0x70) = DAT_006bc944;
  *(undefined4 *)((int)param_1 + 0x74) = DAT_006bc948;
  *(undefined4 *)((int)param_1 + 0x78) = DAT_006bc94c;
LAB_005eb3ad:
  this = (float *)((int)param_1 + 0x20);
  if ((*(float *)((int)param_1 + 0x20) == _DAT_0066d874) &&
     (*(float *)((int)param_1 + 0x28) == _DAT_0066d874)) {
    *this = *(float *)((int)param_1 + 0x10);
    *(undefined4 *)((int)param_1 + 0x24) = *(undefined4 *)((int)param_1 + 0x14);
    *(undefined4 *)((int)param_1 + 0x28) = *(undefined4 *)((int)param_1 + 0x18);
    *(undefined4 *)((int)param_1 + 0x2c) = *(undefined4 *)((int)param_1 + 0x1c);
    return param_4;
  }
  this_00 = (float *)((int)param_1 + 0x10);
  if (*(int *)((int)param_1 + 0x1c) != 0x3f800000) {
    fVar6 = FUN_005b50b0(this_00);
    if ((float10)_DAT_0066d874 < fVar6) {
      FUN_005ba080(this_00,(float)((float10)_DAT_0066d890 / fVar6));
    }
    *(undefined4 *)((int)param_1 + 0x1c) = 0x3f800000;
  }
  if (*(int *)((int)param_1 + 0x2c) != 0x3f800000) {
    fVar6 = FUN_005b50b0(this);
    if ((float10)_DAT_0066d874 < fVar6) {
      FUN_005ba080(this,(float)((float10)_DAT_0066d890 / fVar6));
    }
    *(undefined4 *)((int)param_1 + 0x2c) = 0x3f800000;
  }
  if (*this_00 * *this +
      *(float *)((int)param_1 + 0x28) * *(float *)((int)param_1 + 0x18) +
      *(float *)((int)param_1 + 0x24) * *(float *)((int)param_1 + 0x14) < _DAT_0066d874) {
    fVar2 = *(float *)((int)param_1 + 0x14) * _DAT_0066d878;
    *this_00 = *this_00 * _DAT_0066d878;
    fVar1 = *(float *)((int)param_1 + 0x18) * _DAT_0066d878;
    *(float *)((int)param_1 + 0x14) = fVar2;
    *(float *)((int)param_1 + 0x18) = fVar1;
  }
  fVar6 = (float10)FUN_005cd2d0((int)param_1);
  fVar1 = (float)fVar6;
  if ((fVar6 != (float10)_DAT_0066d874) && (_DAT_006bd884 <= fVar1)) {
                    /* WARNING: Load size is inaccurate */
    (**(code **)(*param_1 + 4))(fVar1,param_3);
    return fVar1;
  }
  iVar5 = FUN_005b39e0((void *)((int)param_1 + 0x6c),(float *)&DAT_006bc940);
  if (iVar5 != 0) {
    *(undefined4 *)((int)param_1 + 0x70) = 0x3f800000;
    *(undefined4 *)((int)param_1 + 0x78) = 0xbf800000;
  }
  return param_4;
}

