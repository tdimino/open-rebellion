
undefined4 * __fastcall FUN_0049a120(undefined4 *param_1)

{
  undefined4 uVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00636a2e;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004c4ce0(param_1);
  local_4 = 0;
  FUN_005f2f50(param_1 + 0x11);
  local_4._0_1_ = 1;
  FUN_005f2f50(param_1 + 0x14);
  local_4 = CONCAT31(local_4._1_3_,2);
  *param_1 = &PTR_FUN_0065bbd0;
  uVar1 = FUN_006158b0();
  param_1[0xd] = uVar1;
  ExceptionList = local_c;
  return param_1;
}

