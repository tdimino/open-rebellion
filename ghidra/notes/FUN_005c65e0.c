
void __thiscall FUN_005c65e0(void *this,undefined4 param_1)

{
  if (*(int *)((int)this + 0x34) == 0) {
    if (*(int **)((int)this + 0x60) != (int *)0x0) {
      (**(code **)(**(int **)((int)this + 0x60) + 4))(param_1);
    }
    FUN_005cb6e0((int)this);
  }
  return;
}

