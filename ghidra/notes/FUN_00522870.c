
bool __thiscall FUN_00522870(void *this,void *param_1)

{
  int iVar1;
  int iVar2;
  bool bVar3;
  uint local_14;
  int local_10;
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006439e0;
  local_c = ExceptionList;
  bVar3 = true;
  if ((*(int *)((int)this + 0x68) != 1) && (*(int *)((int)this + 0x68) != 0xb)) {
    ExceptionList = &local_c;
    FUN_004ece30(&local_14);
    local_4 = 0;
    FUN_0042dbe0(&local_10);
    local_4 = CONCAT31(local_4._1_3_,1);
    iVar1 = FUN_00521300((int)this + 0x84,&local_14,&local_10);
    iVar2 = FUN_00521cc0(this,(int *)&local_14,param_1);
    if ((iVar2 == 0) || (iVar1 == 0)) {
      bVar3 = false;
    }
    else {
      bVar3 = true;
    }
    iVar1 = FUN_00521d70(this,&local_10,param_1);
    if ((iVar1 == 0) || (!bVar3)) {
      bVar3 = false;
    }
    else {
      bVar3 = true;
    }
    if ((bVar3) && ((*(byte *)((int)this + 0xa4) & 2) == 0)) {
      iVar1 = FUN_004ece60(&local_14);
      bVar3 = iVar1 != 0;
    }
    local_4 = local_4 & 0xffffff00;
    FUN_00619730();
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  ExceptionList = local_c;
  return bVar3;
}

