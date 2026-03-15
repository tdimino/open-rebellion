
undefined4 FUN_00435ad0(void)

{
  undefined4 *puVar1;
  undefined4 uVar2;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062d46b;
  local_c = ExceptionList;
  uVar2 = 0;
  ExceptionList = &local_c;
  FUN_00435b60();
  puVar1 = (undefined4 *)FUN_00618b70(0x34);
  local_4 = 0;
  if (puVar1 == (undefined4 *)0x0) {
    DAT_006b14e4 = (undefined4 *)0x0;
  }
  else {
    DAT_006b14e4 = FUN_00435b90(puVar1);
  }
  local_4 = 0xffffffff;
  if (DAT_006b14e4 != (undefined4 *)0x0) {
    if (DAT_006b14e4[8] == 0) {
      FUN_00435b60();
      ExceptionList = local_c;
      return 0;
    }
    uVar2 = 1;
  }
  ExceptionList = local_c;
  return uVar2;
}

