
undefined4 * __thiscall FUN_005a3630(void *this,undefined4 param_1)

{
  int *piVar1;
  undefined4 *puVar2;
  void *pvVar3;
  uint uVar4;
  int iVar5;
  int iVar6;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00652386;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  piVar1 = (int *)FUN_0059ed70();
  if (*piVar1 != 0) {
    if (DAT_006bc470 == (undefined4 *)0x0) {
      puVar2 = (undefined4 *)FUN_00618b70(0x48);
      local_4 = 0;
      if (puVar2 == (undefined4 *)0x0) {
        DAT_006bc470 = (undefined4 *)0x0;
      }
      else {
        DAT_006bc470 = FUN_005a6290(puVar2);
      }
      local_4 = 0xffffffff;
    }
    piVar1 = (int *)FUN_005a6460(DAT_006bc470,*(uint *)((int)this + 0x58));
    if (piVar1 == (int *)0x0) {
      puVar2 = (undefined4 *)FUN_00618b70(0x54);
      local_4 = 1;
      if (puVar2 != (undefined4 *)0x0) {
        puVar2 = FUN_005a0c80(puVar2);
        ExceptionList = local_c;
        return puVar2;
      }
    }
    else {
      iVar5 = piVar1[6];
      pvVar3 = (void *)FUN_0059ed70();
      uVar4 = FUN_005a0300(pvVar3,iVar5);
      if (uVar4 != 0xffffffff) {
        iVar5 = FUN_005aaf90();
        iVar6 = 1;
        if (*(int *)(iVar5 + 0x9ec) == 0) {
          uVar4 = uVar4 + 0x78;
        }
        else {
          uVar4 = uVar4 + 0xf6;
        }
        pvVar3 = (void *)FUN_005ba8b0();
        FUN_005ba980(pvVar3,uVar4,iVar6);
      }
      (**(code **)(*piVar1 + 100))(param_1);
    }
  }
  ExceptionList = local_c;
  return (undefined4 *)0x0;
}

