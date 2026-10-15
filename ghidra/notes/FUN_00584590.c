
undefined4 * __fastcall FUN_00584590(undefined4 *param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064f2b8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_00584100(param_1);
  local_4 = 0;
  FUN_005401b0(param_1 + 6);
  param_1[8] = 0;
  param_1[9] = 0;
  *param_1 = &PTR_FUN_0066a178;
  ExceptionList = local_c;
  return param_1;
}

