
undefined4 * __fastcall FUN_0040afa0(undefined4 *param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00629f27;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  param_1[1] = 0;
  param_1[2] = 0;
  param_1[3] = 0;
  FUN_0042dbe0(param_1 + 4);
  local_4 = 0;
  FUN_0042dbe0(param_1 + 5);
  local_4._0_1_ = 1;
  FUN_004ece30(param_1 + 6);
  local_4._0_1_ = 2;
  FUN_004ece30(param_1 + 7);
  local_4._0_1_ = 3;
  FUN_004ece30(param_1 + 8);
  local_4 = CONCAT31(local_4._1_3_,4);
  FUN_004ece30(param_1 + 9);
  *param_1 = &PTR_FUN_0065864c;
  ExceptionList = local_c;
  return param_1;
}

