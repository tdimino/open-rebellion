
void __thiscall FUN_005307e0(void *this,void *param_1,void *param_2)

{
  int iVar1;
  
  iVar1 = 0;
  if ((*(uint *)((int)param_1 + 0x60) >> 2 & 1) != 0) {
    iVar1 = FUN_0053a860(param_1,0,param_2);
  }
  if (iVar1 != 0) {
    FUN_0052eff0(this,*(int *)((int)this + 0x7c) + 1,param_2);
  }
  return;
}

