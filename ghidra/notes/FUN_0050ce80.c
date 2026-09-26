
void __thiscall FUN_0050ce80(void *this,int param_1)

{
  bool bVar1;
  
  if ((param_1 < 1) || (3 < param_1)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  if ((bVar1) && (param_1 != 3)) {
    *(int *)(*(int *)((int)this + 0x54) + 0x60) = param_1;
  }
  return;
}

