
void __fastcall FUN_00542060(undefined4 *param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  puStack_8 = &LAB_00646fde;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_1 = &PTR_FUN_00661c08;
  param_1[0xc] = &PTR_FUN_00661c04;
  local_4 = 2;
  FUN_0054cb20((int)param_1);
  local_4._0_1_ = 1;
  thunk_FUN_00619730();
  local_4 = (uint)local_4._1_3_ << 8;
  thunk_FUN_00619730();
  local_4 = 0xffffffff;
  FUN_004f5f60(param_1);
  ExceptionList = local_c;
  return;
}

