
void __thiscall FUN_00533610(void *this,int param_1,void *param_2)

{
  int iVar1;
  
  if ((param_1 != 0) && (*(void **)((int)this + 0x8c) != (void *)0x0)) {
    iVar1 = FUN_00583c50(*(void **)((int)this + 0x8c),param_1);
    if (iVar1 != 0) {
      FUN_0052f0c0(this,*(int *)((int)this + 0x84) + 1,param_2);
    }
  }
  return;
}

