
void * __thiscall FUN_004acd80(void *this,undefined4 *param_1,uint param_2)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006386d3;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004ecc00((undefined4 *)this);
  local_4 = 0;
  FUN_005f52c0((undefined4 *)((int)this + 0x1c));
  local_4 = CONCAT31(local_4._1_3_,1);
  *(undefined ***)this = &PTR_FUN_0065c038;
  FUN_004f26d0((void *)((int)this + 0x18),param_1);
  if (param_2 != 0) {
    FUN_004ace40(this,param_2);
  }
  ExceptionList = local_c;
  return this;
}

