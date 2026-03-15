
undefined4 * __fastcall FUN_0051cd00(undefined4 *param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006430b4;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005397d0(param_1 + 0x14);
  local_4 = 0;
  FUN_005397d0(param_1 + 0x1b);
  local_4._0_1_ = 1;
  FUN_005397d0(param_1 + 0x22);
  local_4 = CONCAT31(local_4._1_3_,2);
  *param_1 = &PTR_FUN_0065ecf4;
  FUN_0051e320((int)param_1);
  ExceptionList = local_c;
  return param_1;
}

