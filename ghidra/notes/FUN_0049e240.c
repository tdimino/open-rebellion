
void __fastcall FUN_0049e240(undefined4 *param_1)

{
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  puStack_8 = &LAB_0063724e;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_FUN_0065bc98;
  local_4 = 2;
  param_1[0x15] = 0;
  FUN_00435610(param_1 + 0x11);
  local_4._0_1_ = 1;
  FUN_00619730();
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_004c7d50(param_1);
  ExceptionList = pvStack_c;
  return;
}

