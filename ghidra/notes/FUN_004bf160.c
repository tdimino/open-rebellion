
void __fastcall FUN_004bf160(undefined4 *param_1)

{
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  puStack_8 = &LAB_0063a0a9;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_FUN_0065c480;
  local_4 = 3;
  if ((undefined4 *)param_1[0x18] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x18])(1);
    param_1[0x18] = 0;
  }
  local_4._0_1_ = 2;
  FUN_004e5150(param_1 + 0x19);
  local_4._0_1_ = 1;
  FUN_00619730();
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00435610(param_1 + 0x10);
  local_4 = 0xffffffff;
  FUN_004e5020(param_1);
  ExceptionList = pvStack_c;
  return;
}

