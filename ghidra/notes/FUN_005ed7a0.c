
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

float10 FUN_005ed7a0(int param_1,int *param_2)

{
  int iVar1;
  int iVar2;
  float10 fVar3;
  float10 fVar4;
  float10 fVar5;
  float10 fVar6;
  float fStack_4;
  
  fStack_4 = _DAT_0066da94;
  if (param_2 != (int *)0x0) {
    iVar1 = FUN_005aaf90();
    iVar1 = (**(code **)(*(int *)(iVar1 + 0xe5c) + 0xc))();
    if (iVar1 != 0) {
      do {
        if (*(int *)(iVar1 + 0x48) == param_2[0x3b]) {
          fStack_4 = _DAT_0066da78 * _DAT_0066da70 + fStack_4;
        }
        iVar1 = *(int *)(iVar1 + 0x10);
      } while (iVar1 != 0);
    }
    iVar1 = FUN_005aaf90();
    iVar1 = (**(code **)(*(int *)(iVar1 + 0xe50) + 0xc))();
    if (iVar1 != 0) {
      do {
        if (*(int *)(iVar1 + 0x5c) == param_2[0x3b]) {
          fStack_4 = fStack_4 + _DAT_0066da74;
        }
        iVar1 = *(int *)(iVar1 + 0x10);
      } while (iVar1 != 0);
    }
    fVar3 = FUN_005b7a80(param_2,param_1);
    fVar4 = FUN_005b1710(param_2);
    fVar6 = (float10)_DAT_0066daa0;
    fVar5 = (float10)FUN_005b1640(param_2);
    fStack_4 = (float)((float10)(float)((float10)(float)(fVar3 + (float10)fStack_4) - fVar4 * fVar6)
                      - fVar5 * (float10)_DAT_0066daa0);
    if ((param_2[0x5c] == 1) || (param_2[0x5c] == 3)) {
      fStack_4 = fStack_4 - _DAT_0066da80;
    }
    iVar1 = param_2[0x5a];
    iVar2 = FUN_005aaf90();
    iVar1 = FUN_005f5060(*(int *)(iVar2 + 8 + iVar1 * 4) + 0x88);
    if (iVar1 != 0) {
      fVar6 = (float10)fStack_4;
      do {
        if (*(int *)(iVar1 + 0x20) == param_2[0x3b]) {
          fVar6 = (float10)*(float *)(iVar1 + 0x28) * (float10)_DAT_0066da88 + fVar6;
        }
        iVar1 = *(int *)(iVar1 + 0x10);
      } while (iVar1 != 0);
      return fVar6;
    }
  }
  return (float10)fStack_4;
}

