
undefined4 __fastcall FUN_00580450(int param_1)

{
  void *local_3c;
  uint local_38 [11];
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  puStack_8 = &LAB_0064e8f8;
  local_c = ExceptionList;
  local_3c = (void *)0x0;
  local_38[2] = 1;
  local_38[3] = 0xffffffff;
  local_38[0] = 1;
  local_38[1] = 0x12;
  local_4._0_1_ = 1;
  local_4._1_3_ = 0;
  ExceptionList = &local_c;
  FUN_00504e60((uint *)(param_1 + 0x3c),&local_3c,local_38,local_38 + 2);
  local_4._0_1_ = 0;
  FUN_00619730();
  if (local_3c != (void *)0x0) {
    FUN_004fd450(local_38 + 4,param_1 + 0x20);
    local_4._0_1_ = 2;
    FUN_0053fd60(1,local_3c,0,0,*(undefined4 *)(param_1 + 0x44),*(undefined4 *)(param_1 + 0x48),
                 (int)(local_38 + 4));
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_004fd4d0(local_38 + 4);
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return 0;
}

