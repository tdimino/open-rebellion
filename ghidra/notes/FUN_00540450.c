
undefined4 __thiscall FUN_00540450(void *this,undefined4 *param_1)

{
  uint *puVar1;
  void *local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00646aa8;
  local_c = ExceptionList;
  local_10 = this;
  if (*(void **)((int)this + 0x1c) == (void *)0x0) {
    ExceptionList = &local_c;
    FUN_004ece80(param_1);
  }
  else {
    ExceptionList = &local_c;
    puVar1 = FUN_004025b0(*(void **)((int)this + 0x1c),(uint *)&local_10);
    local_4 = 0;
    FUN_004f26d0(param_1,puVar1);
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  ExceptionList = local_c;
  return 1;
}

