
undefined4 __fastcall FUN_00578a40(int param_1)

{
  undefined4 local_28 [7];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064d0b8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004fd450(local_28,param_1 + 0x20);
  local_4 = 0;
  FUN_00516360((uint *)(param_1 + 0x3c),*(undefined4 *)(param_1 + 0x4c),*(int *)(param_1 + 0x50),
               local_28);
  local_4 = 0xffffffff;
  FUN_004fd4d0(local_28);
  ExceptionList = local_c;
  return 0;
}

