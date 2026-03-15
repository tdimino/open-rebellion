
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined4 __thiscall FUN_005b2e60(int param_1,int *param_2)

{
  int iVar1;
  float fVar2;
  float fVar3;
  float fVar4;
  int iVar5;
  
  iVar5 = (**(code **)(*param_2 + 0xc))();
  if (iVar5 == 0) {
    return 0;
  }
  do {
    iVar1 = *(int *)(iVar5 + 0x10);
    if (ABS(*(float *)(iVar1 + 0x34) - *(float *)(param_1 + 0x34)) <
        *(float *)(iVar1 + 8) + *(float *)(param_1 + 8)) {
      fVar2 = *(float *)(param_1 + 0x30) - *(float *)(iVar1 + 0x30);
      fVar4 = *(float *)(param_1 + 0x38) - *(float *)(iVar1 + 0x38);
      fVar3 = (*(float *)(iVar1 + 0xc) + *(float *)(param_1 + 0xc)) * _DAT_0066c4f8;
      if (fVar2 * fVar2 + fVar4 * fVar4 < fVar3 * fVar3) {
        return 1;
      }
    }
    iVar5 = FUN_0041c210(iVar5);
  } while (iVar5 != 0);
  return 0;
}

