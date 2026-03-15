
void __fastcall FUN_004c4bd0(undefined4 *param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  puStack_8 = &LAB_0063a789;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_1 = &PTR_FUN_0065c4f0;
  local_4 = 3;
  FUN_00619730();
  local_4._0_1_ = 2;
  FUN_00619730();
  local_4._0_1_ = 1;
  FUN_005f2ff0(param_1 + 0x14);
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_005f2ff0(param_1 + 0x11);
  local_4 = 0xffffffff;
  FUN_004c4e30(param_1);
  ExceptionList = local_c;
  return;
}

