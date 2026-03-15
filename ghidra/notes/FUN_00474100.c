
void __fastcall FUN_00474100(undefined4 *param_1)

{
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  puStack_8 = &LAB_00632a49;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_FUN_0065a3a0;
  local_4 = 3;
  param_1[0x13] = 0;
  FUN_00435610(param_1 + 0xf);
  local_4._0_1_ = 2;
  FUN_00619730();
  local_4._0_1_ = 1;
  FUN_00619730();
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_004f55c0(param_1 + 9);
  local_4 = 0xffffffff;
  FUN_005f55f0(param_1);
  ExceptionList = pvStack_c;
  return;
}

