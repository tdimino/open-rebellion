
int __thiscall FUN_004738d0(void *this,int param_1)

{
  int iVar1;
  
  iVar1 = 0;
  if (0 < *(int *)((int)this + 0x50)) {
    iVar1 = (*(int *)((int)this + 0x30) * param_1) / *(int *)((int)this + 0x50);
  }
  return iVar1;
}

