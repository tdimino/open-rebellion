
undefined4 * __fastcall FUN_0054cd80(undefined4 *param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00648159;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_0054f2c0(param_1);
  local_4 = 0;
  param_1[0x14] = 0;
  param_1[0x15] = 0;
  param_1[0x16] = 0;
  FUN_0042dbe0(param_1 + 0x17);
  local_4._0_1_ = 1;
  FUN_004ece30(param_1 + 0x18);
  local_4._0_1_ = 2;
  FUN_004f4340(param_1 + 0x19);
  local_4 = CONCAT31(local_4._1_3_,3);
  FUN_004f4340(param_1 + 0x1b);
  *param_1 = &PTR_FUN_00661e28;
  ExceptionList = local_c;
  return param_1;
}

