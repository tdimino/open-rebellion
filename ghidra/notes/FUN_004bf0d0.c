
void * __thiscall FUN_004bf0d0(void *this,undefined4 param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063a06e;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004e4fb0(this,0xd,param_1);
  local_4 = 0;
  FUN_004355d0((undefined4 *)((int)this + 0x40));
  local_4._0_1_ = 1;
  FUN_004ece30((undefined4 *)((int)this + 0x50));
  local_4 = CONCAT31(local_4._1_3_,2);
  FUN_004e5110((undefined4 *)((int)this + 100));
  *(undefined4 *)((int)this + 0x38) = 0;
  *(undefined4 *)((int)this + 0x3c) = 0;
  *(undefined4 *)((int)this + 0x58) = 0;
  *(undefined4 *)((int)this + 0x5c) = 0;
  *(undefined4 *)((int)this + 0x60) = 0;
  *(undefined ***)this = &PTR_FUN_0065c480;
  *(undefined4 *)((int)this + 0x54) = 200;
  ExceptionList = local_c;
  return this;
}

