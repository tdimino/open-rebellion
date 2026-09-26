
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

float10 __thiscall FUN_005b7780(void *param_1,int *param_2)

{
  float fVar1;
  undefined4 uVar2;
  int iVar3;
  int iVar4;
  float unaff_EBX;
  uint uVar5;
  float10 fVar6;
  float10 fVar7;
  longlong lVar8;
  float fVar9;
  float local_10;
  
  fVar6 = (float10)(**(code **)(*(int *)param_1 + 0x60))();
  if (fVar6 == (float10)_DAT_0066c4f0) {
    return (float10)_DAT_0066c4f0;
  }
  iVar3 = *(int *)param_1;
  uVar2 = (**(code **)(*param_2 + 0x20))();
  iVar3 = (**(code **)(iVar3 + 0x84))(uVar2);
  if (iVar3 != 4) {
    uVar5 = *(int *)((int)param_1 + iVar3 * 0x74 + 0x17c) +
            *(int *)((int)param_1 + iVar3 * 0x74 + 0x180) +
            *(int *)((int)param_1 + iVar3 * 0x74 + 0x184);
    iVar4 = (**(code **)(*(int *)param_1 + 0x20))();
    if (iVar4 == 1) {
      FUN_005b1770(param_1,(float)uVar5);
      lVar8 = __ftol();
      uVar5 = (uint)lVar8;
    }
    fVar1 = (float)uVar5 / 0.0;
    if (fVar1 <= _DAT_0066c4f0) {
      return (float10)_DAT_0066c4f0;
    }
    fVar6 = (float10)(**(code **)(*(int *)param_1 + 0xbc))();
    fVar7 = (float10)(**(code **)(*param_2 + 0xbc))();
    local_10 = (float)((float10)(float)fVar6 / fVar7);
    if (local_10 < _DAT_0066c518) {
      local_10 = 0.1;
    }
    if (0 < *(int *)((int)param_1 + iVar3 * 0x74 + 0x180)) {
      iVar4 = (**(code **)(*param_2 + 0x20))();
      if (iVar4 == 0) {
        fVar9 = *(float *)((int)param_1 + iVar3 * 0x74 + 0x1a0);
      }
      else {
        fVar9 = *(float *)((int)param_1 + iVar3 * 0x74 + 0x19c);
      }
      fVar9 = (float)*(int *)((int)param_1 + iVar3 * 0x74 + 0x180) *
              fVar9 * *(float *)((int)param_1 + iVar3 * 0x74 + 0x198);
      iVar4 = (**(code **)(*param_2 + 0x20))();
      if (iVar4 == 1) {
        fVar9 = 0.0;
      }
      iVar4 = (**(code **)(*(int *)param_1 + 0x20))();
      if (iVar4 == 1) {
        fVar6 = FUN_005b1770(param_1,fVar9);
      }
      else {
        fVar6 = (float10)fVar9;
      }
      unaff_EBX = (float)fVar6;
    }
    if (0 < *(int *)((int)param_1 + iVar3 * 0x74 + 0x17c)) {
      iVar4 = (**(code **)(*param_2 + 0x20))();
      if (iVar4 == 0) {
        fVar9 = *(float *)((int)param_1 + iVar3 * 0x74 + 0x1b4);
      }
      else {
        fVar9 = *(float *)((int)param_1 + iVar3 * 0x74 + 0x1b0);
      }
      fVar9 = (float)*(int *)((int)param_1 + iVar3 * 0x74 + 0x17c) *
              fVar9 * *(float *)((int)param_1 + iVar3 * 0x74 + 0x1ac);
      iVar4 = (**(code **)(*param_2 + 0x20))();
      if (iVar4 == 1) {
        fVar9 = fVar9 * local_10;
      }
      iVar4 = (**(code **)(*(int *)param_1 + 0x20))();
      if (iVar4 == 1) {
        fVar6 = FUN_005b1770(param_1,fVar9);
      }
      else {
        fVar6 = (float10)fVar9;
      }
      unaff_EBX = (float)(fVar6 + (float10)unaff_EBX);
    }
    if (0 < *(int *)((int)param_1 + iVar3 * 0x74 + 0x184)) {
      iVar4 = (**(code **)(*param_2 + 0x20))();
      if (iVar4 == 0) {
        fVar9 = *(float *)((int)param_1 + iVar3 * 0x74 + 0x1c8);
      }
      else {
        fVar9 = *(float *)((int)param_1 + iVar3 * 0x74 + 0x1c4);
      }
      fVar9 = (float)*(int *)((int)param_1 + iVar3 * 0x74 + 0x184) *
              *(float *)((int)param_1 + iVar3 * 0x74 + 0x1c0) * fVar9;
      iVar3 = (**(code **)(*param_2 + 0x20))();
      if (iVar3 == 1) {
        fVar9 = fVar9 * local_10;
      }
      iVar3 = (**(code **)(*(int *)param_1 + 0x20))();
      if (iVar3 == 1) {
        fVar6 = FUN_005b1770(param_1,fVar9);
      }
      else {
        fVar6 = (float10)fVar9;
      }
      unaff_EBX = (float)(fVar6 + (float10)unaff_EBX);
    }
    fVar6 = (float10)FUN_005b23a0(param_1);
    if (fVar6 != (float10)_DAT_0066c4f0) {
      iVar3 = (**(code **)(*(int *)param_1 + 0x20))();
      if (iVar3 == 1) {
        iVar3 = (**(code **)(*param_2 + 0x20))();
        if (iVar3 == 0) {
          fVar6 = FUN_005b1650((int)param_2);
          if (fVar6 == (float10)_DAT_0066c4f0) {
            fVar6 = (float10)FUN_005b23a0(param_1);
            fVar6 = FUN_005b1770(param_1,(float)fVar6);
            return (fVar6 + (float10)unaff_EBX) / (float10)fVar1;
          }
        }
      }
    }
    return (float10)unaff_EBX / (float10)fVar1;
  }
  return (float10)unaff_EBX;
}

