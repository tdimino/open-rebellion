
void * __thiscall
FUN_005f79b0(void *this,undefined4 param_1,char *param_2,undefined4 param_3,undefined4 param_4,
            undefined4 param_5)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00655863;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f5590((undefined4 *)this);
  local_4 = 0;
  FUN_005f2f50((undefined4 *)((int)this + 0x2c));
  local_4 = CONCAT31(local_4._1_3_,1);
  *(undefined ***)this = &PTR_FUN_0066de1c;
  *(undefined4 *)((int)this + 0x1c) = param_1;
  FUN_005f35e0((undefined4 *)((int)this + 0x2c),param_2);
  *(undefined4 *)((int)this + 0x28) = param_5;
  *(undefined4 *)((int)this + 0x24) = param_4;
  *(undefined4 *)((int)this + 0x20) = param_3;
  ExceptionList = local_c;
  return this;
}

