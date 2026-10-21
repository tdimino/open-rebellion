
void __thiscall FUN_006027e0(void *this,int param_1,int param_2)

{
  *(int *)((int)this + 0xd0) = *(int *)((int)this + 0xd0) + param_2;
  *(int *)((int)this + 0xd8) = *(int *)((int)this + 0xd8) + param_2;
  *(int *)((int)this + 0xcc) = *(int *)((int)this + 0xcc) + param_1;
  *(int *)((int)this + 0xd4) = *(int *)((int)this + 0xd4) + param_1;
  return;
}

