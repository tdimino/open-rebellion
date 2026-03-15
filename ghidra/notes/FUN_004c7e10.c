
void * __thiscall FUN_004c7e10(void *this,uint *param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063adde;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_0060ba00(this,*param_1 & 0xffffff);
  local_4 = 0;
  FUN_005f4950((void *)((int)this + 0x68),0);
  local_4._0_1_ = 1;
  FUN_004ece30((undefined4 *)((int)this + 0x6c));
  local_4 = CONCAT31(local_4._1_3_,2);
  *(undefined ***)this = &PTR_FUN_0065c690;
  FUN_004f26d0((undefined4 *)((int)this + 0x6c),param_1);
  ExceptionList = local_c;
  return this;
}

