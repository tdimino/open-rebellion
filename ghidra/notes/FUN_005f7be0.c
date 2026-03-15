
void * __thiscall FUN_005f7be0(void *this,int param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006558e3;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f5590((undefined4 *)this);
  local_4 = 0;
  FUN_005f2f50((undefined4 *)((int)this + 0x2c));
  *(undefined ***)this = &PTR_FUN_0066de20;
  local_4 = CONCAT31(local_4._1_3_,1);
  *(undefined4 *)((int)this + 0x1c) = *(undefined4 *)(param_1 + 0x1c);
  *(undefined4 *)((int)this + 0x20) = *(undefined4 *)(param_1 + 0x20);
  *(undefined4 *)((int)this + 0x24) = *(undefined4 *)(param_1 + 0x24);
  *(undefined4 *)((int)this + 0x28) = *(undefined4 *)(param_1 + 0x28);
  FUN_005f3090((undefined4 *)((int)this + 0x2c),param_1 + 0x2c);
  ExceptionList = local_c;
  return this;
}

