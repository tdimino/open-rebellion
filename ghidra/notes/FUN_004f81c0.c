
undefined4 __thiscall FUN_004f81c0(void *this,void *param_1)

{
  bool bVar1;
  bool bVar2;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  int iVar3;
  void *local_4;
  
  bVar2 = true;
  iVar3 = 0;
  local_4 = this;
  bVar1 = FUN_005406d0((int *)this);
  if (((CONCAT31(extraout_var,bVar1) != 0) && ((*(uint *)((int)this + 0x50) & 0x10000) != 0)) &&
     ((*(uint *)((int)this + 0x50) & 8) == 0)) {
    local_4 = (void *)0x0;
    bVar2 = FUN_004f6b50(this,(int *)&local_4);
    bVar2 = CONCAT31(extraout_var_00,bVar2) != 0;
    if ((*(byte *)((int)this + 0x50) & 0x20) == 0) {
      if (local_4 != (void *)0x0) {
        iVar3 = *(int *)((int)local_4 + 0x44);
      }
    }
    else {
      iVar3 = *(int *)((int)this + 0x44);
    }
  }
  iVar3 = FUN_004f7390(this,iVar3,param_1);
  if ((iVar3 != 0) && (bVar2)) {
    return 1;
  }
  return 0;
}

