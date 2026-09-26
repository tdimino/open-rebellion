
void __fastcall FUN_00476410(undefined4 *param_1)

{
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  puStack_8 = &LAB_00632f47;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_FUN_0065a3b0;
  local_4 = 0xb;
  param_1[0x16] = 0;
  if ((undefined4 *)param_1[0x17] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x17])(1);
  }
  param_1[0x17] = 0;
  FUN_004fcdd0(param_1 + 0x18);
  FUN_004fcdd0(param_1 + 0x19);
  local_4._0_1_ = 10;
  FUN_005f5300(param_1 + 0x35);
  local_4._0_1_ = 9;
  FUN_005f5300(param_1 + 0x2c);
  local_4._0_1_ = 8;
  FUN_00619730();
  local_4._0_1_ = 7;
  FUN_00619730();
  local_4._0_1_ = 6;
  FUN_00435610(param_1 + 0x12);
  local_4._0_1_ = 5;
  FUN_00619730();
  local_4._0_1_ = 4;
  FUN_00619730();
  local_4._0_1_ = 3;
  FUN_00619730();
  local_4._0_1_ = 2;
  FUN_00619730();
  local_4._0_1_ = 1;
  FUN_004f55c0(param_1 + 0xb);
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_004f55c0(param_1 + 9);
  local_4 = 0xffffffff;
  FUN_005f55f0(param_1);
  ExceptionList = pvStack_c;
  return;
}

