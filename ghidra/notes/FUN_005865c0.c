
void * __thiscall FUN_005865c0(void *this,int param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064f703;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_00594b10(this,param_1);
  local_4 = 0;
  FUN_004f26d0((void *)((int)this + 0x4c),(undefined4 *)(param_1 + 0x4c));
  local_4 = CONCAT31(local_4._1_3_,1);
  FUN_004f26d0((void *)((int)this + 0x50),(undefined4 *)(param_1 + 0x50));
  *(undefined ***)this = &PTR_FUN_0066a298;
  ExceptionList = local_c;
  return this;
}

