
bool FUN_005109a0(undefined4 param_1,undefined4 param_2)

{
  int iVar1;
  bool bVar2;
  
  iVar1 = FUN_004fb810(param_1,param_2);
  bVar2 = iVar1 != 0;
  if (DAT_006b90e0 != 0) {
    iVar1 = FUN_0050b760(param_2);
    if ((iVar1 != 0) && (bVar2)) {
      return true;
    }
    bVar2 = false;
  }
  return bVar2;
}

