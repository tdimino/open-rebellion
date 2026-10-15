
bool __thiscall FUN_00501b70(void *this,int param_1,void *param_2)

{
  int iVar1;
  bool bVar2;
  
  bVar2 = false;
  if (-1 < param_1) {
    iVar1 = *(int *)((int)this + 0x60) - param_1;
    if (iVar1 < 0) {
      iVar1 = 0;
    }
    iVar1 = FUN_00501490(this,iVar1,param_2);
    bVar2 = iVar1 != 0;
  }
  return bVar2;
}

