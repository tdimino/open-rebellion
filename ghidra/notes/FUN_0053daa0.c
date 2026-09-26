
bool __thiscall FUN_0053daa0(void *this,void *param_1)

{
  int iVar1;
  ushort uVar2;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00646750;
  local_c = ExceptionList;
  iVar1 = *(int *)((int)this + 0x1c);
  local_14 = 1;
  local_10 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_1,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  FUN_00520460(this,param_1);
  if ((iVar1 != 0) && (*(int *)((int)param_1 + 4) == -1)) {
    uVar2 = FUN_005f6340(*(int *)((int)this + 0x40));
    if (1 < uVar2) {
      local_14 = 1;
      local_10 = 0x15;
      local_4 = 1;
      FUN_00520580(param_1,&local_14);
      local_4 = 0xffffffff;
      FUN_00619730();
    }
  }
  ExceptionList = local_c;
  return *(int *)((int)param_1 + 4) == -1;
}

