
void * __thiscall FUN_00442130(void *this,undefined4 param_1,undefined4 *param_2)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062eae3;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_0060ba00(this,param_1);
  local_4 = 0;
  FUN_004ece30((undefined4 *)((int)this + 0x68));
  local_4 = CONCAT31(local_4._1_3_,1);
  *(undefined ***)this = &PTR_FUN_00659ba0;
  FUN_004f26d0((undefined4 *)((int)this + 0x68),param_2);
  *(undefined4 *)((int)this + 0x6c) = 1;
  *(undefined4 *)((int)this + 0x70) = 0;
  ExceptionList = local_c;
  return this;
}

