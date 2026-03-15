
void __fastcall FUN_00589870(undefined4 *param_1)

{
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  puStack_8 = &LAB_0064fb2c;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_FUN_0066a888;
  local_4 = 3;
  FUN_004f4380(param_1 + 0x1a);
  local_4._0_1_ = 2;
  FUN_004f4380(param_1 + 0x18);
  local_4._0_1_ = 1;
  FUN_004f4380(param_1 + 0x16);
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_004f4380(param_1 + 0x14);
  local_4 = 0xffffffff;
  FUN_005875b0(param_1 + 2);
  ExceptionList = pvStack_c;
  return;
}

