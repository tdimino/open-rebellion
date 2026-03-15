
void __thiscall FUN_0045fd20(void *this,int *param_1)

{
  int iVar1;
  
  if (param_1 == (int *)0x0) {
    (**(code **)(**(int **)((int)this + 0x144) + 0x10))();
    (**(code **)(**(int **)((int)this + 0x140) + 0x10))();
    return;
  }
  iVar1 = (**(code **)(*param_1 + 0x10))();
  if (iVar1 == 0) {
    (**(code **)(**(int **)((int)this + 0x144) + 0x10))();
  }
  else {
    (**(code **)(**(int **)((int)this + 0x144) + 0xc))();
  }
  iVar1 = (**(code **)(*param_1 + 0xc))();
  if (iVar1 == 0) {
    (**(code **)(**(int **)((int)this + 0x140) + 0x10))();
    return;
  }
  (**(code **)(**(int **)((int)this + 0x140) + 0xc))();
  return;
}

