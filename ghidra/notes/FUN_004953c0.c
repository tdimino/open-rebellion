
undefined4 * __fastcall FUN_004953c0(undefined4 *param_1)

{
  undefined4 uVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063607e;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004c4ab0(param_1);
  local_4 = 0;
  FUN_005f2f50(param_1 + 0x1a);
  local_4._0_1_ = 1;
  FUN_004ece30(param_1 + 0x1d);
  local_4 = CONCAT31(local_4._1_3_,2);
  *param_1 = &PTR_FUN_0065b798;
  uVar1 = FUN_0048af30();
  param_1[0xd] = uVar1;
  param_1[0x19] = 0;
  param_1[0x10] = 0;
  ExceptionList = local_c;
  return param_1;
}

