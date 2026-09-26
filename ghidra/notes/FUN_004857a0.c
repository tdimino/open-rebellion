
void * __thiscall FUN_004857a0(void *this,int param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063480c;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004884f0(this,param_1);
  local_4 = 0;
  FUN_004bee50((undefined4 *)((int)this + 0x54));
  local_4._0_1_ = 1;
  FUN_004be7e0((undefined4 *)((int)this + 100));
  local_4._0_1_ = 2;
  FUN_004178f0((undefined4 *)((int)this + 0x84));
  local_4 = CONCAT31(local_4._1_3_,3);
  *(undefined ***)this = &PTR_FUN_0065abf0;
  *(undefined4 *)((int)this + 0x50) = 1;
  *(undefined4 *)((int)this + 0x80) = 0;
  FUN_00417cb0((undefined4 *)((int)this + 0x84),param_1);
  *(int *)((int)this + 0x70) = param_1;
  *(undefined4 *)((int)this + 0x418) = 0;
  *(undefined4 *)((int)this + 0x41c) = 0;
  *(undefined4 *)((int)this + 0x420) = 0;
  *(undefined4 *)((int)this + 0x424) = 0;
  *(undefined4 *)((int)this + 0x428) = 0;
  ExceptionList = local_c;
  return this;
}

