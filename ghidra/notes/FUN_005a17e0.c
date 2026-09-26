
undefined4 * __fastcall FUN_005a17e0(int *param_1)

{
  bool bVar1;
  int *piVar2;
  int iVar3;
  int iVar4;
  undefined4 *puVar5;
  void *pvVar6;
  uint uVar7;
  undefined3 extraout_var;
  uint uVar8;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00652001;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  piVar2 = (int *)(**(code **)(param_1[1] + 0x10))();
  iVar3 = (**(code **)(*piVar2 + 0x10))();
  iVar4 = iVar3;
  FUN_005aaf90();
  iVar4 = FUN_005ad870(iVar4);
  if (iVar4 == 0) {
    *(undefined4 *)(iVar3 + 0xf4) = 0;
    *(undefined4 *)(iVar3 + 0x100) = 0;
    FUN_0059ed70();
    FUN_0059f4d0(iVar3);
    puVar5 = (undefined4 *)FUN_00618b70(0x54);
    uStack_4 = 0;
    if (puVar5 == (undefined4 *)0x0) {
      ExceptionList = pvStack_c;
      return (undefined4 *)0x0;
    }
    puVar5 = FUN_005a0c80(puVar5);
    ExceptionList = pvStack_c;
    return puVar5;
  }
  uVar8 = *(uint *)(iVar3 + 0x178);
  if (uVar8 != 0) {
    iVar4 = iVar3;
    pvVar6 = (void *)FUN_0059ed70();
    uVar7 = FUN_005a0c50(pvVar6,iVar4);
    if (uVar7 == uVar8) {
      if (DAT_006bc470 == (undefined4 *)0x0) {
        puVar5 = (undefined4 *)FUN_00618b70(0x48);
        uStack_4 = 2;
        if (puVar5 == (undefined4 *)0x0) {
          DAT_006bc470 = (undefined4 *)0x0;
        }
        else {
          DAT_006bc470 = FUN_005a6290(puVar5);
        }
        uStack_4 = 0xffffffff;
      }
      iVar4 = FUN_005a6460(DAT_006bc470,uVar8);
    }
    else {
      if (DAT_006bc470 == (undefined4 *)0x0) {
        puVar5 = (undefined4 *)FUN_00618b70(0x48);
        uStack_4 = 1;
        if (puVar5 == (undefined4 *)0x0) {
          DAT_006bc470 = (undefined4 *)0x0;
        }
        else {
          DAT_006bc470 = FUN_005a6290(puVar5);
        }
        uStack_4 = 0xffffffff;
      }
      iVar4 = FUN_005a6460(DAT_006bc470,uVar8);
      FUN_005a09c0(param_1,iVar4);
    }
    FUN_005a08d0(param_1,iVar4);
    FUN_005a0840(param_1,iVar4);
  }
  *(undefined4 *)(iVar3 + 0xf4) = 1;
  *(undefined4 *)(iVar3 + 0x100) = 0;
  *(undefined4 *)(iVar3 + 0x108) = 0;
  iVar4 = iVar3;
  FUN_0059ed70();
  FUN_0059f4d0(iVar4);
  iVar4 = FUN_0059ed70();
  bVar1 = FUN_0059f060(iVar4);
  if (CONCAT31(extraout_var,bVar1) != 0) {
    FUN_005c0ea0(iVar3);
  }
  uVar8 = param_1[0x14];
  pvVar6 = (void *)FUN_005ba8b0();
  iVar4 = FUN_005f5500(pvVar6,uVar8);
  if (iVar4 != 0) {
    if (*(int *)(iVar4 + 0x20) == 0) {
      uVar8 = param_1[0x14];
      pvVar6 = (void *)FUN_005ba8b0();
      FUN_005bab10(pvVar6,uVar8);
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
      pvVar6 = (void *)FUN_005ba8b0();
      FUN_005bab10(pvVar6,uVar8);
      param_1[0x14] = 0;
      ExceptionList = pvStack_c;
      return (undefined4 *)0x0;
    }
  }
  iVar4 = FUN_005aaf90();
  if (*(int *)(iVar4 + 0x8c8) == 0) {
    (**(code **)(*param_1 + 0x3c))(0);
  }
  ExceptionList = pvStack_c;
  return (undefined4 *)0x0;
}

