
bool __thiscall FUN_00588470(void *this,undefined4 *param_1,undefined4 param_2)

{
  int iVar1;
  uint local_18 [2];
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064f928;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_00587310(local_18,this);
  local_4 = 0;
  iVar1 = FUN_005875e0(this,local_18,param_2);
  *param_1 = local_10;
  local_4 = 0xffffffff;
  FUN_00587350(local_18);
  ExceptionList = local_c;
  return iVar1 != 0;
}

