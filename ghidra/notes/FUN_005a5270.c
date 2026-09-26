
undefined4 * __fastcall FUN_005a5270(int *param_1)

{
  bool bVar1;
  int iVar2;
  undefined4 *puVar3;
  void *pvVar4;
  int iVar5;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  undefined4 uVar6;
  uint uVar7;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00652681;
  local_c = ExceptionList;
  iVar2 = param_1[0x16];
  ExceptionList = &local_c;
  FUN_005aaf90();
  iVar2 = FUN_005ad8d0(iVar2);
  if (iVar2 == 0) {
    puVar3 = (undefined4 *)FUN_00618b70(0x54);
    local_4 = 0;
    if (puVar3 != (undefined4 *)0x0) {
      puVar3 = FUN_005a0c80(puVar3);
      ExceptionList = local_c;
      return puVar3;
    }
  }
  else {
    iVar2 = *(int *)(param_1[0x17] + 0x68);
    if ((iVar2 == 3) || (iVar2 == 2)) {
      puVar3 = (undefined4 *)FUN_00618b70(0x54);
      local_4 = 1;
      if (puVar3 != (undefined4 *)0x0) {
        puVar3 = FUN_005a0c80(puVar3);
        ExceptionList = local_c;
        return puVar3;
      }
    }
    else {
      iVar2 = FUN_005c6070((int *)(param_1[0x17] + 0x1c));
      if (iVar2 == 0) {
        puVar3 = (undefined4 *)FUN_00618b70(0x54);
        local_4 = 2;
        if (puVar3 != (undefined4 *)0x0) {
          puVar3 = FUN_005a0c80(puVar3);
          ExceptionList = local_c;
          return puVar3;
        }
      }
      else {
        FUN_005a0840(param_1,param_1[0x17]);
        FUN_005a08d0(param_1,param_1[0x17]);
        pvVar4 = (void *)FUN_0059ed70();
        FUN_0059fca0(pvVar4);
        FUN_005c5eb0((int *)(param_1[0x17] + 0x1c));
        iVar5 = FUN_005aaf90();
        bVar1 = FUN_005ad930(iVar5);
        FUN_005c0e20(CONCAT31(extraout_var,bVar1));
        pvVar4 = (void *)FUN_0059ed70();
        FUN_0051c9b0(pvVar4,iVar2);
        iVar2 = FUN_005c5eb0((int *)(param_1[0x17] + 0x1c));
        iVar5 = FUN_0059ed70();
        bVar1 = FUN_0059f060(iVar5);
        if ((CONCAT31(extraout_var_00,bVar1) != 0) || (param_1[0x18] != iVar2)) {
          FUN_005c0f00(param_1[0x17]);
        }
        uVar6 = 0;
        param_1[0x18] = iVar2;
        pvVar4 = (void *)FUN_0059ed70();
        FUN_0051c9b0(pvVar4,uVar6);
        uVar7 = param_1[0x14];
        pvVar4 = (void *)FUN_005ba8b0();
        iVar2 = FUN_005f5500(pvVar4,uVar7);
        if (iVar2 != 0) {
          if (*(int *)(iVar2 + 0x20) == 0) {
            uVar7 = param_1[0x14];
            pvVar4 = (void *)FUN_005ba8b0();
            FUN_005bab10(pvVar4,uVar7);
            iVar2 = 0;
            param_1[0x14] = 0;
          }
          if (iVar2 != 0) {
            iVar2 = FUN_005aaf90();
            if (*(int *)(iVar2 + 0x8c8) == 0) {
              ExceptionList = local_c;
              return (undefined4 *)0x0;
            }
            uVar7 = param_1[0x14];
            pvVar4 = (void *)FUN_005ba8b0();
            FUN_005bab10(pvVar4,uVar7);
            param_1[0x14] = 0;
            ExceptionList = local_c;
            return (undefined4 *)0x0;
          }
        }
        iVar2 = FUN_005aaf90();
        if (*(int *)(iVar2 + 0x8c8) == 0) {
          (**(code **)(*param_1 + 0x3c))(0);
        }
      }
    }
  }
  ExceptionList = local_c;
  return (undefined4 *)0x0;
}

