
void * __thiscall FUN_004ecc70(void *this,undefined4 *param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063ebc8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f5c10(this);
  local_4 = 0;
  FUN_004f26d0((void *)((int)this + 0x18),param_1);
  *(undefined ***)this = &PTR_FUN_0065ca50;
  ExceptionList = local_c;
  return this;
}

