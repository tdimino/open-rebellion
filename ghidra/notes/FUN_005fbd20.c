
void * __thiscall FUN_005fbd20(void *this,undefined4 param_1,uint param_2,undefined1 param_3)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00655eab;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_0060e5e0((void *)((int)this + 9),0,0,0);
  local_4 = 0;
  FUN_005fc500(this,param_1,param_3);
  FUN_005fc050(this,param_2);
  if (*(int *)((int)this + 4) != 0) {
    FUN_005fd170((uint *)this);
  }
  *(undefined4 *)((int)this + 0x1c) = 0;
  *(undefined4 *)((int)this + 0x24) = 0;
  ExceptionList = local_c;
  return this;
}

