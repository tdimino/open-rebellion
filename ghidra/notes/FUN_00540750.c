
undefined4 * __fastcall FUN_00540750(undefined4 *param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00646b09;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_00583df0(param_1);
  local_4 = 0;
  FUN_005401b0(param_1 + 6);
  local_4._0_1_ = 1;
  FUN_005401b0(param_1 + 8);
  local_4._0_1_ = 2;
  FUN_005401b0(param_1 + 10);
  local_4 = CONCAT31(local_4._1_3_,3);
  FUN_004ece30(param_1 + 0xc);
  param_1[0xd] = 0;
  *param_1 = &PTR_FUN_00661ac0;
  ExceptionList = local_c;
  return param_1;
}

