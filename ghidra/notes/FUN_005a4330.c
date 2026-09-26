
undefined4 * __fastcall FUN_005a4330(int *param_1)

{
  bool bVar1;
  int iVar2;
  undefined4 *puVar3;
  undefined3 extraout_var;
  int iVar4;
  void *pvVar5;
  undefined3 extraout_var_00;
  uint uVar6;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006524dc;
  local_c = ExceptionList;
  iVar2 = param_1[0x16];
  ExceptionList = &local_c;
  FUN_005aaf90();
  iVar2 = FUN_005ad900(iVar2);
  if (iVar2 == 0) {
    puVar3 = (undefined4 *)FUN_00618b70(0x54);
    local_4 = 0;
    if (puVar3 == (undefined4 *)0x0) {
      ExceptionList = local_c;
      return (undefined4 *)0x0;
    }
    puVar3 = FUN_005a0c80(puVar3);
    ExceptionList = local_c;
    return puVar3;
  }
  if (*(int *)(param_1[0x17] + 0x68) == 3) {
    puVar3 = (undefined4 *)FUN_00618b70(0x54);
    local_4 = 1;
    if (puVar3 == (undefined4 *)0x0) {
      ExceptionList = local_c;
      return (undefined4 *)0x0;
    }
    puVar3 = FUN_005a0c80(puVar3);
    ExceptionList = local_c;
    return puVar3;
  }
  FUN_005a09c0(param_1,param_1[0x17]);
  FUN_005a08d0(param_1,param_1[0x17]);
  FUN_005a0840(param_1,param_1[0x17]);
  iVar2 = FUN_0059ed70();
  iVar2 = FUN_00583c40(iVar2);
  if (iVar2 == 0) {
    ExceptionList = local_c;
    return (undefined4 *)0x0;
  }
  bVar1 = FUN_005ca2f0((void *)param_1[0x17],*(uint *)(iVar2 + 0xec));
  if (CONCAT31(extraout_var,bVar1) != 0) {
    iVar4 = iVar2;
    FUN_005aaf90();
    iVar4 = FUN_005ad870(iVar4);
    if (iVar4 != 0) goto LAB_005a44ce;
  }
  uVar6 = *(uint *)(param_1[0x17] + 0x70);
  if (DAT_006bc470 == (undefined4 *)0x0) {
    puVar3 = (undefined4 *)FUN_00618b70(0x48);
    local_4 = 2;
    if (puVar3 == (undefined4 *)0x0) {
      DAT_006bc470 = (undefined4 *)0x0;
    }
    else {
      DAT_006bc470 = FUN_005a6290(puVar3);
    }
    local_4 = 0xffffffff;
  }
  iVar2 = FUN_005a6340(DAT_006bc470,uVar6);
  if (iVar2 == 0) {
    puVar3 = (undefined4 *)FUN_00618b70(0x54);
    local_4 = 3;
    if (puVar3 == (undefined4 *)0x0) {
      ExceptionList = local_c;
      return (undefined4 *)0x0;
    }
    puVar3 = FUN_005a0c80(puVar3);
    ExceptionList = local_c;
    return puVar3;
  }
  iVar4 = iVar2;
  pvVar5 = (void *)FUN_0059ed70();
  FUN_0051c9b0(pvVar5,iVar4);
  iVar4 = FUN_0059ed70();
  FUN_0059ef60(iVar4);
LAB_005a44ce:
  *(undefined4 *)(iVar2 + 0xf4) = 1;
  iVar4 = iVar2;
  FUN_0059ed70();
  FUN_0059f4d0(iVar4);
  iVar4 = FUN_0059ed70();
  bVar1 = FUN_0059f060(iVar4);
  if (CONCAT31(extraout_var_00,bVar1) != 0) {
    FUN_005c0ed0(iVar2);
  }
  uVar6 = param_1[0x14];
  pvVar5 = (void *)FUN_005ba8b0();
  iVar2 = FUN_005f5500(pvVar5,uVar6);
  if (iVar2 != 0) {
    if (*(int *)(iVar2 + 0x20) == 0) {
      uVar6 = param_1[0x14];
      pvVar5 = (void *)FUN_005ba8b0();
      FUN_005bab10(pvVar5,uVar6);
      iVar2 = 0;
      param_1[0x14] = 0;
    }
    if (iVar2 != 0) {
      iVar2 = FUN_005aaf90();
      if (*(int *)(iVar2 + 0x8c8) == 0) {
        ExceptionList = local_c;
        return (undefined4 *)0x0;
      }
      uVar6 = param_1[0x14];
      pvVar5 = (void *)FUN_005ba8b0();
      FUN_005bab10(pvVar5,uVar6);
      param_1[0x14] = 0;
      ExceptionList = local_c;
      return (undefined4 *)0x0;
    }
  }
  iVar2 = FUN_005aaf90();
  if (*(int *)(iVar2 + 0x8c8) == 0) {
    (**(code **)(*param_1 + 0x3c))(0);
  }
  ExceptionList = local_c;
  return (undefined4 *)0x0;
}

