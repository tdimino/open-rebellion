
void * __thiscall FUN_004c4b50(void *this,undefined4 param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063a749;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004c4d90(this,param_1);
  local_4 = 0;
  FUN_005f2f50((undefined4 *)((int)this + 0x44));
  local_4._0_1_ = 1;
  FUN_005f2f50((undefined4 *)((int)this + 0x50));
  local_4._0_1_ = 2;
  FUN_004ece30((undefined4 *)((int)this + 0x5c));
  local_4 = CONCAT31(local_4._1_3_,3);
  FUN_0042dbe0((undefined4 *)((int)this + 0x60));
  *(undefined ***)this = &PTR_FUN_0065c4f0;
  ExceptionList = local_c;
  return this;
}

