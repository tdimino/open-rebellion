
undefined4 __thiscall FUN_00549910(void *this,void *param_1)

{
  bool bVar1;
  bool bVar2;
  undefined3 extraout_var;
  int iVar3;
  undefined3 extraout_var_00;
  undefined3 extraout_var_01;
  undefined3 extraout_var_02;
  uint uVar4;
  
  bVar1 = FUN_005442f0(this,param_1);
  iVar3 = FUN_00544da0(this,param_1);
  if ((iVar3 == 0) || (CONCAT31(extraout_var,bVar1) == 0)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  bVar2 = FUN_00544030(this,param_1);
  if ((CONCAT31(extraout_var_00,bVar2) == 0) || (!bVar1)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  bVar2 = FUN_00544130(this,param_1);
  if ((CONCAT31(extraout_var_01,bVar2) == 0) || (!bVar1)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  bVar2 = FUN_005443f0(this,param_1);
  if ((CONCAT31(extraout_var_02,bVar2) == 0) || (!bVar1)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  uVar4 = FUN_005444e0(this,param_1);
  if ((uVar4 == 0) || (!bVar1)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  iVar3 = FUN_005445d0(this,param_1);
  if ((iVar3 == 0) || (!bVar1)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  iVar3 = FUN_00544a20(this,param_1);
  if ((iVar3 == 0) || (!bVar1)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  iVar3 = FUN_00544be0(this,param_1);
  if ((iVar3 == 0) || (!bVar1)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  iVar3 = FUN_00544fc0(this,param_1);
  if ((iVar3 != 0) && (bVar1)) {
    return 1;
  }
  return 0;
}

