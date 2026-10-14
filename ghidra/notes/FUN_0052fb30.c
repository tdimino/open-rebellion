
int __thiscall FUN_0052fb30(void *this,uint *param_1,void *param_2)

{
  int iVar1;
  uint uVar2;
  
  if (DAT_006b90e0 == 0) {
    *param_1 = 1;
  }
  else {
    uVar2 = (uint)(0 < *(int *)((int)this + 0x78));
    *param_1 = uVar2;
    if (uVar2 != 0) {
      iVar1 = FUN_0052ef90(this,*(int *)((int)this + 0x78) + -1,param_2);
      return iVar1;
    }
  }
  return 1;
}

