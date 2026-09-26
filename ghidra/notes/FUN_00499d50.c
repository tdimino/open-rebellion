
void * __thiscall FUN_00499d50(void *this,int param_1,undefined4 param_2)

{
  undefined4 uVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006369b8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004c4b50(this,param_2);
  local_4 = 0;
  *(undefined ***)this = &PTR_FUN_0065bb88;
  uVar1 = FUN_00526670();
  *(undefined4 *)((int)this + 0x34) = uVar1;
  FUN_004974f0(this,param_1);
  ExceptionList = local_c;
  return this;
}

