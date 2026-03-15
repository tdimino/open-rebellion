
void * __thiscall FUN_00612280(void *this,LPCSTR param_1,int param_2,undefined4 param_3,int param_4)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00656f0e;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f5c10((undefined4 *)this);
  local_4 = 0;
  FUN_00612c20((undefined4 *)((int)this + 0x20));
  local_4._0_1_ = 1;
  FUN_00612aa0((undefined4 *)((int)this + 0x40));
  local_4 = CONCAT31(local_4._1_3_,2);
  *(undefined ***)this = &PTR_FUN_0066e3b8;
  FUN_006123d0(this,param_1,param_2,param_3,param_4);
  ExceptionList = local_c;
  return this;
}

