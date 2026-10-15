
undefined4 __thiscall FUN_005081f0(void *this,void *param_1)

{
  bool bVar1;
  int iVar2;
  int iVar3;
  
  iVar2 = FUN_0051ebb0();
  iVar3 = FUN_0050a430(this,1,param_1);
  if ((iVar3 == 0) || (iVar2 == 0)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  iVar2 = FUN_0050a510(this,1,param_1);
  if ((iVar2 != 0) && (bVar1)) {
    return 1;
  }
  return 0;
}

