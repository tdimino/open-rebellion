
bool __thiscall FUN_00510e30(void *this,int param_1,void *param_2)

{
  bool bVar1;
  uint uVar2;
  int iVar3;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  bool bVar4;
  int iVar5;
  
  bVar4 = true;
  if (DAT_006b90e0 != 0) {
    if (param_1 != 0) {
      uVar2 = *(uint *)((int)this + 0x24) >> 6 & 3;
      iVar3 = DAT_00661a88;
      if (uVar2 == 1) {
        iVar5 = 1;
      }
      else if (uVar2 == 2) {
        iVar5 = 2;
      }
      else {
        iVar5 = 1;
        iVar3 = (DAT_00661a88 + DAT_00661a84) / 2;
      }
      iVar3 = FUN_00509c00(this,iVar3,iVar5,param_2);
      bVar4 = iVar3 != 0;
    }
    bVar1 = FUN_0050b610(this,param_2);
    if ((CONCAT31(extraout_var,bVar1) == 0) || (!bVar4)) {
      bVar4 = false;
    }
    else {
      bVar4 = true;
    }
    iVar3 = FUN_0050b5a0(this,param_2);
    if ((iVar3 == 0) || (!bVar4)) {
      bVar4 = false;
    }
    else {
      bVar4 = true;
    }
    iVar3 = FUN_0050ac10(this,1,param_2);
    if ((iVar3 == 0) || (!bVar4)) {
      bVar4 = false;
    }
    else {
      bVar4 = true;
    }
    bVar1 = FUN_0050b890(this,param_2);
    if ((CONCAT31(extraout_var_00,bVar1) != 0) && (bVar4)) {
      return true;
    }
    bVar4 = false;
  }
  return bVar4;
}

