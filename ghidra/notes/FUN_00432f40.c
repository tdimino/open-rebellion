
void __fastcall FUN_00432f40(undefined4 *param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  puStack_8 = &LAB_0062d129;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_1 = &PTR_FUN_00658d50;
  local_4 = 3;
  param_1[0x11] = 0;
  param_1[0x10] = 0;
  param_1[0x32] = 0;
  FUN_004f55c0(param_1 + 0x12);
  local_4._0_1_ = 2;
  FUN_00619730();
  local_4._0_1_ = 1;
  FUN_00619730();
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_004ecd30(param_1);
  ExceptionList = local_c;
  return;
}

