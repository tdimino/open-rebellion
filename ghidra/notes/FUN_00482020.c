
void * __thiscall FUN_00482020(void *this,undefined4 param_1,undefined4 param_2,undefined4 param_3)

{
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00634240;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004bce70(this,param_1,param_2,param_3);
  *(undefined ***)this = &PTR_FUN_0065a958;
  param_3 = 0x6a000041;
  local_4._0_1_ = 1;
  local_4._1_3_ = 0;
  FUN_004f26d0((void *)((int)this + 0x2c),&param_3);
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  ExceptionList = local_c;
  return this;
}

