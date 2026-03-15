
undefined4 __fastcall FUN_0054dd50(int param_1)

{
  int iVar1;
  undefined4 local_28 [7];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00648448;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004fd450(local_28,param_1 + 0x20);
  local_4 = 0;
  iVar1 = thunk_FUN_00506e40();
  if (iVar1 != 0) {
    FUN_00546e70((uint *)(param_1 + 0x3c),*(int *)(param_1 + 0x50),local_28);
  }
  local_4 = 0xffffffff;
  FUN_004fd4d0(local_28);
  ExceptionList = local_c;
  return 0;
}

