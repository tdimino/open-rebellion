
void FUN_0051ce10(void)

{
  undefined4 *puVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00643102;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_0051cea0();
  puVar1 = (undefined4 *)FUN_00618b70(0xc4);
  if (puVar1 == (undefined4 *)0x0) {
    FUN_00619730();
  }
  local_4 = 0;
  if (puVar1 == (undefined4 *)0x0) {
    DAT_006b2fcc = (undefined4 *)0x0;
  }
  else {
    DAT_006b2fcc = FUN_0051cd00(puVar1);
  }
  local_4 = 0xffffffff;
  if ((DAT_006b2fcc == (undefined4 *)0x0) || (DAT_006b2fcc[1] == 0)) {
    FUN_0051cea0();
  }
  ExceptionList = local_c;
  return;
}

