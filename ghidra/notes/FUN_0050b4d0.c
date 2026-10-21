
void __thiscall FUN_0050b4d0(void *this,void *param_1)

{
  if ((*(byte *)((int)this + 0x88) & 4) != 0) {
    FUN_0050a130(this,*(uint *)((int)this + 0x24) >> 6 & 3,param_1);
    return;
  }
  FUN_0050a130(this,0,param_1);
  return;
}

