
void * __thiscall FUN_0049a1b0(void *this,undefined4 param_1,undefined4 param_2)

{
  undefined4 uVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00636a5e;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004c4d90(this,param_1);
  local_4 = 0;
  FUN_005f2f50((undefined4 *)((int)this + 0x44));
  local_4._0_1_ = 1;
  FUN_005f2f50((undefined4 *)((int)this + 0x50));
  local_4 = CONCAT31(local_4._1_3_,2);
  *(undefined ***)this = &PTR_FUN_0065bbd0;
  uVar1 = FUN_006158b0();
  *(undefined4 *)((int)this + 0x34) = uVar1;
  *(undefined4 *)((int)this + 0x5c) = param_2;
  FUN_0049a350(this);
  ExceptionList = local_c;
  return this;
}

