
int __thiscall FUN_00509a20(void *this,int param_1)

{
  bool bVar1;
  int iVar2;
  int iVar3;
  int iVar4;
  
  iVar4 = 0;
  if ((param_1 < 1) || (3 < param_1)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  if (bVar1 && param_1 != 3) {
    iVar2 = FUN_00509ac0((int)this);
    if ((iVar2 < 1) || (3 < iVar2)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    if ((bVar1) && (iVar2 != 3)) {
      iVar4 = FUN_005099c0(this,param_1);
      iVar3 = FUN_005099f0(this,param_1);
      iVar4 = iVar4 + iVar3;
      if (iVar2 != param_1) {
        iVar2 = FUN_005099c0(this,3);
        iVar3 = FUN_005099f0(this,3);
        iVar4 = iVar3 + iVar4 + iVar2;
      }
    }
  }
  return iVar4;
}

