
void * __thiscall FUN_004ec940(void *this,undefined4 param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063eb13;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f5590((undefined4 *)this);
  local_4 = 0;
  FUN_005f4950((undefined4 *)((int)this + 0x1c),0);
  local_4 = CONCAT31(local_4._1_3_,1);
  *(undefined ***)this = &PTR_FUN_0065ca30;
  FUN_005f5600(this,param_1);
  *(undefined4 *)((int)this + 0x1c) = 0;
  ExceptionList = local_c;
  return this;
}

