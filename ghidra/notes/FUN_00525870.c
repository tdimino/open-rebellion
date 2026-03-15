
void * __thiscall
FUN_00525870(void *this,undefined4 param_1,undefined4 *param_2,undefined4 param_3,undefined4 param_4
            ,undefined4 param_5)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00643dfb;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *(undefined4 *)((int)this + 4) = param_1;
  FUN_00520580((void *)((int)this + 8),param_2);
  *(undefined4 *)((int)this + 0x18) = param_5;
  local_4 = 0;
  *(undefined4 *)((int)this + 0x10) = param_3;
  *(undefined4 *)((int)this + 0x14) = param_4;
  *(undefined4 *)((int)this + 0x1c) = 0;
  *(undefined4 *)((int)this + 0x20) = 0;
  *(undefined4 *)((int)this + 0x24) = 0;
  FUN_004ece30((undefined4 *)((int)this + 0x28));
  *(undefined4 *)((int)this + 0x2c) = 0;
  *(undefined ***)this = &PTR_FUN_0065f070;
  ExceptionList = local_c;
  return this;
}

