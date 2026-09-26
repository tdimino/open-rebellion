
int __thiscall FUN_004eeed0(void *this,void *param_1)

{
  bool bVar1;
  uint uVar2;
  int iVar3;
  
  iVar3 = 1;
  if (*(short *)((int)this + 0x96) == 0) {
    return 1;
  }
  if (((*(short *)((int)this + 0x94) == 0) && ((*(uint *)((int)this + 0xac) & 1) == 0)) &&
     ((*(uint *)((int)this + 0x50) >> 6 & 1) != 0)) {
    uVar2 = *(uint *)((int)this + 0x68) >> 0x18;
    if ((uVar2 < 0x50) || (0x7f < uVar2)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (!bVar1) {
      bVar1 = false;
      goto LAB_004eef48;
    }
  }
  bVar1 = true;
LAB_004eef48:
  if (bVar1) {
    iVar3 = FUN_004ee350(this,0,param_1);
  }
  return iVar3;
}

