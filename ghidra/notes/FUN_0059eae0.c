
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined4 * FUN_0059eae0(void)

{
  int iVar1;
  undefined4 *puVar2;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00651b2b;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005aaf90();
  iVar1 = FUN_005a9330();
  if (((iVar1 != 0) && (*(float *)(iVar1 + 0x6c) == _DAT_0066bb44)) &&
     (_DAT_0066bb48 < *(float *)(iVar1 + 100))) {
    puVar2 = (undefined4 *)FUN_00618b70(0x54);
    local_4 = 0;
    if (puVar2 != (undefined4 *)0x0) {
      puVar2 = FUN_005a5630(puVar2);
      ExceptionList = local_c;
      return puVar2;
    }
  }
  ExceptionList = local_c;
  return (undefined4 *)0x0;
}

