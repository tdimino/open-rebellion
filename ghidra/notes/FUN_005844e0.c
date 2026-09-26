
undefined4 __thiscall FUN_005844e0(void *this,int param_1)

{
  bool bVar1;
  undefined4 uVar2;
  
  if ((param_1 < 0) || (2 < param_1)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  uVar2 = 0;
  if (bVar1) {
    uVar2 = *(undefined4 *)((int)this + param_1 * 4 + 0x24);
  }
  return uVar2;
}

