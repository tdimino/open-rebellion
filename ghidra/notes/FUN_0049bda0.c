
undefined4 * __fastcall FUN_0049bda0(undefined4 *param_1)

{
  undefined4 uVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00636e9f;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004c4ce0(param_1);
  local_4 = 0;
  FUN_004ece30(param_1 + 0x11);
  local_4._0_1_ = 1;
  FUN_005f2f50(param_1 + 0x15);
  local_4._0_1_ = 2;
  FUN_005f2f50(param_1 + 0x18);
  local_4._0_1_ = 3;
  FUN_004f5510(param_1 + 0x1b);
  local_4._0_1_ = 4;
  FUN_004f5510(param_1 + 0x1d);
  local_4 = CONCAT31(local_4._1_3_,5);
  *param_1 = &PTR_FUN_0065bc40;
  uVar1 = FUN_0048bc60();
  param_1[0xd] = uVar1;
  param_1[0x12] = 0;
  param_1[0x14] = 0;
  ExceptionList = local_c;
  return param_1;
}

