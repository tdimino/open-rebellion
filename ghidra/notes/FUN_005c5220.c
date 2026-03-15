
void __thiscall FUN_005c5220(void *this,int param_1)

{
  int iVar1;
  
  if (*(void **)((int)this + 0xa0) != (void *)0x0) {
    FUN_0060e440(*(void **)((int)this + 0xa0),param_1);
    if (param_1 == 100) {
      FUN_006030f0(*(void **)((int)this + 0x9c),5);
      FUN_006030c0(*(void **)((int)this + 0x9c),0);
      (**(code **)(**(int **)((int)this + 0x9c) + 0xc))();
      return;
    }
    iVar1 = FUN_005ffea0(*(int *)((int)this + 0x9c));
    if ((iVar1 != 0) && (param_1 < 100)) {
      (**(code **)(**(int **)((int)this + 0x9c) + 0x10))();
    }
  }
  return;
}

