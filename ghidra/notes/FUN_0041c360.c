
void * __thiscall FUN_0041c360(void *this,undefined4 *param_1,undefined4 param_2)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062b733;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f5270((undefined4 *)this);
  local_4 = 0;
  FUN_004ece30((undefined4 *)((int)this + 0xc));
  local_4 = CONCAT31(local_4._1_3_,1);
  *(undefined ***)this = &PTR_FUN_00658a78;
  FUN_004f26d0((undefined4 *)((int)this + 0xc),param_1);
  *(undefined4 *)((int)this + 0x14) = 0;
  *(undefined4 *)((int)this + 0x10) = param_2;
  ExceptionList = local_c;
  return this;
}

