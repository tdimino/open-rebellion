
void * __thiscall FUN_004f5550(void *this,int param_1)

{
  int *piVar1;
  void *pvVar2;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063f938;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f4ed0((undefined4 *)this);
  local_4 = 0;
  *(undefined ***)this = &PTR_FUN_0065d1c8;
  for (piVar1 = (int *)thunk_FUN_005f5060(param_1); piVar1 != (int *)0x0; piVar1 = (int *)piVar1[4])
  {
    pvVar2 = (void *)(**(code **)(*piVar1 + 4))();
    if (pvVar2 != (void *)0x0) {
      FUN_004f57b0(this,pvVar2);
    }
  }
  ExceptionList = local_c;
  return this;
}

