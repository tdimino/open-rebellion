
int __thiscall FUN_005820a0(void *this,void *param_1)

{
  int iVar1;
  undefined4 local_1c;
  undefined4 local_18;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_0064ee38;
  local_c = ExceptionList;
  local_1c = 1;
  local_18 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_1,&local_1c);
  local_4 = 0xffffffff;
  FUN_00619730();
  iVar1 = FUN_00520040(this,param_1);
  if (iVar1 != 0) {
    if (*(int *)((int)param_1 + 4) == -1) {
      FUN_00541e70(&local_1c);
      local_4 = 1;
      FUN_0051fcb0(this,&local_1c);
      iVar1 = FUN_005539f0((int)&local_1c,*(void **)((int)this + 0x44));
      local_4 = 0xffffffff;
      FUN_00541ea0();
    }
    if (iVar1 != 0) {
      ExceptionList = local_c;
      return iVar1;
    }
  }
  local_1c = 1;
  local_18 = 1;
  local_4 = 2;
  FUN_00520580(param_1,&local_1c);
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return iVar1;
}

