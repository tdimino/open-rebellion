
void * __thiscall FUN_0049e1c0(void *this,undefined4 param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063721e;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004c7d20(this,0x17,param_1);
  local_4 = 0;
  FUN_004ece30((undefined4 *)((int)this + 0x3c));
  local_4._0_1_ = 1;
  FUN_004ece30((undefined4 *)((int)this + 0x40));
  local_4 = CONCAT31(local_4._1_3_,2);
  FUN_004355d0((undefined4 *)((int)this + 0x44));
  *(undefined ***)this = &PTR_FUN_0065bc98;
  *(undefined4 *)((int)this + 0x54) = 0;
  *(undefined4 *)((int)this + 0x38) = 0;
  ExceptionList = local_c;
  return this;
}

