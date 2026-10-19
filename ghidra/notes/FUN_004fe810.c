
undefined4 __thiscall FUN_004fe810(void *this,void *param_1)

{
  bool bVar1;
  bool bVar2;
  int iVar3;
  int iVar4;
  undefined3 extraout_var;
  
  iVar3 = FUN_0051ebb0();
  iVar4 = (**(code **)(*(int *)this + 0x84))(param_1);
  if ((iVar4 == 0) || (iVar3 == 0)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  bVar2 = FUN_004fe630(this,param_1);
  if ((CONCAT31(extraout_var,bVar2) != 0) && (bVar1)) {
    return 1;
  }
  return 0;
}

