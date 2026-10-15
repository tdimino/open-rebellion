
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

float10 FUN_005ed170(int *param_1,void *param_2,float *param_3,float *param_4)

{
  int *piVar1;
  float fVar2;
  float fVar3;
  float fVar4;
  float fVar5;
  int *piVar6;
  float10 fVar7;
  
  piVar6 = param_1;
  (**(code **)(*param_1 + 0x20))();
  *param_4 = 0.0;
  if ((param_1 != (int *)0x0) && (param_2 != (void *)0x0)) {
    fVar7 = FUN_005cb650(param_2,param_1);
    fVar2 = _DAT_0066da90;
    *param_3 = (float)(fVar7 + (float10)*param_3);
    param_1 = (int *)((float)param_1[0xf3] * _DAT_0066da8c + (float)param_1[0xf0]);
    if ((float)param_1 < fVar2) {
      param_1 = (int *)0x3f800000;
    }
    fVar7 = (float10)(**(code **)(*piVar6 + 100))();
    *param_4 = (float)fVar7;
    if ((float10)_DAT_0066da94 < (float10)*param_3 - fVar7) {
      piVar1 = *(int **)((int)param_2 + 0x54);
      fVar4 = (float)piVar6[0xc] - (float)piVar1[0xf];
      fVar2 = (float)piVar6[0xe] - (float)piVar1[0x11];
      fVar3 = (float)piVar6[0xd] - (float)piVar1[0x10];
      param_2 = (void *)(float)((float10)(float)param_1 / ((float10)*param_3 - fVar7));
      if (((fVar4 != DAT_006bc940) || (fVar3 != DAT_006bc944)) || (fVar2 != DAT_006bc948)) {
        fVar7 = (float10)(**(code **)(*piVar1 + 0x34))();
        fVar5 = _DAT_0066da94;
        if (fVar7 != (float10)_DAT_0066da94) {
          fVar5 = SQRT(fVar4 * fVar4 + fVar2 * fVar2 + fVar3 * fVar3) / (float)fVar7;
        }
        if (fVar5 < _DAT_0066da90) {
          fVar5 = _DAT_0066da90;
        }
        param_2 = (void *)(fVar5 * _DAT_0066da98 + (float)param_2);
      }
      return (float10)(float)param_2;
    }
  }
  return (float10)_DAT_0066da9c;
}

