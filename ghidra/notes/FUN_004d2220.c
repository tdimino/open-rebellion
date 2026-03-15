
void * __thiscall FUN_004d2220(void *this,undefined4 param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063c0a8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004e4fb0(this,4,param_1);
  local_4 = 0;
  FUN_004355d0((undefined4 *)((int)this + 0x50));
  *(undefined4 *)((int)this + 0x38) = 0;
  *(undefined4 *)((int)this + 0x3c) = 0;
  *(undefined4 *)((int)this + 0x40) = 0;
  *(undefined4 *)((int)this + 0x44) = 0;
  *(undefined4 *)((int)this + 0x48) = 0;
  *(undefined4 *)((int)this + 0x4c) = 0;
  *(undefined ***)this = &PTR_FUN_0065c778;
  ExceptionList = local_c;
  return this;
}

