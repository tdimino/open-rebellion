
void __fastcall FUN_0043dbb0(undefined4 *param_1)

{
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  puStack_8 = &LAB_0062e129;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_FUN_00659598;
  local_4 = 3;
  FUN_00619730();
  local_4._0_1_ = 2;
  FUN_00619730();
  local_4._0_1_ = 1;
  FUN_00619730();
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_005393b0(param_1);
  ExceptionList = pvStack_c;
  return;
}

