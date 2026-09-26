
void __fastcall FUN_004cfc00(undefined4 *param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  puStack_8 = &LAB_0063bc8a;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_1 = &PTR_FUN_0065c758;
  local_4 = 6;
  FUN_004f55c0(param_1 + 0x20);
  local_4._0_1_ = 5;
  FUN_004f55c0(param_1 + 0x1e);
  local_4._0_1_ = 4;
  FUN_00435610(param_1 + 0x1a);
  local_4._0_1_ = 3;
  FUN_00619730();
  local_4._0_1_ = 2;
  FUN_00619730();
  local_4._0_1_ = 1;
  FUN_00619730();
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_004e5020(param_1);
  ExceptionList = local_c;
  return;
}

