
void * __thiscall FUN_004e52c0(void *this,undefined4 param_1,undefined4 param_2)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063e30b;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f4950((void *)((int)this + 0x10),0);
  local_4 = 0;
  FUN_004355d0((undefined4 *)((int)this + 0x1c));
  *(undefined4 *)((int)this + 0xc) = param_2;
  *(undefined4 *)((int)this + 4) = param_1;
  *(undefined4 *)((int)this + 8) = 0;
  *(undefined4 *)((int)this + 0x18) = 0;
  *(undefined4 *)((int)this + 0x14) = 0;
  *(undefined ***)this = &PTR_FUN_0065c898;
  ExceptionList = local_c;
  return this;
}

