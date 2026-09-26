
undefined4 * __fastcall FUN_004888f0(undefined4 *param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00634bc2;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f4950(param_1 + 2,0);
  local_4 = 0;
  FUN_005f4950(param_1 + 3,0);
  local_4._0_1_ = 1;
  FUN_00436a40(param_1 + 4);
  local_4._0_1_ = 2;
  FUN_00436a40(param_1 + 7);
  local_4._0_1_ = 3;
  FUN_00436a40(param_1 + 10);
  local_4._0_1_ = 4;
  FUN_00436a40(param_1 + 0xd);
  local_4 = CONCAT31(local_4._1_3_,5);
  FUN_005f5ca0(param_1 + 0x11);
  *param_1 = 0;
  param_1[0x14] = 0;
  param_1[0x10] = 0;
  ExceptionList = local_c;
  return param_1;
}

