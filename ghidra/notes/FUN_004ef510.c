
undefined4 __thiscall FUN_004ef510(void *this,undefined4 *param_1,void *param_2)

{
  bool bVar1;
  bool bVar2;
  int iVar3;
  int iVar4;
  undefined3 extraout_var;
  
  iVar3 = FUN_00524fb0();
  iVar4 = FUN_004ee3e0(this,3,param_2);
  if ((iVar4 == 0) || (iVar3 == 0)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  bVar2 = FUN_004ef190(this,param_1);
  if ((CONCAT31(extraout_var,bVar2) != 0) && (bVar1)) {
    return 1;
  }
  return 0;
}

