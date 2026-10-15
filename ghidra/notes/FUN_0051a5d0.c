
undefined4 FUN_0051a5d0(void)

{
  undefined4 uVar1;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00642dc8;
  local_c = ExceptionList;
  local_14 = 0x90;
  local_10 = 0x98;
  local_4 = 0;
  ExceptionList = &local_c;
  uVar1 = FUN_004f6010(&local_14,1);
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return uVar1;
}

