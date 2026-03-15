
int __thiscall FUN_0058c500(void *this,undefined4 param_1)

{
  int iVar1;
  int local_14 [2];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064ff08;
  local_c = ExceptionList;
  iVar1 = 1;
  if ((*(int *)((int)this + 0x28) != 0) && (*(int *)((int)this + 0x24) != 0)) {
    ExceptionList = &local_c;
    FUN_0058be70(local_14,(void *)((int)this + 8));
    local_4 = 0;
    iVar1 = FUN_0058ba10((void *)((int)this + 8),local_14,param_1);
    local_4 = 0xffffffff;
    FUN_0058beb0(local_14);
  }
  ExceptionList = local_c;
  return iVar1;
}

