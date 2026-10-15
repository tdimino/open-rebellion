
bool FUN_0051b710(void *param_1)

{
  void *this;
  int iVar1;
  bool bVar2;
  
  this = (void *)thunk_FUN_00506e80();
  bVar2 = this != (void *)0x0;
  if (this != (void *)0x0) {
    iVar1 = FUN_0055f7b0(this,param_1);
    if ((iVar1 != 0) && (bVar2)) {
      return true;
    }
    bVar2 = false;
  }
  return bVar2;
}

