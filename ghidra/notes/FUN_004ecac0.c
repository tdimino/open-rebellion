
void * __thiscall FUN_004ecac0(void *this,undefined4 *param_1,undefined4 param_2,undefined4 param_3)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063eb73;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f5590((undefined4 *)this);
  local_4 = 0;
  FUN_004ece30((undefined4 *)((int)this + 0x1c));
  local_4 = CONCAT31(local_4._1_3_,1);
  *(undefined ***)this = &PTR_FUN_0065ca40;
  FUN_004f26d0((undefined4 *)((int)this + 0x1c),param_1);
  *(undefined4 *)((int)this + 0x20) = param_2;
  *(undefined4 *)((int)this + 0x24) = param_3;
  ExceptionList = local_c;
  return this;
}

