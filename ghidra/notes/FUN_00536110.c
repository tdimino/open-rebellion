
bool __thiscall FUN_00536110(void *this,int param_1,void *param_2)

{
  int iVar1;
  uint *puVar2;
  int iVar3;
  bool bVar4;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  iVar3 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00645b08;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  iVar1 = FUN_004f2b50(this,param_1,param_2);
  bVar4 = iVar1 != 0;
  if (iVar3 != 0) {
    puVar2 = FUN_0042d170(this,&param_1);
    local_4 = 0;
    iVar1 = FUN_004ece60(puVar2);
    local_4 = 0xffffffff;
    FUN_00619730();
    if (iVar1 != 0) {
      iVar1 = FUN_005345d0(this,1,param_2);
      if ((iVar1 == 0) || (!bVar4)) {
        bVar4 = false;
      }
      else {
        bVar4 = true;
      }
    }
    if (iVar3 != 0) {
      iVar3 = FUN_00534790(this,0,param_2);
      if ((iVar3 == 0) || (!bVar4)) {
        bVar4 = false;
      }
      else {
        bVar4 = true;
      }
    }
  }
  ExceptionList = local_c;
  return bVar4;
}

