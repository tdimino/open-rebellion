
void __thiscall FUN_0050bb00(void *this,void *param_1)

{
  uint uVar1;
  int iVar2;
  
  uVar1 = *(uint *)((int)this + 0x24) >> 6 & 3;
  if ((*(uint *)((int)this + 0x88) & 1) != 0) {
    if (uVar1 == 1) {
      iVar2 = 1;
    }
    else if (uVar1 == 2) {
      iVar2 = 2;
    }
    else {
      iVar2 = 0;
    }
    if (((iVar2 != 0) && ((*(uint *)((int)this + 0x88) & 4) != 0)) &&
       (-1 < *(int *)((int)this + 0x7c))) {
      FUN_0050a7b0(this,1,param_1);
      return;
    }
  }
  FUN_0050a7b0(this,0,param_1);
  return;
}

