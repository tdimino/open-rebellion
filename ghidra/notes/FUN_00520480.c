
void __thiscall FUN_00520480(void *this,void *param_1)

{
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00643528;
  local_c = ExceptionList;
  local_14 = 1;
  local_10 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_1,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  FUN_00520040(this,param_1);
  ExceptionList = local_c;
  return;
}

