
void * __thiscall FUN_0048a6e0(void *this,undefined4 param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00634f1b;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_00536eb0((undefined4 *)((int)this + 4));
  local_4 = 0;
  FUN_005f4950((void *)((int)this + 0x14),0);
  *(undefined4 *)((int)this + 0x24) = param_1;
  *(undefined4 *)((int)this + 0x18) = 0;
  *(undefined4 *)((int)this + 0x1c) = 0;
  *(undefined4 *)((int)this + 0x20) = 0;
  *(undefined4 *)((int)this + 0x28) = 0;
  *(undefined4 *)((int)this + 0x2c) = 0;
  *(undefined ***)this = &PTR_FUN_0065b070;
  ExceptionList = local_c;
  return this;
}

