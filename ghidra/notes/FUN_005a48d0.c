
undefined4 * __thiscall FUN_005a48d0(void *this,int param_1)

{
  bool bVar1;
  undefined4 *puVar2;
  int *piVar3;
  void *pvVar4;
  uint uVar5;
  int iVar6;
  undefined3 extraout_var;
  int iVar7;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00652591;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  if (DAT_006bc470 == (undefined4 *)0x0) {
    ExceptionList = &local_c;
    puVar2 = (undefined4 *)FUN_00618b70(0x48);
    local_4 = 0;
    if (puVar2 == (undefined4 *)0x0) {
      DAT_006bc470 = (undefined4 *)0x0;
    }
    else {
      DAT_006bc470 = FUN_005a6290(puVar2);
    }
  }
  local_4 = 0xffffffff;
  piVar3 = (int *)FUN_005a6470(DAT_006bc470,*(uint *)((int)this + 0x58));
  if (piVar3 == (int *)0x0) {
    puVar2 = (undefined4 *)FUN_00618b70(0x54);
    local_4 = 2;
    if (puVar2 == (undefined4 *)0x0) {
      ExceptionList = local_c;
      return (undefined4 *)0x0;
    }
    puVar2 = FUN_005a0c80(puVar2);
    ExceptionList = local_c;
    return puVar2;
  }
  uVar5 = *(uint *)(*(int *)((int)this + 0x5c) + 0x18);
  pvVar4 = (void *)FUN_0059ed70();
  uVar5 = FUN_005a02c0(pvVar4,uVar5);
  if (uVar5 != 0xffffffff) {
    if (param_1 == 6) {
      if (piVar3[0x14] != 0) {
LAB_005a4975:
        puVar2 = (undefined4 *)FUN_00618b70(0x54);
        local_4 = 1;
        if (puVar2 == (undefined4 *)0x0) {
          ExceptionList = local_c;
          return (undefined4 *)0x0;
        }
        puVar2 = FUN_005a0c80(puVar2);
        ExceptionList = local_c;
        return puVar2;
      }
      iVar6 = FUN_005aaf90();
      bVar1 = FUN_005ad930(iVar6);
      if (CONCAT31(extraout_var,bVar1) == 0) goto LAB_005a4975;
    }
    iVar6 = FUN_005aaf90();
    iVar7 = 1;
    if (*(int *)(iVar6 + 0x9ec) == 0) {
      uVar5 = uVar5 + 0x80;
    }
    else {
      uVar5 = uVar5 + 0xfe;
    }
    pvVar4 = (void *)FUN_005ba8b0();
    FUN_005ba980(pvVar4,uVar5,iVar7);
  }
  (**(code **)(*piVar3 + 100))(param_1);
  ExceptionList = local_c;
  return (undefined4 *)0x0;
}

