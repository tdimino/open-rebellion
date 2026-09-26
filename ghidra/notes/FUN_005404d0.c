
undefined4 __thiscall FUN_005404d0(void *this,uint param_1)

{
  uint uVar1;
  
  uVar1 = *(uint *)((int)this + 0x24) & 0xf;
  if (((uVar1 != param_1) && (param_1 != 3)) && (uVar1 != 3)) {
    return 0;
  }
  return 1;
}

