
void __thiscall FUN_00530670(void *this,void *param_1,void *param_2)

{
  int iVar1;
  
  iVar1 = 0;
  if ((*(uint *)((int)param_1 + 0x60) >> 2 & 1) != 0) {
    iVar1 = FUN_0053a860(param_1,0,param_2);
  }
  if (iVar1 != 0) {
    FUN_0052ef90(this,*(int *)((int)this + 0x78) + 1,param_2);
  }
  return;
}

