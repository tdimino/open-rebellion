
void FUN_005ba8b0(void)

{
  undefined4 *puVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0065358b;
  local_c = ExceptionList;
  if (DAT_006bc4c8 == (undefined4 *)0x0) {
    ExceptionList = &local_c;
    puVar1 = (undefined4 *)FUN_00618b70(0x1420);
    local_4 = 0;
    if (puVar1 != (undefined4 *)0x0) {
      DAT_006bc4c8 = FUN_005bae60(puVar1);
      ExceptionList = local_c;
      return;
    }
    DAT_006bc4c8 = (undefined4 *)0x0;
  }
  ExceptionList = local_c;
  return;
}

