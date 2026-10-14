
undefined4 __thiscall FUN_005099c0(void *this,int param_1)

{
  bool bVar1;
  undefined4 uVar2;
  
  uVar2 = 0;
  if ((param_1 < 1) || (3 < param_1)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  if (bVar1) {
    uVar2 = *(undefined4 *)(*(int *)((int)this + 0x54) + 0x44 + param_1 * 4);
  }
  return uVar2;
}

