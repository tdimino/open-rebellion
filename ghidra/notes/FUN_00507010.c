
undefined4 FUN_00507010(void)

{
  undefined4 uVar1;
  uint local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00641548;
  local_c = ExceptionList;
  local_10 = 0x32000242;
  local_4 = 0;
  ExceptionList = &local_c;
  uVar1 = FUN_0053efa0(&local_10);
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return uVar1;
}

