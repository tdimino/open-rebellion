
undefined4 FUN_005061f0(void)

{
  undefined4 uVar1;
  uint local_14 [2];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00641228;
  local_c = ExceptionList;
  local_14[0] = 0x18;
  local_14[1] = 0x1c;
  local_4 = 0;
  ExceptionList = &local_c;
  uVar1 = FUN_0053efb0(local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return uVar1;
}

