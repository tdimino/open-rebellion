
void * __thiscall FUN_005fbc80(void *this,undefined4 param_1,LPCSTR param_2,byte param_3)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00655e8b;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_0060e5e0((void *)((int)this + 9),0,0,0);
  local_4 = 0;
  FUN_005fc500(this,param_1,param_3);
  if (param_2 != (LPCSTR)0x0) {
    if ((param_3 & 0xf7) == 1) {
      FUN_005fc530(this,param_2);
    }
    else if ((param_3 & 0xf7) == 2) {
      FUN_005fc6a0(this,param_2);
    }
    else {
      *(undefined1 *)((int)this + 0xc) = 0;
    }
  }
  *(undefined4 *)((int)this + 0x1c) = 0;
  if (*(int *)((int)this + 4) != 0) {
    FUN_005fd170(this);
  }
  *(undefined4 *)((int)this + 0x24) = 0;
  ExceptionList = local_c;
  return this;
}

