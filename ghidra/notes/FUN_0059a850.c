
void __thiscall FUN_0059a850(void *this,undefined4 param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00651878;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *(undefined4 *)((int)this + 0x8d0) = param_1;
  local_4 = 0;
  FUN_005f3090((void *)((int)this + 0x8dc),(int)&stack0x00000008);
  local_4 = 0xffffffff;
  FUN_005f2ff0((undefined4 *)&stack0x00000008);
  ExceptionList = local_c;
  return;
}

