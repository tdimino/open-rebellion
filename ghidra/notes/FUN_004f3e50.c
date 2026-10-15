
undefined4 FUN_004f3e50(undefined4 param_1)

{
  undefined4 uVar1;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_0063f5b8;
  local_c = ExceptionList;
  local_10 = 0x32000242;
  local_4 = 0;
  ExceptionList = &local_c;
  uVar1 = FUN_0053ef00(&local_10,param_1);
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return uVar1;
}

