
undefined4 * __fastcall FUN_004acd10(undefined4 *param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006386a8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004ecc00(param_1);
  local_4 = 0;
  FUN_005f52c0(param_1 + 7);
  *param_1 = &PTR_FUN_0065c038;
  ExceptionList = local_c;
  return param_1;
}

