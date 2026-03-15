
void * __thiscall FUN_00494f80(void *this,int param_1,undefined4 param_2)

{
  undefined4 uVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00635fd3;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004c50d0(this,param_2);
  local_4 = 0;
  FUN_004ece30((undefined4 *)((int)this + 0x68));
  local_4 = CONCAT31(local_4._1_3_,1);
  *(undefined ***)this = &PTR_FUN_0065b758;
  *(undefined4 *)((int)this + 0x40) = 0;
  uVar1 = FUN_0048e670();
  *(undefined4 *)((int)this + 0x34) = uVar1;
  *(undefined4 *)((int)this + 100) = 0;
  *(undefined4 *)((int)this + 0x6c) = 0;
  FUN_004974f0(this,param_1);
  ExceptionList = local_c;
  return this;
}

