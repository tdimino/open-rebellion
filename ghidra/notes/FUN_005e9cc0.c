
void * __thiscall FUN_005e9cc0(void *this,int param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00655278;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f5590(this);
  *(undefined ***)this = &PTR_FUN_0066d820;
  *(int *)((int)this + 0x1c) = param_1;
  local_4 = 0;
  FUN_005f5600(this,*(undefined4 *)(param_1 + 0xec));
  ExceptionList = local_c;
  return this;
}

