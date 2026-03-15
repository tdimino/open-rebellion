
undefined4 * __fastcall FUN_005897c0(undefined4 *param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064faec;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  param_1[1] = 1;
  FUN_00587550(param_1 + 2);
  local_4 = 0;
  param_1[0x11] = 1;
  param_1[0x12] = 0;
  param_1[0x13] = 0;
  FUN_004f4340(param_1 + 0x14);
  local_4._0_1_ = 1;
  FUN_004f4340(param_1 + 0x16);
  local_4._0_1_ = 2;
  FUN_004f4340(param_1 + 0x18);
  local_4 = CONCAT31(local_4._1_3_,3);
  FUN_004f4340(param_1 + 0x1a);
  *param_1 = &PTR_FUN_0066a888;
  ExceptionList = local_c;
  return param_1;
}

