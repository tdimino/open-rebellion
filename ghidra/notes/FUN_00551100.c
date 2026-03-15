
undefined4 __thiscall
FUN_00551100(void *this,undefined4 param_1,undefined4 param_2,undefined4 param_3,undefined4 *param_4
            )

{
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00648b98;
  local_c = ExceptionList;
  local_14 = 1;
  ExceptionList = &local_c;
  *(undefined4 *)((int)this + 0x14) = param_1;
  *(undefined4 *)((int)this + 0x18) = param_2;
  *(undefined4 *)((int)this + 0x1c) = param_3;
  local_10 = 0xffffffff;
  local_4 = 0;
  FUN_00520580((void *)((int)this + 0xc),&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  *(undefined4 *)((int)this + 0x20) = *param_4;
  ExceptionList = local_c;
  return 1;
}

