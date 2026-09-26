
void __thiscall FUN_0048a610(void *this,int param_1)

{
  if (param_1 != 0) {
    *(uint *)((int)this + 8) = *(uint *)((int)this + 8) | 0x10000000;
    return;
  }
  *(uint *)((int)this + 8) = *(uint *)((int)this + 8) & 0xefffffff;
  FUN_0048a2a0((int)this);
  return;
}

