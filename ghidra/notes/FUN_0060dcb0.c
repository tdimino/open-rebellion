
undefined4 __thiscall FUN_0060dcb0(void *this,undefined4 param_1,undefined4 param_2)

{
  uint uVar1;
  
  uVar1 = FUN_0060dba0((int)this);
  if (uVar1 != 0) {
    return 0;
  }
  *(undefined4 *)((int)this + 0x10) = param_1;
  *(undefined4 *)((int)this + 0x14) = param_2;
  return 1;
}

