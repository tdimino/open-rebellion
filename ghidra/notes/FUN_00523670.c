
undefined4 __cdecl
FUN_00523670(undefined4 param_1,undefined4 param_2,undefined4 *param_3,void *param_4)

{
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00643b28;
  local_c = ExceptionList;
  local_14 = 1;
  local_10 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_4,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  *param_3 = param_2;
  ExceptionList = local_c;
  return 1;
}

