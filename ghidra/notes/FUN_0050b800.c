
bool __thiscall FUN_0050b800(void *this,void *param_1)

{
  int iVar1;
  uint uVar2;
  uint uVar3;
  bool bVar4;
  
  uVar3 = 0;
  uVar2 = *(uint *)((int)this + 0x24) >> 6 & 3;
  if (((*(uint *)((int)this + 0x88) & 1) != 0) && ((uVar2 == 1 || (uVar2 == 2)))) {
    if (((*(uint *)((int)this + 0x88) & 4) == 0) && (-1 < *(int *)((int)this + 0x7c))) {
      uVar3 = 0;
    }
    else {
      uVar3 = 1;
    }
  }
  iVar1 = FUN_0050a4a0(this,uVar3,param_1);
  bVar4 = iVar1 != 0;
  if ((*(uint *)((int)this + 0x88) >> 2 & 1) == uVar3) {
    if (uVar3 == 0) {
      uVar2 = 0;
    }
    iVar1 = FUN_0050a130(this,uVar2,param_1);
    if ((iVar1 != 0) && (bVar4)) {
      return true;
    }
    bVar4 = false;
  }
  return bVar4;
}

