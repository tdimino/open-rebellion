
void __fastcall FUN_00527ab0(undefined4 *param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  puStack_8 = &LAB_00644434;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_1 = &PTR_FUN_0065fbf8;
  param_1[0xc] = &PTR_LAB_0065fbf0;
  local_4 = 4;
  FUN_005f2ff0(param_1 + 0x20);
  local_4._0_1_ = 3;
  FUN_00619730();
  local_4._0_1_ = 2;
  FUN_00619730();
  local_4._0_1_ = 1;
  FUN_00619730();
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_004f5f60(param_1);
  ExceptionList = local_c;
  return;
}

