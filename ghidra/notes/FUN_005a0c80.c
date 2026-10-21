
undefined4 * __fastcall FUN_005a0c80(undefined4 *param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00651dd8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_0059e880(param_1);
  local_4 = 0;
  *param_1 = &PTR_FUN_0066be30;
  FUN_005c0e70();
  FUN_005c0e60();
  FUN_005c0e80();
  ExceptionList = local_c;
  return param_1;
}

