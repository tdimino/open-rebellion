
void __thiscall FUN_006082c0(void *this,undefined4 param_1,undefined4 param_2)

{
  *(undefined4 *)((int)this + 0xcc) = param_1;
  *(undefined4 *)((int)this + 0xd0) = param_2;
  if (*(int *)((int)this + 0xf0) != 0) {
    *(undefined4 *)((int)this + 0xbc) = param_1;
    return;
  }
  *(undefined4 *)((int)this + 0xb8) = param_2;
  return;
}

