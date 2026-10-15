
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

float10 FUN_005ed8e0(int param_1,int *param_2)

{
  float10 fVar1;
  int iVar2;
  int iVar3;
  float10 fVar4;
  float10 fVar5;
  float10 fVar6;
  float10 fVar7;
  float fStack_4;
  
  fStack_4 = _DAT_0066da94;
  if (param_2 != (int *)0x0) {
    iVar2 = FUN_005aaf90();
    iVar2 = (**(code **)(*(int *)(iVar2 + 0xe5c) + 0xc))();
    if (iVar2 != 0) {
      do {
        if (*(int *)(iVar2 + 0x48) == param_2[0x3b]) {
          fStack_4 = fStack_4 + _DAT_0066da70;
        }
        iVar2 = *(int *)(iVar2 + 0x10);
      } while (iVar2 != 0);
    }
    iVar2 = FUN_005aaf90();
    iVar2 = (**(code **)(*(int *)(iVar2 + 0xe50) + 0xc))();
    if (iVar2 != 0) {
      do {
        if (*(int *)(iVar2 + 0x5c) == param_2[0x3b]) {
          fStack_4 = fStack_4 + _DAT_0066da74;
        }
        iVar2 = *(int *)(iVar2 + 0x10);
      } while (iVar2 != 0);
    }
    fVar4 = FUN_005b7a80(param_2,param_1);
    fVar7 = (float10)_DAT_0066da7c;
    fVar5 = FUN_005b1710(param_2);
    fVar1 = (float10)_DAT_0066daa0;
    fVar6 = (float10)FUN_005b1640(param_2);
    fStack_4 = (float)((float10)(float)((float10)(float)(fVar4 * fVar7 + (float10)fStack_4) -
                                       fVar5 * fVar1) - fVar6 * (float10)_DAT_0066daa0);
    if ((param_2[0x5c] == 1) || (param_2[0x5c] == 3)) {
      fStack_4 = fStack_4 - _DAT_0066da80;
    }
    iVar2 = param_2[0x5a];
    iVar3 = FUN_005aaf90();
    iVar2 = FUN_005f5060(*(int *)(iVar3 + 8 + iVar2 * 4) + 0x88);
    if (iVar2 != 0) {
      fVar7 = (float10)fStack_4;
      do {
        if (*(int *)(iVar2 + 0x20) == param_2[0x3b]) {
          fVar7 = (float10)*(float *)(iVar2 + 0x28) * (float10)_DAT_0066da88 + fVar7;
        }
        iVar2 = *(int *)(iVar2 + 0x10);
      } while (iVar2 != 0);
      return fVar7;
    }
  }
  return (float10)fStack_4;
}

