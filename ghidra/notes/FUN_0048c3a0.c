
void * __thiscall FUN_0048c3a0(void *this,int param_1,undefined4 param_2)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00635343;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004c4b50(this,param_2);
  local_4 = 0;
  FUN_004ece30((undefined4 *)((int)this + 0x68));
  local_4 = CONCAT31(local_4._1_3_,1);
  *(undefined ***)this = &PTR_FUN_0065b2a8;
  *(undefined4 *)((int)this + 100) = 0;
  *(undefined4 *)((int)this + 0x40) = 0;
  FUN_004974f0(this,param_1);
  ExceptionList = local_c;
  return this;
}

