
undefined4 * FUN_00411d40(void)

{
  undefined4 *puVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062a74b;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  puVar1 = (undefined4 *)FUN_00618b70(0x58);
  local_4 = 0;
  if (puVar1 != (undefined4 *)0x0) {
    FUN_00403dd0(puVar1);
    *puVar1 = &PTR_FUN_006587e8;
    puVar1[0x12] = 0;
    puVar1[0x14] = 0;
    puVar1[0x13] = 0;
    puVar1[0x15] = 0;
    ExceptionList = local_c;
    return puVar1;
  }
  ExceptionList = local_c;
  return (undefined4 *)0x0;
}

