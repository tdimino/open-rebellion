
void * __thiscall FUN_0047aae0(void *this,undefined4 param_1,undefined4 param_2,undefined4 param_3)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063359b;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f5590(this);
  local_4 = 0;
  FUN_005f4950((void *)((int)this + 0x20),0);
  local_4._0_1_ = 1;
  FUN_004ece30((undefined4 *)((int)this + 0x28));
  local_4._0_1_ = 2;
  FUN_0042dbe0((undefined4 *)((int)this + 0x2c));
  local_4._0_1_ = 3;
  FUN_004ece30((undefined4 *)((int)this + 0x30));
  local_4._0_1_ = 4;
  FUN_004ece30((undefined4 *)((int)this + 0x34));
  local_4._0_1_ = 5;
  FUN_004ece30((undefined4 *)((int)this + 0x38));
  local_4._0_1_ = 6;
  FUN_004f5510((undefined4 *)((int)this + 0x3c));
  local_4._0_1_ = 7;
  FUN_004f5510((undefined4 *)((int)this + 0x44));
  local_4._0_1_ = 8;
  FUN_004f5510((undefined4 *)((int)this + 0x4c));
  local_4 = CONCAT31(local_4._1_3_,9);
  FUN_004355d0((undefined4 *)((int)this + 0x54));
  *(undefined4 *)((int)this + 0x68) = param_2;
  *(undefined4 *)((int)this + 0x24) = param_1;
  *(undefined ***)this = &PTR_FUN_0065a4d0;
  *(undefined4 *)((int)this + 0x1c) = 0;
  *(undefined4 *)((int)this + 100) = param_3;
  *(undefined4 *)((int)this + 0x6c) = 0;
  ExceptionList = local_c;
  return this;
}

