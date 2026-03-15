
void FUN_0059ed70(void)

{
  undefined4 *puVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00651bab;
  local_c = ExceptionList;
  if (DAT_006bc468 == (undefined4 *)0x0) {
    ExceptionList = &local_c;
    puVar1 = (undefined4 *)FUN_00618b70(0x188);
    local_4 = 0;
    if (puVar1 != (undefined4 *)0x0) {
      DAT_006bc468 = FUN_0059eb70(puVar1);
      ExceptionList = local_c;
      return;
    }
    DAT_006bc468 = (undefined4 *)0x0;
  }
  ExceptionList = local_c;
  return;
}

