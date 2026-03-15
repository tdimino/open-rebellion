
undefined4 * __fastcall FUN_00473fd0(undefined4 *param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006329c9;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f5590(param_1);
  local_4 = 0;
  FUN_004f5510(param_1 + 9);
  local_4._0_1_ = 1;
  FUN_004ece30(param_1 + 0xb);
  local_4._0_1_ = 2;
  FUN_004ece30(param_1 + 0xc);
  local_4 = CONCAT31(local_4._1_3_,3);
  FUN_004355d0(param_1 + 0xf);
  param_1[7] = 0;
  param_1[8] = 0;
  param_1[0x13] = 0;
  param_1[0xd] = 0;
  param_1[0xe] = 0;
  *param_1 = &PTR_FUN_0065a3a0;
  ExceptionList = local_c;
  return param_1;
}

