
undefined4 FUN_00419e40(void)

{
  undefined4 uVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_0062b2a8;
  local_c = ExceptionList;
  local_4 = 0;
  ExceptionList = &local_c;
  uVar1 = FUN_004f5940(&stack0x00000004);
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return uVar1;
}

