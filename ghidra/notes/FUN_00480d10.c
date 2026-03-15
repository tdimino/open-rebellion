
void * __thiscall FUN_00480d10(void *this,undefined4 param_1,undefined4 param_2,undefined4 param_3)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063408b;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004bce70(this,param_1,param_2,param_3);
  local_4 = 0;
  FUN_004ece30((undefined4 *)((int)this + 0x70));
  *(undefined ***)this = &PTR_FUN_0065a8a8;
  param_3 = 0x52000013;
  local_4._0_1_ = 2;
  FUN_004f26d0((void *)((int)this + 0x2c),&param_3);
  local_4 = CONCAT31(local_4._1_3_,1);
  FUN_00619730();
  ExceptionList = local_c;
  return this;
}

