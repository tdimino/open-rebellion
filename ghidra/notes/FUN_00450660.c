
void __thiscall FUN_00450660(void *this,int param_1)

{
  int iVar1;
  
  if (param_1 == *(int *)((int)this + 0x13c)) {
    return;
  }
  FUN_005f5b20(*(void **)((int)this + 0x130));
  switch(param_1) {
  case 1:
    (**(code **)(**(int **)((int)this + 300) + 8))();
    FUN_006075e0(this,0x32);
    if (*(int **)((int)this + 0x11c) != (int *)0x0) {
      (**(code **)(**(int **)((int)this + 0x11c) + 4))(5);
      *(int *)((int)this + 0x13c) = param_1;
      return;
    }
    break;
  case 2:
    FUN_006075e0(this,0x33);
    iVar1 = 1;
    goto LAB_004506de;
  case 3:
    FUN_006075e0(this,0x33);
    iVar1 = 2;
LAB_004506de:
    FUN_00450770(this,iVar1);
    FUN_0060a280(*(void **)((int)this + 300));
    (**(code **)(**(int **)((int)this + 300) + 4))(5);
    if (*(int **)((int)this + 0x11c) != (int *)0x0) {
      (**(code **)(**(int **)((int)this + 0x11c) + 8))();
      *(int *)((int)this + 0x13c) = param_1;
      return;
    }
    break;
  case 4:
    FUN_006075e0(this,0x33);
    FUN_00450770(this,0);
    FUN_0060a280(*(void **)((int)this + 300));
    (**(code **)(**(int **)((int)this + 300) + 4))(5);
    if (*(int **)((int)this + 0x11c) != (int *)0x0) {
      (**(code **)(**(int **)((int)this + 0x11c) + 8))();
    }
  }
  *(int *)((int)this + 0x13c) = param_1;
  return;
}

