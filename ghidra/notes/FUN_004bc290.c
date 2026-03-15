
void * __thiscall FUN_004bc290(void *this,undefined4 param_1,undefined4 param_2,undefined4 param_3)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00639be3;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004bbd60(this,param_1,param_2,param_3);
  local_4 = 0;
  FUN_004ece30((undefined4 *)((int)this + 0x48));
  local_4 = CONCAT31(local_4._1_3_,1);
  FUN_004ece30((undefined4 *)((int)this + 0x58));
  *(undefined4 *)((int)this + 0x54) = 0;
  *(undefined4 *)((int)this + 0x4c) = 0;
  *(undefined4 *)((int)this + 0x50) = 0;
  *(undefined ***)this = &PTR_FUN_0065c380;
  ExceptionList = local_c;
  return this;
}

