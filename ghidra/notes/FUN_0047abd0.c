
void __fastcall FUN_0047abd0(undefined4 *param_1)

{
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  puStack_8 = &LAB_0063361b;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_FUN_0065a4d0;
  local_4 = 9;
  param_1[0x19] = 0;
  param_1[0x1a] = 0;
  FUN_00435610(param_1 + 0x15);
  local_4._0_1_ = 8;
  FUN_004f55c0(param_1 + 0x13);
  local_4._0_1_ = 7;
  FUN_004f55c0(param_1 + 0x11);
  local_4._0_1_ = 6;
  FUN_004f55c0(param_1 + 0xf);
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
  FUN_005f55f0(param_1);
  ExceptionList = pvStack_c;
  return;
}

