
int __thiscall FUN_005236e0(void *this,int *param_1,void *param_2)

{
  int iVar1;
  bool bVar2;
  undefined3 extraout_var;
  uint uVar4;
  undefined4 local_14;
  uint local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  int iVar3;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00643b48;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  bVar2 = FUN_004f9190(this,param_1,param_2);
  iVar3 = CONCAT31(extraout_var,bVar2);
  if (iVar3 != 0) {
    iVar1 = *(int *)((int)this + 0x54);
    uVar4 = FUN_00520b40((int)this);
    FUN_00540230((void *)(iVar1 + 0x20),uVar4);
    iVar1 = *(int *)((int)this + 0x54);
    uVar4 = FUN_00520b50((int)this);
    FUN_00540270((void *)(iVar1 + 0x20),uVar4);
    FUN_00540230((void *)(*(int *)((int)this + 0x54) + 0x28),1);
    FUN_00540270((void *)(*(int *)((int)this + 0x54) + 0x28),0);
    if (iVar3 != 0) {
      FUN_005834d0(&local_14);
      local_4 = 0;
      local_10 = (local_10 & 0xfffffff0 ^ 2) & 0xffff411f | 0x4110;
      FUN_00522a60(this,&local_14);
      local_4 = 0xffffffff;
      FUN_00583500(&local_14);
    }
  }
  ExceptionList = local_c;
  return iVar3;
}

