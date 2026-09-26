
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

float10 __thiscall FUN_005b7a80(void *param_1,int param_2)

{
  int iVar1;
  int iVar2;
  float unaff_EBX;
  uint uVar3;
  float10 fVar4;
  longlong lVar5;
  float fVar6;
  float local_10;
  
  fVar4 = (float10)(**(code **)(*(int *)param_1 + 0x60))();
  if (fVar4 == (float10)_DAT_0066c4f0) {
    return (float10)_DAT_0066c4f0;
  }
  iVar1 = (**(code **)(*(int *)param_1 + 0x84))(param_2);
  if (iVar1 != 4) {
    uVar3 = *(int *)((int)param_1 + iVar1 * 0x74 + 0x17c) +
            *(int *)((int)param_1 + iVar1 * 0x74 + 0x180) +
            *(int *)((int)param_1 + iVar1 * 0x74 + 0x184);
    iVar2 = (**(code **)(*(int *)param_1 + 0x20))();
    if (iVar2 == 1) {
      FUN_005b1770(param_1,(float)uVar3);
      lVar5 = __ftol();
      uVar3 = (uint)lVar5;
    }
    if ((float)uVar3 / 0.0 <= _DAT_0066c4f0) {
      return (float10)_DAT_0066c4f0;
    }
    fVar4 = (float10)(**(code **)(*(int *)param_1 + 0xbc))();
    local_10 = (float)(fVar4 / (float10)_DAT_0066c4d4);
    if (local_10 < _DAT_0066c518) {
      local_10 = 0.1;
    }
    iVar2 = *(int *)((int)param_1 + iVar1 * 0x74 + 0x180);
    if (0 < iVar2) {
      if (param_2 == 0) {
        fVar6 = *(float *)((int)param_1 + iVar1 * 0x74 + 0x1a0);
      }
      else {
        fVar6 = *(float *)((int)param_1 + iVar1 * 0x74 + 0x19c);
      }
      fVar6 = (float)iVar2 * *(float *)((int)param_1 + iVar1 * 0x74 + 0x198) * fVar6;
      if (param_2 == 1) {
        fVar6 = 0.0;
      }
      iVar2 = (**(code **)(*(int *)param_1 + 0x20))();
      if (iVar2 == 1) {
        fVar4 = FUN_005b1770(param_1,fVar6);
      }
      else {
        fVar4 = (float10)fVar6;
      }
      unaff_EBX = (float)fVar4;
    }
    iVar2 = *(int *)((int)param_1 + iVar1 * 0x74 + 0x17c);
    if (0 < iVar2) {
      if (param_2 == 0) {
        fVar6 = *(float *)((int)param_1 + iVar1 * 0x74 + 0x1b4);
      }
      else {
        fVar6 = *(float *)((int)param_1 + iVar1 * 0x74 + 0x1b0);
      }
      fVar6 = (float)iVar2 * fVar6 * *(float *)((int)param_1 + iVar1 * 0x74 + 0x1ac);
      if (param_2 == 1) {
        fVar6 = fVar6 * local_10;
      }
      iVar2 = (**(code **)(*(int *)param_1 + 0x20))();
      if (iVar2 == 1) {
        fVar4 = FUN_005b1770(param_1,fVar6);
      }
      else {
        fVar4 = (float10)fVar6;
      }
      unaff_EBX = (float)(fVar4 + (float10)unaff_EBX);
    }
    iVar2 = *(int *)((int)param_1 + iVar1 * 0x74 + 0x184);
    if (0 < iVar2) {
      if (param_2 == 0) {
        fVar6 = *(float *)((int)param_1 + iVar1 * 0x74 + 0x1c8);
      }
      else {
        fVar6 = *(float *)((int)param_1 + iVar1 * 0x74 + 0x1c4);
      }
      fVar6 = (float)iVar2 * fVar6 * *(float *)((int)param_1 + iVar1 * 0x74 + 0x1c0);
      if (param_2 == 1) {
        fVar6 = fVar6 * local_10;
      }
      iVar1 = (**(code **)(*(int *)param_1 + 0x20))();
      if (iVar1 == 1) {
        fVar4 = FUN_005b1770(param_1,fVar6);
      }
      else {
        fVar4 = (float10)fVar6;
      }
      unaff_EBX = (float)(fVar4 + (float10)unaff_EBX);
    }
    return (float10)unaff_EBX / (float10)((float)uVar3 / 0.0);
  }
  return (float10)unaff_EBX;
}

