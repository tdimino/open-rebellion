
undefined4 __thiscall FUN_0060b660(void *this,int param_1)

{
  int iVar1;
  int iVar2;
  int iVar3;
  
  iVar2 = *(int *)((int)this + 0xe0) + param_1;
  iVar3 = *(int *)((int)this + 0xe4) + param_1;
  *(int *)((int)this + 0xe0) = iVar2;
  *(int *)((int)this + 0xe4) = iVar3;
  if (iVar2 < 0) {
    *(undefined4 *)((int)this + 0xe0) = 0;
    *(int *)((int)this + 0xe4) = iVar3 - iVar2;
    return 1;
  }
  iVar1 = *(int *)((int)this + 0xdc);
  if (iVar1 < iVar3) {
    *(int *)((int)this + 0xe4) = iVar1;
    *(int *)((int)this + 0xe0) = (iVar1 - iVar3) + iVar2;
  }
  return 1;
}

