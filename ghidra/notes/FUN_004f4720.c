
void __fastcall FUN_004f4720(undefined4 *param_1)

{
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  puStack_8 = &LAB_0063f77e;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_FUN_0065d0c0;
  local_4 = 2;
  FUN_004f4380(param_1 + 0x16);
  local_4._0_1_ = 1;
  FUN_004f4380(param_1 + 0x14);
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_0051faf0(param_1);
  ExceptionList = pvStack_c;
  return;
}

