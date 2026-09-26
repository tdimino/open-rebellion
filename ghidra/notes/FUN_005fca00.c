
bool __thiscall FUN_005fca00(void *this,int param_1,uint param_2)

{
  bool bVar1;
  int iVar2;
  int iVar3;
  uint uVar4;
  undefined1 local_4;
  
  local_4 = false;
  iVar2 = FUN_005fc0f0(this);
  iVar3 = FUN_005fc110(this);
  if (iVar3 != 0) {
    bVar1 = FUN_005fcaa0(this,param_1,param_2);
    return bVar1;
  }
  if (iVar2 < 1) {
    iVar2 = -iVar2;
  }
  if (((int)param_2 < iVar2) && (0 < (int)param_2)) {
    iVar2 = FUN_005fc0e0(this);
    if ((param_1 < iVar2) && (0 < param_1)) {
      iVar2 = FUN_005fc9e0(this,param_2);
      uVar4 = FUN_005fc9c0(this);
      if (*(char *)(uVar4 * iVar2 + *(int *)((int)this + 4) + param_1) != *(char *)((int)this + 8))
      {
        local_4 = true;
      }
    }
  }
  return local_4;
}

