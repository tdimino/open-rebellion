
void __thiscall FUN_0059cba0(void *this,int param_1)

{
  if (*(int *)((int)this + 0x108) != 0) {
    *(int *)(*(int *)((int)this + 0x108) + 0x648) = param_1;
  }
  if (param_1 == 1) {
    FUN_00595b10(0xf);
    return;
  }
  if (*(void **)((int)this + 0x110) != (void *)0x0) {
    FUN_005c51c0(*(void **)((int)this + 0x110),0);
  }
  return;
}

