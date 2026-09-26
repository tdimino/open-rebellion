
undefined4 __thiscall FUN_00507f10(void *this,undefined4 param_1,void *param_2)

{
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00641740;
  local_c = ExceptionList;
  local_14 = 1;
  local_10 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_2,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  if ((*(int *)((int)param_2 + 4) == -1) && ((*(byte *)((int)this + 0x50) & 8) != 0)) {
    local_14 = 1;
    local_10 = 0x22;
    local_4 = 1;
    FUN_00520580(param_2,&local_14);
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  ExceptionList = local_c;
  return 1;
}

