
void * __thiscall FUN_004988f0(void *this,int param_1,undefined4 param_2)

{
  undefined4 uVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006366b8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004c4b50(this,param_2);
  local_4 = 0;
  *(undefined ***)this = &PTR_FUN_0065b990;
  uVar1 = FUN_0048c2a0();
  *(undefined4 *)((int)this + 0x34) = uVar1;
  *(undefined4 *)((int)this + 100) = 0;
  *(undefined4 *)((int)this + 0x40) = 0;
  FUN_0048bbf0(this,param_1);
  ExceptionList = local_c;
  return this;
}

