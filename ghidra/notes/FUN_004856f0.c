
undefined4 * __fastcall FUN_004856f0(undefined4 *param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006347ce;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_00488460(param_1);
  local_4 = 0;
  FUN_004bee50(param_1 + 0x15);
  local_4._0_1_ = 1;
  FUN_004be7e0(param_1 + 0x19);
  local_4 = CONCAT31(local_4._1_3_,2);
  FUN_004178f0(param_1 + 0x21);
  param_1[0x14] = 0;
  param_1[0x20] = 0;
  param_1[0x106] = 0;
  param_1[0x107] = 0;
  param_1[0x108] = 0;
  param_1[0x109] = 0;
  param_1[0x10a] = 0;
  *param_1 = &PTR_FUN_0065abf0;
  ExceptionList = local_c;
  return param_1;
}

