
int __thiscall FUN_00473910(void *this,int param_1)

{
  int iVar1;
  
  iVar1 = 0;
  if (0 < *(int *)((int)this + 0x54)) {
    iVar1 = (*(int *)((int)this + 0x3c) * param_1) / *(int *)((int)this + 0x54);
  }
  return iVar1;
}

