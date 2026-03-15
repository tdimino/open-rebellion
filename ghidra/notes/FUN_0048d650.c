
undefined4 * __fastcall FUN_0048d650(undefined4 *param_1)

{
  undefined4 uVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006355f9;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004c4ab0(param_1);
  local_4 = 0;
  FUN_005f4950(param_1 + 0x19,0);
  local_4._0_1_ = 1;
  FUN_004ece30(param_1 + 0x1a);
  local_4._0_1_ = 2;
  FUN_004ece30(param_1 + 0x1b);
  local_4 = CONCAT31(local_4._1_3_,3);
  *param_1 = &PTR_FUN_0065b408;
  uVar1 = FUN_0048af30();
  param_1[0xd] = uVar1;
  param_1[0x10] = 0;
  ExceptionList = local_c;
  return param_1;
}

