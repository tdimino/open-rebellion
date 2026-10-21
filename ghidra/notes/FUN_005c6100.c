
void * __thiscall FUN_005c6100(void *this,int *param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00653c98;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f52c0(this);
  local_4 = 0;
  *(undefined ***)this = &PTR_FUN_0066bdd0;
  FUN_005c6090(this,param_1);
  ExceptionList = local_c;
  return this;
}

