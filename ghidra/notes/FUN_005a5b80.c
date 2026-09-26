
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined4 * FUN_005a5b80(void)

{
  int iVar1;
  int iVar2;
  int *piVar3;
  undefined4 *puVar4;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006527ac;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005aaf90();
  iVar1 = FUN_005a9330();
  if (iVar1 == 0) {
    puVar4 = (undefined4 *)FUN_00618b70(0x54);
    local_4 = 1;
    if (puVar4 != (undefined4 *)0x0) {
      puVar4 = FUN_005a0c80(puVar4);
      ExceptionList = local_c;
      return puVar4;
    }
  }
  else if ((*(float *)(iVar1 + 100) <= _DAT_0066bb48) || (*(float *)(iVar1 + 0x6c) < _DAT_0066bb44))
  {
    puVar4 = (undefined4 *)FUN_00618b70(0x54);
    local_4 = 0;
    if (puVar4 != (undefined4 *)0x0) {
      puVar4 = FUN_005a0c80(puVar4);
      ExceptionList = local_c;
      return puVar4;
    }
  }
  else {
    if (((*(int *)(iVar1 + 0x78) == 0) &&
        (iVar2 = FUN_005aaf90(), *(int *)(*(int *)(iVar2 + 0xc) + 0x84) == 0)) &&
       (iVar1 = FUN_005ba420(iVar1), iVar1 != 0)) {
      piVar3 = (int *)FUN_0059ed70();
      if (*piVar3 != 0) {
        ExceptionList = local_c;
        return (undefined4 *)0x0;
      }
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
    FUN_005c0fa0();
    puVar4 = (undefined4 *)FUN_00618b70(0x54);
    local_4 = 2;
    if (puVar4 != (undefined4 *)0x0) {
      puVar4 = FUN_005a0c80(puVar4);
      ExceptionList = local_c;
      return puVar4;
    }
  }
  ExceptionList = local_c;
  return (undefined4 *)0x0;
}

