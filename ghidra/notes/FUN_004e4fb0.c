
void * __thiscall FUN_004e4fb0(void *this,undefined4 param_1,undefined4 param_2)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063e2a8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004ec940(this,param_1);
  local_4 = 0;
  FUN_005f4950((void *)((int)this + 0x28),0);
  *(undefined4 *)((int)this + 0x20) = 0;
  *(undefined4 *)((int)this + 0x30) = param_2;
  *(undefined4 *)((int)this + 0x34) = 0;
  *(undefined4 *)((int)this + 0x24) = 0;
  *(undefined4 *)((int)this + 0x2c) = 0;
  *(undefined ***)this = &PTR_FUN_0065c868;
  ExceptionList = local_c;
  return this;
}

