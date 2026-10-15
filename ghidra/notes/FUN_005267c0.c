
undefined4 * __fastcall FUN_005267c0(undefined4 *param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00644078;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_00539980(param_1);
  local_4 = 0;
  *param_1 = &PTR_FUN_0065f280;
  param_1[0xc] = &PTR_FUN_0065f27c;
  FUN_005f5600(param_1,5);
  ExceptionList = local_c;
  return param_1;
}

