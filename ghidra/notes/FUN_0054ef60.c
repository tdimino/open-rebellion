
void * __thiscall FUN_0054ef60(void *this,int param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00648703;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f5c10((undefined4 *)this);
  local_4 = 0;
  *(undefined4 *)((int)this + 0x18) = *(undefined4 *)(param_1 + 0x18);
  *(undefined4 *)((int)this + 0x1c) = *(undefined4 *)(param_1 + 0x1c);
  FUN_004fd450((void *)((int)this + 0x20),param_1 + 0x20);
  local_4 = CONCAT31(local_4._1_3_,1);
  FUN_004f26d0((void *)((int)this + 0x3c),(undefined4 *)(param_1 + 0x3c));
  *(undefined ***)this = &PTR_FUN_00662038;
  ExceptionList = local_c;
  return this;
}

