
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

int __thiscall FUN_005d0ce0(int *param_1,undefined4 param_2)

{
  bool bVar1;
  int *piVar2;
  int iVar3;
  int *piVar4;
  int iVar5;
  int *piVar6;
  void *pvVar7;
  undefined3 extraout_var;
  float10 fVar8;
  float10 fVar9;
  int iStack_c;
  
  iStack_c = 0;
  param_1[0xf] = 0;
  for (piVar2 = (int *)(**(code **)(param_1[0x16] + 0x10))(); piVar2 != (int *)0x0;
      piVar2 = (int *)(**(code **)(*piVar2 + 8))()) {
    bVar1 = false;
    iVar3 = (**(code **)(*param_1 + 4))();
    if (iVar3 == 1) {
      iVar3 = FUN_00596f30();
      piVar4 = (int *)(**(code **)(*(int *)(*(int *)(iVar3 + 0x24c) + 0x70) + 0xc))();
    }
    else {
      iVar3 = FUN_00596f30();
      piVar4 = (int *)(**(code **)(*(int *)(*(int *)(iVar3 + 0x250) + 0x70) + 0xc))();
    }
    for (; piVar4 != (int *)0x0; piVar4 = (int *)(**(code **)(*piVar4 + 4))()) {
      iVar3 = (**(code **)(*piVar4 + 0xc))();
      iVar3 = FUN_005d2190((int *)(iVar3 + 0x8c));
      if ((iVar3 != 0) &&
         (iVar3 = *(int *)(iVar3 + 0xec), iVar5 = (**(code **)(*piVar2 + 4))(), iVar3 == iVar5)) {
        piVar6 = (int *)(**(code **)(*piVar2 + 0x10))();
        pvVar7 = (void *)(**(code **)(*piVar4 + 0xc))();
        fVar8 = FUN_005cb650(pvVar7,piVar6);
        piVar6 = (int *)(**(code **)(*piVar2 + 0x10))();
        fVar9 = (float10)(**(code **)(*piVar6 + 100))();
        if (fVar9 <= (float10)(float)fVar8) {
          bVar1 = true;
          break;
        }
      }
    }
    if (!bVar1) {
      iVar3 = 0;
      pvVar7 = (void *)(**(code **)(*piVar2 + 0x10))(0,param_2);
      fVar8 = FUN_005b7a80(pvVar7,iVar3);
      param_1[0xf] = (int)(float)(fVar8 + (float10)(float)param_1[0xf]);
    }
    iStack_c = iStack_c + 1;
  }
  iVar3 = (**(code **)(*param_1 + 4))();
  if (iVar3 == 1) {
    iVar3 = FUN_00596f30();
    for (piVar2 = (int *)(**(code **)(*(int *)(iVar3 + 0x58) + 0x10))(); piVar2 != (int *)0x0;
        piVar2 = (int *)(**(code **)(*piVar2 + 8))()) {
      iVar3 = 0;
      pvVar7 = (void *)(**(code **)(*piVar2 + 0x10))(0,param_2);
      fVar8 = FUN_005b7a80(pvVar7,iVar3);
      iStack_c = iStack_c + 1;
      param_1[0xf] = (int)(float)(fVar8 + (float10)(float)param_1[0xf]);
    }
  }
  for (piVar2 = (int *)(**(code **)(param_1[0x19] + 0x10))(); piVar2 != (int *)0x0;
      piVar2 = (int *)(**(code **)(*piVar2 + 8))()) {
    iVar3 = 0;
    pvVar7 = (void *)(**(code **)(*piVar2 + 0x10))(0,param_2);
    fVar8 = FUN_005b7a80(pvVar7,iVar3);
    iStack_c = iStack_c + 1;
    param_1[0xf] = (int)(float)(fVar8 + (float10)(float)param_1[0xf]);
  }
  FUN_00596f30();
  iVar3 = FUN_00596bd0();
  iVar3 = FUN_005a66a0(iVar3);
  if ((iVar3 != 0) && (iVar3 = FUN_005ba420(iVar3), iVar3 != 0)) {
    iVar3 = (**(code **)(*param_1 + 4))();
    if (iVar3 == 1) {
      param_1[0xf] = (int)(_DAT_0066d0f0 + (float)param_1[0xf]);
      return iStack_c + 1;
    }
    iVar3 = FUN_00596f30();
    bVar1 = FUN_005afcc0(iVar3);
    if (CONCAT31(extraout_var,bVar1) == 0) {
      bVar1 = false;
      for (piVar2 = (int *)(**(code **)(param_1[0x1c] + 0xc))(); piVar2 != (int *)0x0;
          piVar2 = (int *)(**(code **)(*piVar2 + 4))()) {
        iVar3 = (**(code **)(*piVar2 + 0xc))();
        if ((*(int *)(iVar3 + 0x68) == 6) ||
           (iVar3 = (**(code **)(*piVar2 + 0xc))(), *(int *)(iVar3 + 100) == 6)) {
          bVar1 = true;
          break;
        }
      }
      if (!bVar1) {
        return iStack_c;
      }
    }
    param_1[0xf] = (int)(_DAT_0066d0f0 + (float)param_1[0xf]);
  }
  return iStack_c;
}

