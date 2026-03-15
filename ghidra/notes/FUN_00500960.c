
int __thiscall FUN_00500960(void *this,int param_1,void *param_2)

{
  int iVar1;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_006408a0;
  local_c = ExceptionList;
  local_14 = 1;
  local_10 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_2,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  iVar1 = FUN_004f9860(this,param_1,param_2);
  if (iVar1 == 0) {
    local_14 = 1;
    local_10 = 1;
    local_4 = 1;
    FUN_00520580(param_2,&local_14);
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  ExceptionList = local_c;
  return iVar1;
}

