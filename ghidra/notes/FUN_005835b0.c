
undefined4 * __fastcall FUN_005835b0(undefined4 *param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064f10e;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_00553e90(param_1);
  local_4 = 0;
  param_1[6] = 1;
  param_1[7] = 0;
  FUN_005401b0(param_1 + 8);
  local_4._0_1_ = 1;
  FUN_005401b0(param_1 + 10);
  local_4 = CONCAT31(local_4._1_3_,2);
  FUN_005834d0(param_1 + 0xc);
  *param_1 = &PTR_FUN_0066a0b0;
  ExceptionList = local_c;
  return param_1;
}

