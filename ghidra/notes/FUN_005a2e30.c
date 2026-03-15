
undefined4 * __fastcall FUN_005a2e30(int *param_1)

{
  bool bVar1;
  int *piVar2;
  int *piVar3;
  undefined4 uVar4;
  int iVar5;
  uint uVar6;
  int *piVar7;
  void *pvVar8;
  undefined4 *puVar9;
  int iVar10;
  int iVar11;
  undefined3 extraout_var;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_006522b6;
  pvStack_c = ExceptionList;
  piVar7 = param_1 + 1;
  ExceptionList = &pvStack_c;
  piVar2 = (int *)(**(code **)(param_1[1] + 0x10))();
  while (piVar2 != (int *)0x0) {
    piVar3 = (int *)(**(code **)(*piVar2 + 8))();
    uVar4 = (**(code **)(*piVar2 + 0x10))();
    FUN_005aaf90();
    iVar5 = FUN_005ad870(uVar4);
    if (iVar5 == 0) {
      iVar5 = (**(code **)(*piVar2 + 0x10))();
      *(undefined4 *)(iVar5 + 0xf4) = 0;
      uVar6 = (**(code **)(*piVar2 + 4))();
      FUN_005c60d0(piVar7,uVar6);
      piVar2 = piVar3;
    }
    else {
      iVar5 = (**(code **)(*piVar2 + 0x10))();
      *(undefined4 *)(iVar5 + 0xf4) = 1;
      iVar5 = (**(code **)(*piVar2 + 0x10))();
      *(undefined4 *)(iVar5 + 0x108) = 0;
      iVar5 = (**(code **)(*piVar2 + 0x10))();
      FUN_0059ed70();
      FUN_0059f4d0(iVar5);
      piVar2 = piVar3;
    }
  }
  iVar5 = FUN_005c5eb0(piVar7);
  if (iVar5 == 1) {
    piVar7 = (int *)(**(code **)(*piVar7 + 0x10))();
    uVar6 = (**(code **)(*piVar7 + 4))();
    pvVar8 = (void *)FUN_00618b70(0x54);
    uStack_4 = 0;
    if (pvVar8 == (void *)0x0) {
      ExceptionList = pvStack_c;
      return (undefined4 *)0x0;
    }
    puVar9 = (undefined4 *)FUN_005a1550(pvVar8,uVar6);
    ExceptionList = pvStack_c;
    return puVar9;
  }
  iVar5 = FUN_005c5eb0(piVar7);
  if (iVar5 == 0) {
    puVar9 = (undefined4 *)FUN_00618b70(0x54);
    uStack_4 = 1;
    if (puVar9 == (undefined4 *)0x0) {
      ExceptionList = pvStack_c;
      return (undefined4 *)0x0;
    }
    puVar9 = FUN_005a0c80(puVar9);
    ExceptionList = pvStack_c;
    return puVar9;
  }
  iVar5 = FUN_0059ed70();
  iVar5 = FUN_00583c40(iVar5);
  uVar6 = param_1[0x14];
  pvVar8 = (void *)FUN_005ba8b0();
  iVar10 = FUN_005f5500(pvVar8,uVar6);
  if (iVar10 == 0) {
LAB_005a2fb6:
    iVar11 = FUN_005aaf90();
    if (*(int *)(iVar11 + 0x8c8) == 0) {
      (**(code **)(*param_1 + 0x3c))(0);
    }
    if (iVar10 == 0) goto LAB_005a2ff0;
  }
  else {
    if (*(int *)(iVar10 + 0x20) == 0) {
      uVar6 = param_1[0x14];
      pvVar8 = (void *)FUN_005ba8b0();
      FUN_005bab10(pvVar8,uVar6);
      iVar10 = 0;
      param_1[0x14] = 0;
    }
    if (iVar10 == 0) goto LAB_005a2fb6;
  }
  iVar10 = FUN_005aaf90();
  if (*(int *)(iVar10 + 0x8c8) != 0) {
    uVar6 = param_1[0x14];
    pvVar8 = (void *)FUN_005ba8b0();
    FUN_005bab10(pvVar8,uVar6);
    param_1[0x14] = 0;
  }
LAB_005a2ff0:
  iVar10 = FUN_0059ed70();
  bVar1 = FUN_0059f060(iVar10);
  if (CONCAT31(extraout_var,bVar1) != 0) {
    FUN_005c0ed0(iVar5);
  }
  ExceptionList = pvStack_c;
  return (undefined4 *)0x0;
}

