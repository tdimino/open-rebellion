
void FUN_005f7cc0(void)

{
  undefined4 *puVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0065591b;
  local_c = ExceptionList;
  if (DAT_006be3a8 == (undefined4 *)0x0) {
    ExceptionList = &local_c;
    puVar1 = (undefined4 *)FUN_00618b70(0x268c);
    local_4 = 0;
    if (puVar1 != (undefined4 *)0x0) {
      DAT_006be3a8 = FUN_005f8410(puVar1);
      ExceptionList = local_c;
      return;
    }
    DAT_006be3a8 = (undefined4 *)0x0;
  }
  ExceptionList = local_c;
  return;
}

