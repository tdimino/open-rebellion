
undefined4 * __fastcall FUN_0058eb60(undefined4 *param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006501d8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f5590();
  local_4 = 0;
  FUN_0058e5a0();
  *param_1 = &PTR_FUN_0066a96c;
  param_1[7] = param_1[9];
  ExceptionList = local_c;
  return param_1;
}

