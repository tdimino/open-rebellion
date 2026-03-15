
undefined4 __fastcall FUN_004a2430(int param_1)

{
  undefined4 uVar1;
  undefined4 local_1c [2];
  int local_14;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00637898;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_0060a790(local_1c,1);
  uVar1 = 0;
  local_4 = 0;
  FUN_00609410(*(void **)(param_1 + 0x1b8),local_1c);
  if (local_14 != 0) {
    uVar1 = FUN_0060a860(*(void **)(param_1 + 0x1b4),*(int *)(local_14 + 0xc));
  }
  local_4 = 0xffffffff;
  FUN_0060a810(local_1c);
  ExceptionList = local_c;
  return uVar1;
}

