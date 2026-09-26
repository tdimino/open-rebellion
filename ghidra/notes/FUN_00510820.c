
bool __thiscall FUN_00510820(void *this,int param_1,int param_2,void *param_3)

{
  bool bVar1;
  int iVar2;
  undefined3 extraout_var;
  bool bVar3;
  
  iVar2 = FUN_004fae40(this,param_1,param_2,param_3);
  bVar3 = iVar2 != 0;
  if (DAT_006b90e0 != 0) {
    iVar2 = FUN_0050ac80(this,param_3);
    if ((iVar2 == 0) || (!bVar3)) {
      bVar3 = false;
    }
    else {
      bVar3 = true;
    }
    bVar1 = FUN_0050b890(this,param_3);
    if ((CONCAT31(extraout_var,bVar1) == 0) || (!bVar3)) {
      bVar3 = false;
    }
    else {
      bVar3 = true;
    }
    iVar2 = FUN_0050c350(this,param_3);
    if ((iVar2 == 0) || (!bVar3)) {
      bVar3 = false;
    }
    else {
      bVar3 = true;
    }
    iVar2 = FUN_0050a4a0(this,0,param_3);
    if ((iVar2 == 0) || (!bVar3)) {
      bVar3 = false;
    }
    else {
      bVar3 = true;
    }
    iVar2 = FUN_00509ec0(this,0,param_3);
    if ((iVar2 == 0) || (!bVar3)) {
      bVar3 = false;
    }
    else {
      bVar3 = true;
    }
    iVar2 = FUN_0050b230(this,param_3);
    if ((iVar2 == 0) || (!bVar3)) {
      bVar3 = false;
    }
    else {
      bVar3 = true;
    }
    iVar2 = FUN_0050b5a0(this,param_3);
    if ((iVar2 == 0) || (!bVar3)) {
      bVar3 = false;
    }
    else {
      bVar3 = true;
    }
    iVar2 = FUN_0050c820(this,param_1,param_2,param_3);
    if ((iVar2 == 0) || (!bVar3)) {
      bVar3 = false;
    }
    else {
      bVar3 = true;
    }
    iVar2 = FUN_0050ac10(this,1,param_3);
    if ((iVar2 != 0) && (bVar3)) {
      return true;
    }
    bVar3 = false;
  }
  return bVar3;
}

