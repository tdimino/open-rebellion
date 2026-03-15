
void __fastcall FUN_0049c810(int param_1)

{
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  puStack_8 = &LAB_00637051;
  pvStack_c = ExceptionList;
  local_4 = 2;
  ExceptionList = &pvStack_c;
  *(undefined4 *)(param_1 + 0x74) = 0;
  FUN_0042dc70((undefined4 *)(param_1 + 0x34));
  local_4._0_1_ = 1;
  FUN_004c5490((undefined4 *)(param_1 + 0x20));
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_004c5950((undefined4 *)(param_1 + 8));
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return;
}

