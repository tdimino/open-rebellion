
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

int __thiscall FUN_005d21b0(void *this,float *param_1)

{
  float fVar1;
  int *piVar2;
  int iVar3;
  void *this_00;
  float *pfVar4;
  int iVar5;
  undefined4 *puVar6;
  float fStack_30;
  float fStack_2c;
  float fStack_28;
  undefined4 auStack_10 [4];
  
  piVar2 = (int *)(**(code **)(*(int *)this + 0x10))();
  iVar5 = 0;
  fStack_30 = *param_1;
  fStack_2c = param_1[1];
  fStack_28 = param_1[2];
  fVar1 = param_1[3];
  if (((fStack_30 == DAT_006bc940) && (fStack_2c == DAT_006bc944)) && (fStack_28 == DAT_006bc948)) {
    iVar3 = (**(code **)(*piVar2 + 0x10))();
    if (iVar3 == 0) {
      return 0;
    }
    iVar3 = (**(code **)(*piVar2 + 0x10))();
    iVar3 = *(int *)(iVar3 + 0x168);
    puVar6 = auStack_10;
    this_00 = (void *)FUN_005aaf90();
    pfVar4 = (float *)FUN_005a7490(this_00,puVar6,iVar3);
    fStack_30 = -*pfVar4;
    fStack_2c = -pfVar4[1];
    fStack_28 = -pfVar4[2];
    fVar1 = _DAT_0066d128;
  }
  if (fVar1 != _DAT_0066d12c) {
    if (fVar1 == _DAT_0066d128) {
      fVar1 = SQRT(fStack_30 * fStack_30 + fStack_28 * fStack_28 + fStack_2c * fStack_2c);
    }
    if (_DAT_0066d124 < fVar1) {
      fVar1 = _DAT_0066d12c / fVar1;
      fStack_30 = fVar1 * fStack_30;
      fStack_2c = fVar1 * fStack_2c;
      fStack_28 = fVar1 * fStack_28;
    }
  }
  param_1 = (float *)0x0;
  for (; piVar2 != (int *)0x0; piVar2 = (int *)(**(code **)(*piVar2 + 8))()) {
    iVar3 = (**(code **)(*piVar2 + 0x10))();
    pfVar4 = (float *)(*(float *)(iVar3 + 0x30) * fStack_30 +
                      *(float *)(iVar3 + 0x38) * fStack_28 + *(float *)(iVar3 + 0x34) * fStack_2c);
    if ((float)param_1 < (float)pfVar4) {
      iVar5 = iVar3;
      param_1 = pfVar4;
    }
  }
  return iVar5;
}

