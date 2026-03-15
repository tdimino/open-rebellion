
void * __thiscall FUN_004c7340(void *this,undefined4 param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063acb9;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004c7d20(this,0x15,param_1);
  local_4 = 0;
  FUN_004ece30((undefined4 *)((int)this + 0x40));
  local_4._0_1_ = 1;
  FUN_004ece30((undefined4 *)((int)this + 0x44));
  local_4._0_1_ = 2;
  FUN_0042dbe0((undefined4 *)((int)this + 0x48));
  local_4 = CONCAT31(local_4._1_3_,3);
  FUN_004355d0((undefined4 *)((int)this + 0x4c));
  *(undefined4 *)((int)this + 0x34) = 0;
  *(undefined4 *)((int)this + 0x3c) = 0;
  *(undefined4 *)((int)this + 0x38) = 0;
  *(undefined ***)this = &PTR_FUN_0065c650;
  ExceptionList = local_c;
  return this;
}

