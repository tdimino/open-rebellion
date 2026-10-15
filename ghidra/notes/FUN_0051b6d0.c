
bool FUN_0051b6d0(void *param_1)

{
  int iVar1;
  bool bVar2;
  
  iVar1 = thunk_FUN_00506e80();
  bVar2 = iVar1 != 0;
  if (iVar1 != 0) {
    iVar1 = FUN_0055f730(param_1);
    if ((iVar1 != 0) && (bVar2)) {
      return true;
    }
    bVar2 = false;
  }
  return bVar2;
}

