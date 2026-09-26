
void * __thiscall FUN_005a3020(void *this,uint param_1)

{
  undefined4 *puVar1;
  int iVar2;
  int *piVar3;
  void *pvVar4;
  uint uVar5;
  int iVar6;
  int *piVar7;
  uint uVar8;
  int iVar9;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006522ec;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  if (DAT_006bc470 == (undefined4 *)0x0) {
    ExceptionList = &local_c;
    puVar1 = (undefined4 *)FUN_00618b70(0x48);
    local_4 = 0;
    if (puVar1 == (undefined4 *)0x0) {
      DAT_006bc470 = (undefined4 *)0x0;
    }
    else {
      DAT_006bc470 = FUN_005a6290(puVar1);
    }
  }
  local_4 = 0xffffffff;
  iVar2 = FUN_005a6450(DAT_006bc470,param_1);
  piVar3 = (int *)FUN_0059ed70();
  if (*piVar3 == 0) {
    pvVar4 = (void *)FUN_00618b70(0x54);
    local_4 = 1;
    if (pvVar4 != (void *)0x0) {
      pvVar4 = FUN_005a0e70(pvVar4,param_1);
      ExceptionList = local_c;
      return pvVar4;
    }
  }
  else {
    pvVar4 = (void *)FUN_0059ed70();
    uVar5 = FUN_0059f610(pvVar4);
    if (uVar5 == 0xffffffff) {
      iVar6 = FUN_005c6070((int *)((int)this + 4));
      if (iVar6 != 0) {
        iVar9 = 1;
        if (*(int *)(iVar6 + 0x168) == 0) {
          uVar5 = 0x84;
        }
        else {
          uVar5 = 0x102;
        }
        pvVar4 = (void *)FUN_005ba8b0();
        FUN_005ba980(pvVar4,uVar5,iVar9);
        for (piVar3 = (int *)(**(code **)(*(int *)((int)this + 4) + 0x10))(); piVar3 != (int *)0x0;
            piVar3 = (int *)(**(code **)(*piVar3 + 8))()) {
          iVar9 = (**(code **)(*piVar3 + 0x10))();
          pvVar4 = (void *)FUN_005aaf90();
          piVar7 = FUN_005aded0(pvVar4,iVar9);
          iVar9 = iVar2;
          pvVar4 = (void *)FUN_005aaf90();
          FUN_005a8f20(pvVar4,iVar9,piVar7);
        }
        pvVar4 = (void *)FUN_00618b70(0x54);
        local_4 = 3;
        if (pvVar4 != (void *)0x0) {
          pvVar4 = FUN_005a1550(pvVar4,*(uint *)(iVar6 + 0xec));
          ExceptionList = local_c;
          return pvVar4;
        }
      }
    }
    else {
      iVar6 = FUN_005aaf90();
      iVar9 = 1;
      if (*(int *)(iVar6 + 0x9ec) == 0) {
        uVar8 = uVar5 + 0x22;
      }
      else {
        uVar8 = uVar5 + 0x9c;
      }
      pvVar4 = (void *)FUN_005ba8b0();
      FUN_005ba980(pvVar4,uVar8,iVar9);
      FUN_005c5ca0((int *)((int)this + 0x34),iVar2);
      pvVar4 = (void *)FUN_00618b70(0x60);
      local_4 = 2;
      if (pvVar4 != (void *)0x0) {
        pvVar4 = FUN_005a34e0(pvVar4,(int *)((int)this + 4),(int *)((int)this + 0x34),
                              (int *)((int)this + 0x1c),uVar5);
        ExceptionList = local_c;
        return pvVar4;
      }
    }
  }
  ExceptionList = local_c;
  return (void *)0x0;
}

