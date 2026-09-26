
undefined4 * __fastcall FUN_004923a0(undefined4 *param_1)

{
  undefined4 uVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00635dc7;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004c4ab0(param_1);
  local_4 = 0;
  FUN_005f2f50(param_1 + 0x1a);
  local_4._0_1_ = 1;
  FUN_004ece30(param_1 + 0x1d);
  local_4._0_1_ = 2;
  FUN_004ece30(param_1 + 0x1e);
  local_4._0_1_ = 3;
  FUN_004f5510(param_1 + 0x22);
  local_4 = CONCAT31(local_4._1_3_,4);
  *param_1 = &PTR_FUN_0065b710;
  param_1[0x19] = 0;
  param_1[0x1f] = 0x16;
  param_1[0x20] = 0;
  param_1[0x21] = 3;
  uVar1 = FUN_0048af30();
  param_1[0xd] = uVar1;
  param_1[0x24] = 0;
  param_1[0x25] = 0;
  param_1[0x10] = 0;
  ExceptionList = local_c;
  return param_1;
}

