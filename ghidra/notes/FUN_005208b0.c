
undefined4 * __fastcall FUN_005208b0(undefined4 *param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00643637;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004f5ea0(param_1);
  local_4 = 0;
  param_1[0x16] = 0;
  param_1[0x17] = 0;
  param_1[0x18] = 0;
  param_1[0x19] = 0;
  param_1[0x1a] = 0;
  FUN_004ece30(param_1 + 0x1b);
  local_4._0_1_ = 1;
  FUN_004ece30(param_1 + 0x1c);
  local_4._0_1_ = 2;
  FUN_004ece30(param_1 + 0x1d);
  local_4._0_1_ = 3;
  FUN_004ece30(param_1 + 0x1e);
  local_4._0_1_ = 4;
  FUN_004ece30(param_1 + 0x1f);
  local_4._0_1_ = 5;
  FUN_0042dbe0(param_1 + 0x20);
  local_4._0_1_ = 6;
  FUN_004f4340(param_1 + 0x21);
  local_4._0_1_ = 7;
  FUN_004f4340(param_1 + 0x23);
  local_4._0_1_ = 8;
  FUN_004f4340(param_1 + 0x25);
  local_4 = CONCAT31(local_4._1_3_,9);
  FUN_004f4340(param_1 + 0x27);
  param_1[0x29] = 0;
  *param_1 = &PTR_FUN_0065edf8;
  param_1[0xc] = &PTR_FUN_0065edf4;
  ExceptionList = local_c;
  return param_1;
}

