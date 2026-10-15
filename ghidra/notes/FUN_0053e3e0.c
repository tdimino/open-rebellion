
int __cdecl FUN_0053e3e0(int param_1,int param_2,undefined4 *param_3)

{
  void *this;
  undefined4 uVar1;
  bool bVar2;
  int iVar3;
  
  *param_3 = 0;
  this = (void *)FUN_0055bed0(0x3d,param_1);
  if ((this == (void *)0x0) || (*(int *)((int)this + 0x20) != param_1)) {
    bVar2 = false;
  }
  else {
    bVar2 = true;
  }
  iVar3 = 0;
  if (bVar2) {
    if ((param_2 < 1) || (2 < param_2)) {
      iVar3 = 0;
    }
    else {
      iVar3 = 1;
    }
  }
  if (iVar3 != 0) {
    uVar1 = FUN_00585ad0(this,DAT_006b904c,param_2);
    *param_3 = uVar1;
  }
  return iVar3;
}

