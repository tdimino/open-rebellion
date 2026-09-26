
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __fastcall FUN_005cf1a0(int param_1)

{
  float fVar1;
  float *pfVar2;
  void *pvVar3;
  float *pfVar4;
  int iVar5;
  void *this;
  float10 fVar6;
  float10 fVar7;
  undefined4 *puVar8;
  float fStack_40;
  float fStack_3c;
  float fStack_38;
  undefined4 uStack_34;
  float fStack_30;
  float fStack_2c;
  float fStack_28;
  undefined4 uStack_24;
  undefined4 uStack_20;
  float fStack_1c;
  float fStack_18;
  undefined4 uStack_14;
  undefined4 auStack_10 [4];
  
  *(undefined4 *)(param_1 + 4) = 0;
  iVar5 = *(int *)((int)*(void **)(param_1 + 8) + 0x50);
  pfVar2 = (float *)FUN_005cb020(*(void **)(param_1 + 8),&uStack_20);
  puVar8 = auStack_10;
  pvVar3 = (void *)FUN_005aaf90();
  pfVar4 = (float *)FUN_005a9650(pvVar3,puVar8,iVar5);
  fStack_2c = pfVar4[1] - pfVar2[1];
  fStack_30 = pfVar4[2] - pfVar2[2];
  fStack_28 = -(*pfVar4 - *pfVar2);
  uStack_24 = _DAT_0066d08c;
  fStack_40 = _DAT_0066d090;
  fStack_3c = _DAT_0066d090;
  fStack_38 = _DAT_0066d090;
  uStack_34 = _DAT_0066d08c;
  fStack_1c = fStack_2c;
  fStack_18 = fStack_30;
  iVar5 = FUN_005d21b0((void *)(*(int *)(param_1 + 8) + 0x8c),&fStack_30);
  if (iVar5 == 0) {
    pfVar2 = &fStack_30;
    iVar5 = *(int *)(*(int *)(param_1 + 8) + 0x50);
    pvVar3 = (void *)FUN_005aaf90();
    iVar5 = FUN_005a95d0(pvVar3,iVar5,pfVar2);
    if (iVar5 == 0) {
      (**(code **)(**(int **)(param_1 + 8) + 100))(0);
      return;
    }
    fStack_3c = *(float *)(iVar5 + 0x34);
    fStack_40 = *(float *)(iVar5 + 0x30);
    fStack_18 = *(float *)(iVar5 + 0x38);
    uStack_34 = *(undefined4 *)(iVar5 + 0x3c);
  }
  else {
    fStack_3c = *(float *)(iVar5 + 0x34);
    fStack_40 = *(float *)(iVar5 + 0x30);
    fStack_18 = *(float *)(iVar5 + 0x38);
    uStack_34 = *(undefined4 *)(iVar5 + 0x3c);
  }
  fStack_38 = fStack_18;
  uStack_14 = uStack_34;
  pfVar2 = (float *)FUN_005cb020(*(void **)(param_1 + 8),auStack_10);
  fVar6 = (float10)fcos((float10)_DAT_0066d070 * (float10)_DAT_0066d098);
  fVar7 = (float10)fsin((float10)_DAT_0066d070 * (float10)_DAT_0066d098);
  fVar1 = fStack_38 - pfVar2[2];
  fStack_3c = (fStack_3c - pfVar2[1]) * _DAT_0066d07c;
  fStack_38 = (float)(fVar7 * (float10)(fStack_40 - *pfVar2) + fVar6 * (float10)fVar1) *
              _DAT_0066d07c;
  fStack_40 = ((float)fVar6 * (fStack_40 - *pfVar2) - (float)fVar7 * fVar1) * _DAT_0066d07c;
  uStack_34 = _DAT_0066d08c;
  pfVar4 = (float *)FUN_005cb020(*(void **)(param_1 + 8),auStack_10);
  fStack_40 = *pfVar4 + fStack_40;
  pfVar2 = &fStack_40;
  fStack_3c = pfVar4[1] + fStack_3c;
  fStack_38 = pfVar4[2] + fStack_38;
  uStack_34 = _DAT_0066d08c;
  pvVar3 = (void *)FUN_005aaf90();
  iVar5 = FUN_005a9530(pvVar3,pfVar2);
  pvVar3 = *(void **)(param_1 + 8);
  this = (void *)FUN_005aaf90();
  FUN_005a8f70(this,iVar5,pvVar3);
  *(undefined4 *)(param_1 + 0x10) = 1;
  *(undefined4 *)(param_1 + 0x14) = 1;
  return;
}

