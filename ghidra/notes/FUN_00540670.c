
void __thiscall FUN_00540670(void *this,uint param_1)

{
  *(uint *)((int)this + 0x24) = (param_1 & 3) << 6 | *(uint *)((int)this + 0x24) & 0xffffff3f;
  return;
}

