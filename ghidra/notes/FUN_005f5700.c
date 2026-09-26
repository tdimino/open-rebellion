
int __thiscall FUN_005f5700(void *this,int param_1)

{
  int iVar1;
  
  iVar1 = 0;
  if (1 < param_1) {
    if (*(int *)((int)this + 4) == -1) {
      iVar1 = FUN_005f56b0(this,-1);
      *(int *)((int)this + 4) = iVar1;
    }
    iVar1 = FUN_005f57a0(*(int *)((int)this + 4));
    *(int *)((int)this + 4) = iVar1;
    iVar1 = iVar1 / (int)(0x7ffffffe / (longlong)param_1);
  }
  return iVar1;
}

