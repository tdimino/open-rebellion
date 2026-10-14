
int __thiscall FUN_006075e0(void *this,int param_1)

{
  int iVar1;
  
  iVar1 = FUN_006076b0(this,param_1);
  if (iVar1 != 0) {
    *(int *)((int)this + 0xa4) = iVar1;
    FUN_006076e0((int)this);
  }
  return iVar1;
}

