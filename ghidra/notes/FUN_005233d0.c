
bool __thiscall FUN_005233d0(void *this,int param_1,int param_2)

{
  int iVar1;
  undefined4 local_28 [5];
  undefined4 local_14;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00643ad8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004fd450(local_28,param_2);
  local_4 = 0;
  local_14 = FUN_004fd2f0();
  iVar1 = FUN_00521900(this,param_1,local_28);
  local_4 = 0xffffffff;
  FUN_004fd4d0(local_28);
  ExceptionList = local_c;
  return iVar1 != 0;
}

