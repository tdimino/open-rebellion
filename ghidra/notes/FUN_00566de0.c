
uint __thiscall FUN_00566de0(void *this,void *param_1)

{
  int iVar1;
  bool bVar2;
  int iVar3;
  undefined3 extraout_var;
  int *piVar4;
  int iVar5;
  uint uVar6;
  uint local_18;
  int local_14;
  int local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064b118;
  local_c = ExceptionList;
  uVar6 = 1;
  local_14 = *(int *)((int)this + 0x5c);
  ExceptionList = &local_c;
  FUN_0042dbe0(&local_18);
  iVar1 = *(int *)((int)this + 100);
  iVar5 = 0;
  local_4 = 0;
  do {
    if (local_14 <= iVar5) break;
    iVar3 = FUN_0052d370(this,*(uint *)((int)this + 0x24) >> 6 & 3,1);
    bVar2 = FUN_00559850(iVar1,iVar3,&local_10,(int *)&local_18);
    uVar6 = CONCAT31(extraout_var,bVar2);
    if ((uVar6 != 0) && (local_10 != 0)) {
      piVar4 = FUN_004f7d50(this,&local_18,*(uint *)((int)this + 0x24) >> 6 & 3,param_1,1,1);
      uVar6 = (uint)(piVar4 != (int *)0x0);
    }
    iVar5 = iVar5 + 1;
  } while (uVar6 != 0);
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return uVar6;
}

