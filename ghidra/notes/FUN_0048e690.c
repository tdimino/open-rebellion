
undefined4 * __fastcall FUN_0048e690(undefined4 *param_1)

{
  undefined4 uVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006357be;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004c4ab0(param_1);
  local_4 = 0;
  FUN_005f4950(param_1 + 0x19,0);
  local_4._0_1_ = 1;
  FUN_005f2f50(param_1 + 0x1a);
  local_4 = CONCAT31(local_4._1_3_,2);
  *param_1 = &PTR_FUN_0065b490;
  uVar1 = FUN_004047d0();
  param_1[0xd] = uVar1;
  ExceptionList = local_c;
  return param_1;
}

