
int __thiscall FUN_005a6620(void *this,uint param_1)

{
  int iVar1;
  
  if (param_1 < 2) {
    iVar1 = *(int *)((int)this + param_1 * 4 + 4) + 1;
    *(int *)((int)this + param_1 * 4 + 4) = iVar1;
    return iVar1;
  }
  return 0;
}

