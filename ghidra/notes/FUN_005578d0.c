
void __thiscall FUN_005578d0(void *this,void *param_1)

{
  if (((*(uint *)((int)this + 0x50) & 0x200) != 0) && ((*(uint *)((int)this + 0x50) & 1) != 0)) {
    FUN_005577f0(this,1,param_1);
    return;
  }
  FUN_005577f0(this,0,param_1);
  return;
}

