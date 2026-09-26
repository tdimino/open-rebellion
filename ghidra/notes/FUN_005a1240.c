
void * FUN_005a1240(uint param_1)

{
  undefined4 *puVar1;
  int iVar2;
  int iVar3;
  void *pvVar4;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00651f11;
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
  iVar2 = FUN_005a6340(DAT_006bc470,param_1);
  iVar2 = *(int *)(iVar2 + 0x168);
  iVar3 = FUN_005aaf90();
  if (iVar2 == *(int *)(iVar3 + 0x9ec)) {
    pvVar4 = (void *)FUN_00618b70(0x54);
    local_4 = 1;
    if (pvVar4 != (void *)0x0) {
      pvVar4 = FUN_005a1550(pvVar4,param_1);
      ExceptionList = local_c;
      return pvVar4;
    }
  }
  else {
    pvVar4 = (void *)FUN_00618b70(0x54);
    local_4 = 2;
    if (pvVar4 != (void *)0x0) {
      pvVar4 = FUN_005a1110(pvVar4,param_1);
      ExceptionList = local_c;
      return pvVar4;
    }
  }
  ExceptionList = local_c;
  return (void *)0x0;
}

