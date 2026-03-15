
undefined4 * __fastcall FUN_00554020(undefined4 *param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006491ae;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004f5ea0(param_1);
  local_4 = 0;
  FUN_005401b0(param_1 + 0x16);
  local_4._0_1_ = 1;
  FUN_005401b0(param_1 + 0x18);
  local_4 = CONCAT31(local_4._1_3_,2);
  param_1[0x1a] = 0;
  *param_1 = &PTR_FUN_00662158;
  param_1[0xc] = &PTR_FUN_00662154;
  FUN_005f5600(param_1,7);
  ExceptionList = local_c;
  return param_1;
}

