
void __fastcall FUN_0054ce20(undefined4 *param_1)

{
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  puStack_8 = &LAB_00648199;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_FUN_00661e28;
  local_4 = 3;
  FUN_004f4380(param_1 + 0x1b);
  local_4._0_1_ = 2;
  FUN_004f4380(param_1 + 0x19);
  local_4._0_1_ = 1;
  FUN_00619730();
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_0054f330(param_1);
  ExceptionList = pvStack_c;
  return;
}

