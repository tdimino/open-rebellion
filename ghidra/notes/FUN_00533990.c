
undefined4 FUN_00533990(undefined4 param_1,void *param_2)

{
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00645898;
  local_c = ExceptionList;
  local_14 = 1;
  local_10 = 0x17;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_2,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return 1;
}

