
void __fastcall FUN_00488990(int param_1)

{
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  puStack_8 = &LAB_00634c1d;
  pvStack_c = ExceptionList;
  local_4 = 6;
  ExceptionList = &pvStack_c;
  FUN_00488b30(param_1);
  local_4._0_1_ = 5;
  FUN_005f5ce0((undefined4 *)(param_1 + 0x44));
  local_4._0_1_ = 4;
  FUN_00437690((undefined4 *)(param_1 + 0x34));
  local_4._0_1_ = 3;
  FUN_00437690((undefined4 *)(param_1 + 0x28));
  local_4._0_1_ = 2;
  FUN_00437690((undefined4 *)(param_1 + 0x1c));
  local_4._0_1_ = 1;
  FUN_00437690((undefined4 *)(param_1 + 0x10));
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return;
}

