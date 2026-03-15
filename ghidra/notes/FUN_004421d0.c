
void * __thiscall FUN_004421d0(void *this,uint *param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062eb03;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_0060ba00(this,*param_1 & 0xffffff);
  local_4 = 0;
  FUN_004ece30((undefined4 *)((int)this + 0x68));
  local_4 = CONCAT31(local_4._1_3_,1);
  *(undefined ***)this = &PTR_FUN_00659ba0;
  FUN_004f26d0((undefined4 *)((int)this + 0x68),param_1);
  *(undefined4 *)((int)this + 0x70) = 0;
  *(undefined4 *)((int)this + 0x6c) = 1;
  ExceptionList = local_c;
  return this;
}

