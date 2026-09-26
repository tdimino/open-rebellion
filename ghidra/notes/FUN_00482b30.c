
void * __thiscall FUN_00482b30(void *this,int param_1,undefined4 param_2,undefined4 param_3)

{
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00634368;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004bd810(this,param_1,param_2,param_3);
  *(undefined ***)this = &PTR_FUN_0065a9b0;
  local_4._1_3_ = 0;
  if (param_1 == 2) {
    param_3 = 0x63000081;
    local_4._0_1_ = 1;
    FUN_004f26d0((void *)((int)this + 0x2c),&param_3);
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_00619730();
    *(undefined4 *)((int)this + 0x7c) = 0x40;
  }
  else {
    param_3 = 0x62000017;
    local_4._0_1_ = 2;
    FUN_004f26d0((void *)((int)this + 0x2c),&param_3);
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_00619730();
    *(undefined4 *)((int)this + 0x7c) = 0x10;
  }
  ExceptionList = local_c;
  return this;
}

