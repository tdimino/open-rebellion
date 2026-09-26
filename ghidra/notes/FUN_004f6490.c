
undefined4 __thiscall FUN_004f6490(void *this,int param_1,void *param_2)

{
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_0063fb60;
  local_c = ExceptionList;
  if (((*(uint *)(param_1 + 0x24) ^ *(uint *)((int)this + 0x24)) & 0xc0) == 0) {
    local_14 = 1;
    local_10 = 0xffffffff;
    local_4 = 0;
    ExceptionList = &local_c;
    FUN_00520580(param_2,&local_14);
  }
  else {
    local_14 = 1;
    local_10 = 0x24;
    local_4 = 1;
    ExceptionList = &local_c;
    FUN_00520580(param_2,&local_14);
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return 1;
}

