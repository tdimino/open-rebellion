
undefined4 * __fastcall FUN_004f5ea0(undefined4 *param_1)

{
  uint uVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063fa69;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_00539e80(param_1);
  local_4 = 0;
  FUN_005f6360(param_1 + 0xc);
  local_4._0_1_ = 1;
  param_1[0xd] = 0;
  FUN_004ece30(param_1 + 0xe);
  local_4._0_1_ = 2;
  FUN_004ece30(param_1 + 0xf);
  local_4 = CONCAT31(local_4._1_3_,3);
  param_1[0x10] = 0;
  param_1[0x11] = 0;
  param_1[0x12] = 0;
  param_1[0x13] = 0;
  param_1[0x14] = 0;
  param_1[0x15] = 0;
  *param_1 = &PTR_FUN_0065d260;
  param_1[0xc] = &PTR_FUN_0065d258;
  uVar1 = FUN_00540420((int)param_1);
  FUN_00540430(param_1,uVar1);
  ExceptionList = local_c;
  return param_1;
}

