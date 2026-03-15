
void __fastcall FUN_005407f0(undefined4 *param_1)

{
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  puStack_8 = &LAB_00646b49;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_FUN_00661ac0;
  local_4 = 3;
  FUN_00619730();
  local_4._0_1_ = 2;
  thunk_FUN_00619730();
  local_4._0_1_ = 1;
  thunk_FUN_00619730();
  local_4 = (uint)local_4._1_3_ << 8;
  thunk_FUN_00619730();
  local_4 = 0xffffffff;
  FUN_00583e30(param_1);
  ExceptionList = pvStack_c;
  return;
}

