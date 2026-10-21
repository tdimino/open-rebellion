
undefined4 __thiscall FUN_00577130(void *this,void *param_1)

{
  uint *puVar1;
  void *local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064cc48;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  local_10 = this;
  puVar1 = FUN_004025b0(this,(uint *)&local_10);
  local_4 = 0;
  FUN_004f26d0(param_1,puVar1);
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return 1;
}

