
void * __thiscall FUN_0054fe80(void *this,int param_1)

{
  int iVar1;
  void *this_00;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006488e6;
  local_c = ExceptionList;
  this_00 = (void *)((int)this + 4);
  ExceptionList = &local_c;
  FUN_00618c80(this_00,0x1c,9,FUN_0054f830);
  local_4 = 0;
  *(undefined ***)this = &PTR_FUN_006620e8;
  iVar1 = 9;
  do {
    FUN_0054fae0(this_00,(param_1 - (int)this) + (int)this_00);
    this_00 = (void *)((int)this_00 + 0x1c);
    iVar1 = iVar1 + -1;
  } while (iVar1 != 0);
  ExceptionList = local_c;
  return this;
}

