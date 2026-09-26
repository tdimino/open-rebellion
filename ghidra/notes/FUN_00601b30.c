
void __thiscall FUN_00601b30(void *this,int param_1,int param_2)

{
  int iVar1;
  int iVar2;
  
  if ((*(uint *)((int)this + 0x2c) & 1) == 0) {
    iVar2 = *(int *)((int)this + 0x1c) - *(int *)((int)this + 0x14);
  }
  else {
    iVar2 = *(int *)this;
  }
  if ((*(uint *)((int)this + 0x2c) & 2) == 0) {
    iVar1 = *(int *)((int)this + 0x20) - *(int *)((int)this + 0x18);
  }
  else {
    iVar1 = *(int *)((int)this + 4);
  }
  *(int *)((int)this + 0x18) = param_2;
  *(int *)((int)this + 0x14) = param_1;
  *(int *)((int)this + 0x1c) = iVar2 + param_1;
  *(int *)((int)this + 0x20) = iVar1 + param_2;
  return;
}

