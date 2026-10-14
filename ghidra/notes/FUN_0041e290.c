
undefined4 __thiscall FUN_0041e290(void *this,uint param_1)

{
  if (*(uint *)((int)this + 0x10) < param_1) {
    *(uint *)((int)this + 0x10) = param_1;
    FUN_00401930();
  }
  return *(undefined4 *)((int)this + 0x10);
}

