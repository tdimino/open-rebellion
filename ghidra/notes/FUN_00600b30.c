
void __thiscall FUN_00600b30(void *this)

{
  int iVar1;
  undefined4 in_stack_00000018;
  undefined4 local_18 [3];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00656058;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f2fc0(local_18,in_stack_00000018);
  local_4 = 0;
  iVar1 = FUN_00583c40((int)local_18);
  if (iVar1 != 0) {
    FUN_00583c40((int)local_18);
    FUN_00600bd0(this);
  }
  local_4 = 0xffffffff;
  FUN_005f2ff0(local_18);
  ExceptionList = local_c;
  return;
}

