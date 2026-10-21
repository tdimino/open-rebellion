
void __thiscall FUN_004ef0c0(void *this,undefined4 param_1)

{
  bool bVar1;
  
  if (((((*(uint *)((int)this + 0xac) & 1) == 0) || ((*(byte *)((int)this + 0x50) & 1) == 0)) ||
      ((*(byte *)((int)this + 0x78) & 0x80) != 0)) || ((*(uint *)((int)this + 0xac) & 0x200) == 0))
  {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  if (bVar1) {
    (**(code **)(*(int *)this + 0x214))(param_1);
  }
  return;
}

