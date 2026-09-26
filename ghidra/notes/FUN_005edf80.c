
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __fastcall FUN_005edf80(int param_1)

{
  float fVar1;
  float fVar2;
  int *piVar3;
  int *piVar4;
  int iVar5;
  void *this;
  int *piVar6;
  float10 fVar7;
  undefined ***pppuVar8;
  float fStack_48;
  float fStack_44;
  float fStack_3c;
  undefined **local_38 [2];
  undefined **local_30 [6];
  undefined4 local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00655428;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *(undefined4 *)(param_1 + 0x30) = 0;
  *(float *)(param_1 + 0x34) = DAT_0066daa8;
  *(undefined4 *)(param_1 + 0x38) = 0;
  *(float *)(param_1 + 0xc) = DAT_0066daa8;
  FUN_005f4ed0(local_38);
  local_38[0] = &PTR_FUN_0066dac8;
  local_4 = 0;
  FUN_005f5c10(local_30);
  local_18 = 0;
  local_10 = _DAT_006bdcd4;
  *(undefined4 *)(param_1 + 0x38) = _DAT_006bdcd4;
  local_14 = 0;
  *(undefined4 *)(param_1 + 0x30) = 0;
  *(undefined4 *)(param_1 + 0x34) = 0;
  local_30[0] = &PTR_FUN_0066c678;
  FUN_005f5c50(local_30);
  piVar3 = (int *)FUN_005d2190((int *)(*(int *)(param_1 + 8) + 0x8c));
  piVar4 = (int *)(**(code **)(*(int *)(*(int *)(param_1 + 8) + 0x1c) + 0x10))();
  fStack_44 = 0.0;
  if ((piVar3 == (int *)0x0) ||
     (iVar5 = FUN_005c6070((int *)(*(int *)(param_1 + 8) + 0x1c)), iVar5 == 0)) goto LAB_005ee1e8;
  fVar7 = FUN_005b18d0(piVar3);
  fVar1 = (float)fVar7;
  for (; piVar4 != (int *)0x0; piVar4 = (int *)(**(code **)(*piVar4 + 8))()) {
    pppuVar8 = local_38;
    piVar6 = piVar3;
    this = (void *)(**(code **)(*piVar4 + 0x10))(piVar3,pppuVar8,0);
    FUN_005b49e0(this,piVar6,(int)pppuVar8);
    piVar6 = (int *)(**(code **)(*piVar4 + 0x10))();
    fVar7 = FUN_005b18d0(piVar6);
    if ((float10)fStack_44 <= fVar7) {
      piVar6 = (int *)(**(code **)(*piVar4 + 0x10))();
      fVar7 = FUN_005b18d0(piVar6);
      fStack_44 = (float)fVar7;
    }
  }
  pppuVar8 = local_38;
  piVar4 = (int *)FUN_005c6070((int *)(*(int *)(param_1 + 8) + 0x1c));
  FUN_005b49e0(piVar3,piVar4,(int)pppuVar8);
  fStack_48 = 0.0;
  fStack_3c = -3.4028235e+38;
  piVar4 = (int *)0x0;
  for (piVar3 = (int *)(*(code *)local_38[0][3])(); piVar3 != (int *)0x0;
      piVar3 = (int *)(**(code **)(*piVar3 + 8))()) {
    if ((fStack_48 <= (float)piVar3[7]) && (fVar1 < (float)piVar3[7])) {
      fStack_48 = (float)piVar3[7] - fVar1;
    }
    if ((fStack_3c <= (float)piVar3[8]) && ((float)piVar3[7] <= fStack_44)) {
      fStack_3c = (float)piVar3[8];
      piVar4 = piVar3;
    }
  }
  if (fStack_48 <= _DAT_0066dab0) {
    if (piVar4 != (int *)0x0) {
      *(float *)(param_1 + 0xc) =
           ((float)piVar4[7] - (float)piVar4[6]) * _DAT_0066dab8 + (float)piVar4[6];
      goto LAB_005ee15e;
    }
  }
  else {
    *(float *)(param_1 + 0xc) = fStack_48;
LAB_005ee15e:
    *(int *)(param_1 + 0x30) = piVar4[6];
    *(int *)(param_1 + 0x34) = piVar4[7];
    *(int *)(param_1 + 0x38) = piVar4[8];
  }
  if (fStack_44 == _DAT_0066dab0) {
    fVar2 = fVar1 * _DAT_0066dabc;
    if (fVar1 * _DAT_0066dabc <= _DAT_0066dac0) {
      fVar2 = _DAT_0066dac0;
    }
    *(float *)(param_1 + 0xc) = fVar2;
    *(float *)(param_1 + 0x30) = fVar2;
    *(undefined4 *)(param_1 + 0x34) = 0x497423f0;
  }
  if (*(float *)(param_1 + 0x34) == _DAT_0066dab0) {
    *(float *)(param_1 + 0x34) = DAT_0066daa8 + DAT_0066daa8;
  }
  fVar1 = DAT_0066daa8;
  if (DAT_0066daa8 <= *(float *)(param_1 + 0x30)) {
    fVar1 = *(float *)(param_1 + 0x30);
  }
  *(float *)(param_1 + 0x30) = fVar1;
LAB_005ee1e8:
  local_4 = 0xffffffff;
  local_38[0] = &PTR_FUN_0066dac8;
  FUN_005f4f00(local_38);
  ExceptionList = pvStack_c;
  return;
}

