
undefined4 * __thiscall FUN_005a3740(void *this,int param_1)

{
  int *piVar1;
  undefined4 *puVar2;
  void *this_00;
  void *pvVar3;
  uint uVar4;
  int iVar5;
  int iVar6;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006523a6;
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
    this_00 = (void *)FUN_005a6460(DAT_006bc470,*(uint *)((int)this + 0x58));
    if (this_00 == (void *)0x0) {
      puVar2 = (undefined4 *)FUN_00618b70(0x54);
      local_4 = 1;
      if (puVar2 != (undefined4 *)0x0) {
        puVar2 = FUN_005a0c80(puVar2);
        ExceptionList = local_c;
        return puVar2;
      }
    }
    else {
      iVar5 = *(int *)((int)this_00 + 0x18);
      pvVar3 = (void *)FUN_0059ed70();
      uVar4 = FUN_005a0300(pvVar3,iVar5);
      if (uVar4 != 0xffffffff) {
        iVar5 = FUN_005aaf90();
        iVar6 = 1;
        if (*(int *)(iVar5 + 0x9ec) == 0) {
          uVar4 = uVar4 + 0x6f;
        }
        else {
          uVar4 = uVar4 + 0xed;
        }
        pvVar3 = (void *)FUN_005ba8b0();
        FUN_005ba980(pvVar3,uVar4,iVar6);
      }
      FUN_005c9be0(this_00,param_1);
    }
  }
  ExceptionList = local_c;
  return (undefined4 *)0x0;
}

