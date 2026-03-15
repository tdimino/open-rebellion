
undefined4 * __fastcall FUN_004f4690(undefined4 *param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063f74e;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_0051fa20(param_1);
  local_4 = 0;
  param_1[0x11] = 0;
  param_1[0x12] = 0;
  FUN_0042dbe0(param_1 + 0x13);
  local_4._0_1_ = 1;
  FUN_004f4340(param_1 + 0x14);
  local_4 = CONCAT31(local_4._1_3_,2);
  FUN_004f4340(param_1 + 0x16);
  *param_1 = &PTR_FUN_0065d0c0;
  ExceptionList = local_c;
  return param_1;
}

