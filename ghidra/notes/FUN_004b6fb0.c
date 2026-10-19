
void * __thiscall FUN_004b6fb0(void *this,undefined4 param_1,undefined4 param_2,undefined4 param_3)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006395a8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004ad8e0(this,param_1,param_2,param_3);
  local_4 = 0;
  FUN_004ece30((undefined4 *)((int)this + 0x5c));
  *(undefined ***)this = &PTR_FUN_0065c210;
  ExceptionList = local_c;
  return this;
}

