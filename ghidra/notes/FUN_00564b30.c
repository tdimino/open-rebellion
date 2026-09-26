
undefined4 __fastcall FUN_00564b30(int param_1)

{
  uint local_30;
  uint local_2c;
  undefined4 local_28 [7];
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064aaf8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004fd450(local_28,param_1 + 0x20);
  local_4 = 0;
  FUN_00558820(&local_2c);
  local_4._0_1_ = 1;
  FUN_00558820(&local_30);
  local_4._0_1_ = 2;
  FUN_00558880(&local_2c,*(void **)(param_1 + 0x4c));
  FUN_00558880(&local_30,*(void **)(param_1 + 0x50));
  FUN_00515c00((uint *)(param_1 + 0x3c),&local_2c,&local_30,local_28);
  local_4._0_1_ = 1;
  thunk_FUN_00619730();
  local_4 = (uint)local_4._1_3_ << 8;
  thunk_FUN_00619730();
  local_4 = 0xffffffff;
  FUN_004fd4d0(local_28);
  ExceptionList = local_c;
  return 0;
}

