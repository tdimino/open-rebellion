
uint __thiscall FUN_004ef1e0(void *this,int param_1,int param_2,void *param_3)

{
  int iVar1;
  uint uVar2;
  
  if ((param_2 < 1) || (3 < param_2)) {
    uVar2 = 0;
  }
  else {
    uVar2 = 1;
  }
  if ((uVar2 != 0) && (uVar2 = (uint)(param_2 != 3), uVar2 != 0)) {
    iVar1 = FUN_004ed240((int)this);
    if (iVar1 == 0) {
      iVar1 = FUN_004edc80((int)this);
      if (iVar1 != param_2) {
        param_1 = -param_1;
      }
      iVar1 = FUN_0053e0d0(*(short *)((int)this + 0x66) + param_1);
      uVar2 = FUN_005341a0(this,iVar1,param_3);
    }
  }
  return uVar2;
}

