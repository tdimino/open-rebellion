
undefined4 * __fastcall FUN_00488460(undefined4 *param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00634af3;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_0048a660(param_1);
  local_4 = 0;
  FUN_0042dbe0(param_1 + 0xf);
  local_4 = CONCAT31(local_4._1_3_,1);
  FUN_004ece30(param_1 + 0x10);
  param_1[0xd] = 0;
  param_1[0xe] = 0;
  param_1[0x11] = 0;
  param_1[0x12] = 0;
  param_1[0x13] = 0;
  *param_1 = &PTR_FUN_0065aef0;
  ExceptionList = local_c;
  return param_1;
}

