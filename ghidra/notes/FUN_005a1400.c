
undefined4 * __fastcall FUN_005a1400(int *param_1)

{
  bool bVar1;
  int *piVar2;
  undefined4 uVar3;
  int iVar4;
  undefined4 *puVar5;
  int iVar6;
  undefined3 extraout_var;
  void *pvVar7;
  uint uVar8;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00651f6b;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  piVar2 = (int *)(**(code **)(param_1[10] + 0x10))();
  uVar3 = (**(code **)(*piVar2 + 0x10))();
  FUN_005aaf90();
  iVar4 = FUN_005ad870(uVar3);
  if (iVar4 == 0) {
    puVar5 = (undefined4 *)FUN_00618b70(0x54);
    uStack_4 = 0;
    if (puVar5 != (undefined4 *)0x0) {
      puVar5 = FUN_005a0c80(puVar5);
      ExceptionList = pvStack_c;
      return puVar5;
    }
  }
  else {
    piVar2 = (int *)(**(code **)(param_1[10] + 0x10))();
    iVar6 = (**(code **)(*piVar2 + 0x10))();
    *(undefined4 *)(iVar6 + 0xf4) = 0;
    *(undefined4 *)(iVar6 + 0x100) = 1;
    iVar4 = iVar6;
    FUN_0059ed70();
    FUN_0059f4d0(iVar4);
    iVar4 = FUN_0059ed70();
    bVar1 = FUN_0059f060(iVar4);
    if (CONCAT31(extraout_var,bVar1) != 0) {
      FUN_005c0ea0(iVar6);
    }
    uVar8 = param_1[0x14];
    pvVar7 = (void *)FUN_005ba8b0();
    iVar4 = FUN_005f5500(pvVar7,uVar8);
    if (iVar4 != 0) {
      if (*(int *)(iVar4 + 0x20) == 0) {
        uVar8 = param_1[0x14];
        pvVar7 = (void *)FUN_005ba8b0();
        FUN_005bab10(pvVar7,uVar8);
        iVar4 = 0;
        param_1[0x14] = 0;
      }
      if (iVar4 != 0) {
        iVar4 = FUN_005aaf90();
        if (*(int *)(iVar4 + 0x8c8) == 0) {
          ExceptionList = pvStack_c;
          return (undefined4 *)0x0;
        }
        uVar8 = param_1[0x14];
        pvVar7 = (void *)FUN_005ba8b0();
        FUN_005bab10(pvVar7,uVar8);
        param_1[0x14] = 0;
        ExceptionList = pvStack_c;
        return (undefined4 *)0x0;
      }
    }
    iVar4 = FUN_005aaf90();
    if (*(int *)(iVar4 + 0x8c8) == 0) {
      (**(code **)(*param_1 + 0x3c))(0);
    }
  }
  ExceptionList = pvStack_c;
  return (undefined4 *)0x0;
}

