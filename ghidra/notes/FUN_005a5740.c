
undefined4 * FUN_005a5740(uint param_1)

{
  int iVar1;
  int *piVar2;
  void *pvVar3;
  undefined4 *puVar4;
  int iVar5;
  int iVar6;
  longlong lVar7;
  uint uVar8;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00652707;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  piVar2 = (int *)FUN_0059ed70();
  if (*piVar2 == 0) {
    pvVar3 = (void *)FUN_00618b70(0x54);
    local_4 = 0;
    if (pvVar3 == (void *)0x0) {
      ExceptionList = local_c;
      return (undefined4 *)0x0;
    }
    puVar4 = FUN_005a1550(pvVar3,param_1);
    ExceptionList = local_c;
    return puVar4;
  }
  if (DAT_006bc470 == (undefined4 *)0x0) {
    puVar4 = (undefined4 *)FUN_00618b70(0x48);
    local_4 = 1;
    if (puVar4 == (undefined4 *)0x0) {
      DAT_006bc470 = (undefined4 *)0x0;
    }
    else {
      DAT_006bc470 = FUN_005a6290(puVar4);
    }
    local_4 = 0xffffffff;
  }
  iVar5 = FUN_005a6340(DAT_006bc470,param_1);
  if (iVar5 != 0) {
    iVar1 = *(int *)(iVar5 + 0x168);
    iVar6 = FUN_005aaf90();
    if (iVar1 == *(int *)(iVar6 + 0x9ec)) {
      puVar4 = (undefined4 *)FUN_00618b70(0x54);
      local_4 = 2;
      if (puVar4 == (undefined4 *)0x0) {
        ExceptionList = local_c;
        return (undefined4 *)0x0;
      }
      puVar4 = FUN_005a0c80(puVar4);
      ExceptionList = local_c;
      return puVar4;
    }
    FUN_005aaf90();
    pvVar3 = (void *)FUN_005a9330();
    if (pvVar3 == (void *)0x0) {
      puVar4 = (undefined4 *)FUN_00618b70(0x54);
      local_4 = 3;
      if (puVar4 == (undefined4 *)0x0) {
        ExceptionList = local_c;
        return (undefined4 *)0x0;
      }
      puVar4 = FUN_005a0c80(puVar4);
      ExceptionList = local_c;
      return puVar4;
    }
    FUN_005ba720(pvVar3,*(undefined4 *)(iVar5 + 0xec));
    iVar5 = FUN_005aaf90();
    if (*(int *)(iVar5 + 0x9ec) == 1) {
      iVar5 = 1;
      uVar8 = 0x111;
      pvVar3 = (void *)FUN_005ba8b0();
      FUN_005ba980(pvVar3,uVar8,iVar5);
    }
    lVar7 = __ftol();
    FUN_005c0de0((int)lVar7);
  }
  puVar4 = (undefined4 *)FUN_00618b70(0x54);
  local_4 = 4;
  if (puVar4 == (undefined4 *)0x0) {
    ExceptionList = local_c;
    return (undefined4 *)0x0;
  }
  puVar4 = FUN_005a0c80(puVar4);
  ExceptionList = local_c;
  return puVar4;
}

