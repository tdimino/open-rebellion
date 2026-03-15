
void * __thiscall FUN_004ad7e0(void *this,undefined4 *param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006387be;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f5c10((undefined4 *)this);
  local_4 = 0;
  FUN_005f4950((void *)((int)this + 0x18),0);
  local_4._0_1_ = 1;
  FUN_0042dbe0((undefined4 *)((int)this + 0x2c));
  local_4 = CONCAT31(local_4._1_3_,2);
  *(undefined ***)this = &PTR_FUN_0065c0a8;
  FUN_004f26d0((undefined4 *)((int)this + 0x2c),param_1);
  *(undefined4 *)((int)this + 0x1c) = 0;
  *(undefined4 *)((int)this + 0x20) = 0;
  *(undefined4 *)((int)this + 0x24) = 0;
  *(undefined4 *)((int)this + 0x28) = 0;
  ExceptionList = local_c;
  return this;
}

