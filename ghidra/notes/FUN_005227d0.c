
undefined4 __thiscall FUN_005227d0(void *this,void *param_1)

{
  bool bVar1;
  undefined3 extraout_var;
  int iVar2;
  int iVar3;
  undefined4 uVar4;
  int iVar5;
  
  uVar4 = 1;
  bVar1 = FUN_00520e40((int)this);
  if (CONCAT31(extraout_var,bVar1) != 0) {
    FUN_00521980(this,0xb,param_1);
    return 1;
  }
  if ((*(byte *)((int)this + 0xa4) & 1) != 0) {
    iVar2 = FUN_00522130(this,0,param_1);
    iVar3 = *(int *)((int)this + 0x68);
    iVar5 = iVar3 + 1;
    if (iVar3 == 10) {
      iVar3 = FUN_00520b60((int)this);
      if (iVar3 != 0) {
        iVar5 = 8;
      }
    }
    else if (iVar3 == 0xb) {
      iVar5 = 0xb;
    }
    iVar5 = FUN_00521980(this,iVar5,param_1);
    if ((iVar5 != 0) && (iVar2 != 0)) {
      return 1;
    }
    uVar4 = 0;
  }
  return uVar4;
}

