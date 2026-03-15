
void __fastcall FUN_00533770(undefined4 *param_1)

{
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  puStack_8 = &LAB_00645859;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_FUN_00660d60;
  param_1[0xc] = &PTR_FUN_00660d5c;
  local_4 = 3;
  FUN_00619730();
  local_4._0_1_ = 2;
  FUN_00619730();
  local_4._0_1_ = 1;
  FUN_00619730();
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_004f2750(param_1);
  ExceptionList = pvStack_c;
  return;
}

