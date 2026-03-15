
void * __thiscall FUN_00594b10(void *this,int param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00650bb3;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_0054f530(this,param_1);
  local_4 = 0;
  FUN_004f26d0((void *)((int)this + 0x40),(undefined4 *)(param_1 + 0x40));
  local_4 = CONCAT31(local_4._1_3_,1);
  FUN_004f26d0((void *)((int)this + 0x44),(undefined4 *)(param_1 + 0x44));
  *(undefined4 *)((int)this + 0x48) = *(undefined4 *)(param_1 + 0x48);
  *(undefined ***)this = &PTR_FUN_0066ba18;
  ExceptionList = local_c;
  return this;
}

