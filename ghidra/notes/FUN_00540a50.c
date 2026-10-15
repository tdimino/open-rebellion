
undefined4 * __fastcall FUN_00540a50(undefined4 *param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00646b68;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f5590(param_1);
  local_4 = 0;
  param_1[8] = 0;
  FUN_0042dbe0(param_1 + 9);
  *param_1 = &PTR_FUN_00661aec;
  param_1[7] = 1;
  ExceptionList = local_c;
  return param_1;
}

