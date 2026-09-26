
void __fastcall FUN_00417b40(int param_1)

{
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  puStack_8 = &LAB_0062b087;
  pvStack_c = ExceptionList;
  local_4 = 0x13;
  ExceptionList = &pvStack_c;
  *(undefined4 *)(param_1 + 0x178) = 0;
  FUN_0042dc70((undefined4 *)(param_1 + 0x138));
  local_4._0_1_ = 0x12;
  FUN_00619730();
  local_4._0_1_ = 0x11;
  FUN_00619730();
  local_4._0_1_ = 0x10;
  FUN_0042ec70((undefined4 *)(param_1 + 0x11c));
  local_4._0_1_ = 0xf;
  FUN_0042ee40((undefined4 *)(param_1 + 0x104));
  local_4._0_1_ = 0xe;
  FUN_0042ee40((undefined4 *)(param_1 + 0xec));
  local_4._0_1_ = 0xd;
  FUN_0042f1d0((undefined4 *)(param_1 + 0xd8));
  local_4._0_1_ = 0xc;
  FUN_0042f500((undefined4 *)(param_1 + 0xc0));
  local_4._0_1_ = 0xb;
  FUN_0042f500((undefined4 *)(param_1 + 0xa8));
  local_4._0_1_ = 10;
  FUN_004030b0((undefined4 *)(param_1 + 0x8c));
  local_4._0_1_ = 9;
  FUN_0042fd90((undefined4 *)(param_1 + 0x78));
  local_4._0_1_ = 8;
  FUN_00430290((undefined4 *)(param_1 + 0x58));
  local_4._0_1_ = 7;
  FUN_004308d0((undefined4 *)(param_1 + 0x44));
  local_4._0_1_ = 6;
  FUN_00430d90((undefined4 *)(param_1 + 0x2c));
  local_4._0_1_ = 5;
  FUN_00619730();
  local_4._0_1_ = 4;
  FUN_00619730();
  local_4._0_1_ = 3;
  FUN_00619730();
  local_4._0_1_ = 2;
  FUN_00619730();
  local_4._0_1_ = 1;
  FUN_00619730();
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return;
}

