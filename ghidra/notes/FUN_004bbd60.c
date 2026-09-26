
void * __thiscall FUN_004bbd60(void *this,undefined4 param_1,undefined4 param_2,undefined4 param_3)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00639b09;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f5590(this);
  local_4 = 0;
  FUN_005f4950((void *)((int)this + 0x20),0);
  local_4._0_1_ = 1;
  FUN_004ece30((undefined4 *)((int)this + 0x28));
  local_4._0_1_ = 2;
  FUN_004ece30((undefined4 *)((int)this + 0x2c));
  local_4 = CONCAT31(local_4._1_3_,3);
  FUN_004355d0((undefined4 *)((int)this + 0x30));
  *(undefined4 *)((int)this + 0x44) = param_2;
  *(undefined4 *)((int)this + 0x24) = param_1;
  *(undefined ***)this = &PTR_FUN_0065c358;
  *(undefined4 *)((int)this + 0x1c) = 0;
  *(undefined4 *)((int)this + 0x40) = param_3;
  ExceptionList = local_c;
  return this;
}

