
uint __thiscall FUN_004eee60(void *this,void *param_1)

{
  int iVar1;
  int iVar2;
  uint uVar3;
  
  uVar3 = 1;
  iVar2 = (int)*(short *)((int)this + 0x8e);
  if (((iVar2 == 0) && ((*(uint *)((int)this + 0xac) & 0x20) != 0)) &&
     ((*(uint *)((int)this + 0xac) & 0x10) != 0)) {
    iVar2 = *(int *)((int)this + 0x2c);
    uVar3 = (uint)(iVar2 != 0);
    if (uVar3 == 0) {
      return 0;
    }
    iVar1 = *(int *)(iVar2 + 0xa0);
    iVar2 = FUN_0053e290(*(int *)(iVar2 + 0xa4));
    iVar2 = iVar2 + iVar1;
  }
  if (uVar3 != 0) {
    uVar3 = FUN_004ee150(this,iVar2,param_1);
  }
  return uVar3;
}

