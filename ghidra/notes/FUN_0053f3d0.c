
void FUN_0053f3d0(void)

{
  undefined4 *puVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064691f;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_0053f450();
  puVar1 = (undefined4 *)FUN_00618b70(0x10);
  if (puVar1 == (undefined4 *)0x0) {
    FUN_00619730();
  }
  local_4 = 0;
  if (puVar1 == (undefined4 *)0x0) {
    DAT_006b90e4 = (undefined4 *)0x0;
  }
  else {
    DAT_006b90e4 = FUN_00585b20(puVar1);
  }
  local_4 = 0xffffffff;
  if (DAT_006b90e4 == (undefined4 *)0x0) {
    FUN_0053f450();
  }
  ExceptionList = local_c;
  return;
}

