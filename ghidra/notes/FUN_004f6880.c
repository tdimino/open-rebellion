
undefined4 FUN_004f6880(void)

{
  void *in_stack_00000014;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_0063fc58;
  local_c = ExceptionList;
  local_14 = 1;
  local_10 = 0x25;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(in_stack_00000014,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return 1;
}

