
void __thiscall FUN_00418970(void *this,undefined4 *param_1)

{
  *(undefined4 *)((int)this + 0x184) = *param_1;
  *(uint *)((int)this + 4) = *(uint *)((int)this + 4) | 0x80000000;
  return;
}

