
undefined4 __thiscall FUN_004f80e0(void *this,void *param_1)

{
  void *pvVar1;
  bool bVar2;
  bool bVar3;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  int iVar4;
  undefined4 uVar5;
  int local_14;
  void *local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063fdb8;
  local_c = ExceptionList;
  bVar3 = true;
  ExceptionList = &local_c;
  FUN_004ece30(&local_14);
  local_4 = 0;
  bVar2 = FUN_005406d0(this);
  if (((CONCAT31(extraout_var,bVar2) != 0) && ((*(uint *)((int)this + 0x50) & 0x10000) != 0)) &&
     ((*(uint *)((int)this + 0x50) & 8) == 0)) {
    local_10 = (void *)0x0;
    bVar3 = FUN_004f6b50(this,(int *)&local_10);
    bVar3 = CONCAT31(extraout_var_00,bVar3) != 0;
    pvVar1 = this;
    if (((*(byte *)((int)this + 0x50) & 0x20) != 0) || (pvVar1 = local_10, local_10 != (void *)0x0))
    {
      FUN_004f26d0(&local_14,(undefined4 *)((int)pvVar1 + 0x3c));
    }
  }
  iVar4 = FUN_004f7120(this,&local_14,param_1);
  if ((iVar4 == 0) || (!bVar3)) {
    uVar5 = 0;
  }
  else {
    uVar5 = 1;
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return uVar5;
}

