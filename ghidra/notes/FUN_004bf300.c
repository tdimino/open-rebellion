
void __thiscall FUN_004bf300(void *this,undefined4 param_1,undefined4 param_2)

{
  if (*(int **)((int)this + 0x60) != (int *)0x0) {
    (**(code **)(**(int **)((int)this + 0x60) + 0x10))(param_1,param_2);
  }
  return;
}

