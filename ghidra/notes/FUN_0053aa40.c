
int __thiscall FUN_0053aa40(void *this,void *param_1)

{
  int iVar1;
  int iVar2;
  
  iVar1 = *(int *)((int)this + 0x58);
  if ((iVar1 < 1) || (4 < iVar1)) {
    iVar2 = 0;
  }
  else {
    iVar2 = 1;
  }
  if (iVar2 != 0) {
    if (((byte)*(undefined4 *)((int)this + 0x24) & 0xc0) == 0xc0) {
      iVar1 = 4;
    }
    else if (iVar1 == 1) {
      if ((*(byte *)((int)this + 0x60) & 2) != 0) {
        iVar1 = 2;
      }
    }
    else if (iVar1 == 2) {
      if ((*(byte *)((int)this + 0x60) & 4) != 0) {
        iVar1 = 3;
      }
    }
    else if ((iVar1 == 3) && ((*(byte *)((int)this + 0x60) & 2) == 0)) {
      iVar1 = 1;
    }
    if (iVar2 != 0) {
      iVar2 = FUN_0053a6f0(this,iVar1,param_1);
    }
  }
  return iVar2;
}

