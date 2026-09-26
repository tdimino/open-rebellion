
void * __thiscall FUN_0060ba00(void *this,undefined4 param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006569e3;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f5270(this);
  local_4 = 0;
  FUN_005f2f50((undefined4 *)((int)this + 0x14));
  local_4 = CONCAT31(local_4._1_3_,1);
  FUN_005f4950((void *)((int)this + 0x3c),0);
  *(undefined4 *)((int)this + 0xc) = param_1;
  *(undefined4 *)((int)this + 0x20) = 0;
  *(undefined4 *)((int)this + 0x24) = 0;
  *(undefined2 *)((int)this + 0x10) = 0;
  *(undefined2 *)((int)this + 0x12) = 0;
  *(undefined4 *)((int)this + 0x28) = 0;
  *(undefined4 *)((int)this + 0x2c) = 0;
  *(undefined4 *)((int)this + 0x30) = 0;
  *(undefined4 *)((int)this + 0x34) = 0;
  *(undefined4 *)((int)this + 0x50) = 0;
  *(undefined4 *)((int)this + 0x38) = 0;
  *(undefined4 *)((int)this + 0x54) = 0;
  *(undefined4 *)((int)this + 0x58) = 0;
  *(undefined4 *)((int)this + 0x5c) = 0xffffff;
  *(undefined4 *)((int)this + 0x60) = 0xffffff;
  *(undefined4 *)((int)this + 100) = 0xffffff;
  *(undefined ***)this = &PTR_FUN_0066e1b0;
  ExceptionList = local_c;
  return this;
}

