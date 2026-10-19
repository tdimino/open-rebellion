
int __thiscall FUN_005a6640(void *this,uint param_1)

{
  int iVar1;
  
  if (param_1 < 2) {
    iVar1 = *(int *)((int)this + param_1 * 4 + 0xc) + 1;
    *(int *)((int)this + param_1 * 4 + 0xc) = iVar1;
    return iVar1;
  }
  return 0;
}

