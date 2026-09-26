
undefined4 __thiscall FUN_0054fca0(void *this,int param_1,int param_2,int param_3)

{
  int iVar1;
  undefined4 *puVar2;
  void *this_00;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_006488c0;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *(undefined4 *)((int)this + 4) = 1;
  *(undefined4 *)((int)this + 8) = 0;
  local_14 = 1;
  local_10 = 0xffffffff;
  local_4 = 0;
  this_00 = (void *)((int)this + 0xc);
  FUN_00520580(this_00,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  local_14 = 1;
  local_10 = 0xffffffff;
  local_4 = 1;
  FUN_00520580((void *)((int)this + 0x14),&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  if (*(int *)(param_2 + 4) == 0) {
    ExceptionList = local_c;
    return 1;
  }
  if (*(int *)(param_1 + 0x10) == -1) {
    if (*(int *)(param_1 + 4) != 0) {
      ExceptionList = local_c;
      return 1;
    }
    *(undefined4 *)((int)this + 4) = 0;
    iVar1 = *(int *)(param_2 + 8);
    if (param_3 != 0) {
      iVar1 = -iVar1;
    }
    iVar1 = *(int *)(param_1 + 8) + iVar1;
    *(int *)((int)this + 8) = iVar1;
    if (-1 < iVar1) {
      ExceptionList = local_c;
      return 1;
    }
    puVar2 = (undefined4 *)(param_1 + 0x14);
    this_00 = (void *)((int)this + 0x14);
  }
  else {
    puVar2 = (undefined4 *)(param_1 + 0xc);
  }
  FUN_00520580(this_00,puVar2);
  ExceptionList = local_c;
  return 1;
}

