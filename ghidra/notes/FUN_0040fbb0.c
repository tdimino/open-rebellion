
void __thiscall FUN_0040fbb0(void *this,int param_1)

{
  bool bVar1;
  
  if (((((*(int *)((int)this + 0x13c) == 0) || (*(int *)((int)this + 0x140) == 0)) ||
       (*(int *)((int)this + 0x144) == 0)) ||
      (((*(int *)((int)this + 0x148) == 0 || (*(int *)((int)this + 0x14c) == 0)) ||
       ((*(int *)((int)this + 0x150) == 0 ||
        ((*(int *)((int)this + 0x154) == 0 || (*(int *)((int)this + 0x158) == 0)))))))) ||
     ((*(int *)((int)this + 0x15c) == 0 ||
      ((((*(int *)((int)this + 0x160) == 0 || (*(int *)((int)this + 0x164) == 0)) ||
        (*(int *)((int)this + 0x168) == 0)) || (*(int *)((int)this + 0x16c) == 0)))))) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  if (bVar1) {
    if (param_1 != 0) {
      (**(code **)(**(int **)((int)this + 0x150) + 0xc))();
      (**(code **)(**(int **)((int)this + 0x154) + 0xc))();
      (**(code **)(**(int **)((int)this + 0x158) + 0xc))();
      (**(code **)(**(int **)((int)this + 0x15c) + 0xc))();
      (**(code **)(**(int **)((int)this + 0x160) + 0xc))();
      (**(code **)(**(int **)((int)this + 0x164) + 0x10))();
      (**(code **)(**(int **)((int)this + 0x168) + 0xc))();
      (**(code **)(**(int **)((int)this + 0x16c) + 0xc))();
      (**(code **)(**(int **)((int)this + 0x144) + 0x10))();
      return;
    }
    (**(code **)(**(int **)((int)this + 0x150) + 0x10))();
    (**(code **)(**(int **)((int)this + 0x154) + 0x10))();
    (**(code **)(**(int **)((int)this + 0x158) + 0x10))();
    (**(code **)(**(int **)((int)this + 0x15c) + 0x10))();
    (**(code **)(**(int **)((int)this + 0x160) + 0x10))();
    (**(code **)(**(int **)((int)this + 0x164) + 0x10))();
    (**(code **)(**(int **)((int)this + 0x168) + 0x10))();
    (**(code **)(**(int **)((int)this + 0x16c) + 0x10))();
    (**(code **)(**(int **)((int)this + 0x144) + 0x10))();
  }
  return;
}

