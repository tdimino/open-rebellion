
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

float10 __fastcall FUN_005b17f0(int param_1)

{
  int *piVar1;
  void *pvVar2;
  int iVar3;
  float10 fVar4;
  float fStack_c;
  
  fStack_c = 0.0;
  if (*(float *)(param_1 + 0x3b4) == _DAT_0066c4f0) {
    for (piVar1 = (int *)(**(code **)(*(int *)(param_1 + 0x360) + 0x10))(); piVar1 != (int *)0x0;
        piVar1 = (int *)(**(code **)(*piVar1 + 8))()) {
      pvVar2 = (void *)(**(code **)(*piVar1 + 0x10))();
      fVar4 = FUN_005b1790(pvVar2);
      fStack_c = (float)(fVar4 + (float10)fStack_c);
    }
    iVar3 = FUN_00597530();
    if (*(int *)(iVar3 + 0x668) == 0) {
      iVar3 = *(int *)(param_1 + 0x168);
      pvVar2 = (void *)FUN_005aaf90();
      fVar4 = FUN_005ad750(pvVar2,iVar3);
    }
    else {
      iVar3 = *(int *)(param_1 + 0x168);
      pvVar2 = (void *)FUN_00596f30();
      fVar4 = FUN_005afb70(pvVar2,iVar3);
    }
    fVar4 = fVar4 + (float10)*(float *)(param_1 + 0x3f8);
    fVar4 = fVar4 - (fVar4 * (float10)_DAT_0066c4f4 * (float10)*(uint *)(param_1 + 0x36c) +
                    (float10)fStack_c);
    if ((float10)_DAT_0066c4f0 < fVar4) {
      return fVar4;
    }
  }
  return (float10)_DAT_0066c4f0;
}

