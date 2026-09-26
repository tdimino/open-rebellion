
bool __thiscall FUN_00519800(void *this,void *param_1)

{
  bool bVar1;
  int iVar2;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  bool bVar3;
  uint local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00642c98;
  local_c = ExceptionList;
  local_10 = 0xf1000001;
  local_4 = 0;
  ExceptionList = &local_c;
  iVar2 = FUN_0051cab0(&local_10);
  local_4 = 0xffffffff;
  FUN_00619730();
  bVar3 = iVar2 != 0;
  if (iVar2 != 0) {
    iVar2 = (**(code **)(*(int *)this + 0x28))(iVar2);
    if ((iVar2 == 0) || (!bVar3)) {
      bVar3 = false;
    }
    else {
      bVar3 = true;
    }
    iVar2 = FUN_005197c0((int)this);
    if ((iVar2 == 0) || (!bVar3)) {
      bVar3 = false;
    }
    else {
      bVar3 = true;
    }
    bVar1 = FUN_00519740((int)this);
    if ((CONCAT31(extraout_var,bVar1) == 0) || (!bVar3)) {
      bVar3 = false;
    }
    else {
      bVar3 = true;
    }
    bVar1 = FUN_004f9190(this,(int *)0x3,param_1);
    if ((CONCAT31(extraout_var_00,bVar1) == 0) || (!bVar3)) {
      bVar3 = false;
    }
    else {
      bVar3 = true;
    }
    iVar2 = FUN_004f7480(this,1,param_1);
    if ((iVar2 == 0) || (!bVar3)) {
      bVar3 = false;
    }
    else {
      bVar3 = true;
    }
    iVar2 = FUN_004f74f0(this,1,param_1);
    if ((iVar2 == 0) || (!bVar3)) {
      bVar3 = false;
    }
    else {
      bVar3 = true;
    }
  }
  ExceptionList = local_c;
  return bVar3;
}

