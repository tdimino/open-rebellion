
undefined4 * __fastcall FUN_0049aad0(undefined4 *param_1)

{
  undefined4 uVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00636bf9;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004c4ce0(param_1);
  local_4 = 0;
  FUN_004ece30(param_1 + 0x11);
  local_4._0_1_ = 1;
  FUN_004ece30(param_1 + 0x12);
  local_4._0_1_ = 2;
  FUN_004ece30(param_1 + 0x13);
  local_4._0_1_ = 3;
  FUN_004f5510(param_1 + 0x18);
  local_4._0_1_ = 4;
  FUN_004f5510(param_1 + 0x1a);
  local_4._0_1_ = 5;
  FUN_004f5510(param_1 + 0x1c);
  local_4._0_1_ = 6;
  FUN_004f5510(param_1 + 0x1e);
  local_4._0_1_ = 7;
  FUN_004f5510(param_1 + 0x20);
  local_4._0_1_ = 8;
  FUN_005f2f50(param_1 + 0x22);
  local_4._0_1_ = 9;
  FUN_005f2f50(param_1 + 0x25);
  local_4._0_1_ = 10;
  FUN_005f2f50(param_1 + 0x28);
  local_4._0_1_ = 0xb;
  FUN_005f2f50(param_1 + 0x2b);
  local_4._0_1_ = 0xc;
  FUN_005f2f50(param_1 + 0x2e);
  local_4 = CONCAT31(local_4._1_3_,0xd);
  *param_1 = &PTR_FUN_0065bc08;
  uVar1 = FUN_0048bc60();
  param_1[0xd] = uVar1;
  param_1[0x14] = 0;
  param_1[0x15] = 0;
  param_1[0x16] = 0;
  ExceptionList = local_c;
  return param_1;
}

