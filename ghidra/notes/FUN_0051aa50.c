
undefined4 __thiscall FUN_0051aa50(void *this,void *param_1)

{
  bool bVar1;
  bool bVar2;
  int iVar3;
  int iVar4;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  
  iVar3 = FUN_0051ab20(param_1);
  iVar4 = FUN_0051acb0(param_1);
  if ((iVar4 == 0) || (iVar3 == 0)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  iVar3 = FUN_0051ad90(this,param_1);
  if ((iVar3 == 0) || (!bVar1)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  iVar3 = FUN_0051ae20(param_1);
  if ((iVar3 == 0) || (!bVar1)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  iVar3 = FUN_0051af00(this,param_1);
  if ((iVar3 == 0) || (!bVar1)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  iVar3 = FUN_0051b080(param_1);
  if ((iVar3 == 0) || (!bVar1)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  bVar2 = FUN_0051b1b0(this,param_1);
  if ((CONCAT31(extraout_var,bVar2) == 0) || (!bVar1)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  bVar2 = FUN_0051b1d0(this,param_1);
  if ((CONCAT31(extraout_var_00,bVar2) != 0) && (bVar1)) {
    return 1;
  }
  return 0;
}

