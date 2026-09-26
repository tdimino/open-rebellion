
void __fastcall FUN_0052d880(undefined4 *param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  puStack_8 = &LAB_00645075;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_1 = &PTR_FUN_00660af8;
  param_1[0xc] = &PTR_FUN_00660af4;
  local_4 = 5;
  FUN_005332a0((int)param_1);
  local_4._0_1_ = 4;
  thunk_FUN_00619730();
  local_4._0_1_ = 3;
  thunk_FUN_00619730();
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

