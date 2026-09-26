
void __thiscall FUN_0050ce50(void *this,undefined4 param_1,int param_2)

{
  bool bVar1;
  
  if ((param_2 < 1) || (3 < param_2)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  if (bVar1) {
    *(undefined4 *)(*(int *)((int)this + 0x54) + 0x50 + param_2 * 4) = param_1;
  }
  return;
}

