
int __thiscall FUN_00528430(void *this,int param_1,undefined4 param_2,void *param_3)

{
  int iVar1;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_006445c0;
  local_c = ExceptionList;
  local_14 = 1;
  local_10 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_3,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  iVar1 = FUN_004f9860(this,param_1,param_3);
  if (iVar1 == 0) {
    local_14 = 1;
    local_10 = 1;
    local_4 = 1;
    FUN_00520580(param_3,&local_14);
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  ExceptionList = local_c;
  return iVar1;
}

