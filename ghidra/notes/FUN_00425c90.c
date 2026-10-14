
void __thiscall FUN_00425c90(void *this,uint param_1)

{
  int iVar1;
  
  iVar1 = 0;
  if ((param_1 & 0x100000) != 0) {
    *(uint *)((int)this + 0xa8) = (*(int *)((int)this + 0x9c) == 2) + 0x11;
    return;
  }
  if ((param_1 & 0x1000000) != 0) {
    iVar1 = (-(uint)(*(int *)((int)this + 0x9c) != 1) & 3) + 0x10;
  }
  *(int *)((int)this + 0xa8) = iVar1;
  return;
}

