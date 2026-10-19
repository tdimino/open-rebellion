
void __thiscall FUN_00418990(void *this,undefined4 param_1)

{
  *(undefined4 *)((int)this + 0x188) = param_1;
  *(uint *)((int)this + 4) = *(uint *)((int)this + 4) | 0x80000000;
  return;
}

