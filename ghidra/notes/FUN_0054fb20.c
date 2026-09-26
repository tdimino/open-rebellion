
undefined4 __fastcall FUN_0054fb20(int param_1)

{
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_006488a0;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *(undefined4 *)(param_1 + 4) = 1;
  *(undefined4 *)(param_1 + 8) = 0;
  local_14 = 1;
  local_10 = 0xffffffff;
  local_4 = 0;
  FUN_00520580((void *)(param_1 + 0xc),&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  local_14 = 1;
  local_10 = 0xffffffff;
  local_4 = 1;
  FUN_00520580((void *)(param_1 + 0x14),&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return 1;
}

