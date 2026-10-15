
int __thiscall FUN_0052fb80(void *this,uint *param_1,void *param_2)

{
  int iVar1;
  uint uVar2;
  
  iVar1 = 1;
  uVar2 = (uint)(0 < *(int *)((int)this + 0x7c));
  *param_1 = uVar2;
  if (uVar2 != 0) {
    iVar1 = FUN_0052eff0(this,*(int *)((int)this + 0x7c) + -1,param_2);
  }
  return iVar1;
}

