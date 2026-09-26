
void __fastcall FUN_005209c0(undefined4 *param_1)

{
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  puStack_8 = &LAB_006436c7;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_FUN_0065edf8;
  param_1[0xc] = &PTR_FUN_0065edf4;
  local_4 = 9;
  FUN_004f4380(param_1 + 0x27);
  local_4._0_1_ = 8;
  FUN_004f4380(param_1 + 0x25);
  local_4._0_1_ = 7;
  FUN_004f4380(param_1 + 0x23);
  local_4._0_1_ = 6;
  FUN_004f4380(param_1 + 0x21);
  local_4._0_1_ = 5;
  FUN_00619730();
  local_4._0_1_ = 4;
  FUN_00619730();
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
  ExceptionList = pvStack_c;
  return;
}

