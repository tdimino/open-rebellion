
void __fastcall FUN_0055a1e0(undefined4 *param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  puStack_8 = &LAB_006499ef;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_1 = &PTR_FUN_00662760;
  local_4 = 5;
  thunk_FUN_00619730();
  local_4._0_1_ = 4;
  thunk_FUN_00619730();
  local_4._0_1_ = 3;
  thunk_FUN_00619730();
  local_4._0_1_ = 2;
  thunk_FUN_00619730();
  local_4._0_1_ = 1;
  thunk_FUN_00619730();
  local_4 = (uint)local_4._1_3_ << 8;
  thunk_FUN_00619730();
  local_4 = 0xffffffff;
  FUN_00553f10(param_1);
  ExceptionList = local_c;
  return;
}

