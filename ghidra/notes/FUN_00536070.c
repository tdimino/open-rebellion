
bool __thiscall FUN_00536070(void *this,void *param_1,int *param_2,void *param_3)

{
  int iVar1;
  uint *puVar2;
  bool bVar3;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00645ae8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  iVar1 = FUN_004f2ab0(this,param_1,param_2,param_3);
  bVar3 = iVar1 != 0;
  if ((((*(uint *)((int)this + 0x50) & 0x40) != 0) && ((*(uint *)((int)this + 0x50) & 4) != 0)) &&
     ((*(byte *)((int)this + 0x78) & 0x40) == 0)) {
    puVar2 = FUN_0042d170(this,&param_3);
    local_4 = 0;
    iVar1 = FUN_004ece60(puVar2);
    if ((iVar1 == 0) || (!bVar3)) {
      bVar3 = false;
    }
    else {
      bVar3 = true;
    }
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  ExceptionList = local_c;
  return bVar3;
}

