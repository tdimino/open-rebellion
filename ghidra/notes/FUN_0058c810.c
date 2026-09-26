
int __thiscall FUN_0058c810(void *this,undefined4 param_1)

{
  int iVar1;
  int local_1c [3];
  int local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064ff58;
  local_c = ExceptionList;
  iVar1 = 1;
  if ((*(int *)((int)this + 0x24) != 0) && (*(int *)((int)this + 0x28) == 0)) {
    ExceptionList = &local_c;
    iVar1 = FUN_00559fe0(*(int *)((int)this + 0x14),0,0,0,0);
    FUN_0058bdb0(local_1c,(void *)((int)this + 8));
    local_4 = 0;
    local_10 = iVar1;
    iVar1 = FUN_0058ba10((void *)((int)this + 8),local_1c,param_1);
    local_4 = 0xffffffff;
    FUN_0058be00(local_1c);
  }
  ExceptionList = local_c;
  return iVar1;
}

