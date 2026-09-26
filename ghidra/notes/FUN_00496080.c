
void * __thiscall FUN_00496080(void *this,int param_1,undefined4 param_2)

{
  undefined4 uVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006361e8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004c4b50(this,param_2);
  local_4 = 0;
  *(undefined ***)this = &PTR_FUN_0065b828;
  *(undefined4 *)((int)this + 0x40) = 0;
  uVar1 = FUN_0048bc60();
  *(undefined4 *)((int)this + 0x34) = uVar1;
  FUN_004974f0(this,param_1);
  ExceptionList = local_c;
  return this;
}

