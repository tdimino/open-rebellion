
void * __thiscall FUN_004884f0(void *this,int param_1)

{
  int iVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00634b1e;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_0048a6e0(this,param_1);
  local_4 = 0;
  FUN_0042dbe0((undefined4 *)((int)this + 0x3c));
  local_4._0_1_ = 1;
  FUN_004ece30((undefined4 *)((int)this + 0x40));
  local_4 = CONCAT31(local_4._1_3_,2);
  *(undefined ***)this = &PTR_FUN_0065aef0;
  if (param_1 == 1) {
    iVar1 = 1;
  }
  else {
    iVar1 = 0x77359401;
  }
  FUN_004885e0(this,iVar1);
  *(undefined4 *)((int)this + 0x38) = 0;
  *(undefined4 *)((int)this + 0x44) = 0;
  *(undefined4 *)((int)this + 0x48) = 0;
  *(undefined4 *)((int)this + 0x4c) = 0;
  ExceptionList = local_c;
  return this;
}

