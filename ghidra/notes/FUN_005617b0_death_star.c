
bool __thiscall FUN_005617b0(void *this,void *param_1)

{
  bool bVar1;
  void *this_00;
  undefined3 extraout_var;
  uint *puVar2;
  int iVar3;
  bool bVar4;
  void *local_18;
  uint local_14;
  uint local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064a2a8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  this_00 = (void *)FUN_005070d0();
  local_18 = (void *)0x0;
  if (this_00 == (void *)0x0) {
    ExceptionList = local_c;
    return this_00 != (void *)0x0;
  }
  bVar1 = FUN_004f6b50(this_00,(int *)&local_18);
  if ((CONCAT31(extraout_var,bVar1) == 0) || (this_00 == (void *)0x0)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  if (this_00 == (void *)0x0) {
    ExceptionList = local_c;
    return bVar1;
  }
  if (local_18 == (void *)0x0) {
    ExceptionList = local_c;
    return bVar1;
  }
  if (((*(byte *)((int)this_00 + 0xac) & 1) == 0) && ((*(byte *)((int)this_00 + 0x50) & 1) != 0)) {
    local_14 = 0x90000109;
    local_4 = 0;
    puVar2 = FUN_004025b0(local_18,&local_10);
    bVar4 = *puVar2 == local_14;
    FUN_00619730();
    local_4 = 0xffffffff;
    FUN_00619730();
    if ((bVar4) && (((byte)*(undefined4 *)((int)local_18 + 0x24) & 0xc0) == 0x80)) {
      iVar3 = 1;
      goto LAB_0056188f;
    }
  }
  iVar3 = 0;
LAB_0056188f:
  iVar3 = FUN_0055f650(this,iVar3,param_1);
  if ((iVar3 == 0) || (!bVar1)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  ExceptionList = local_c;
  return bVar1;
}

