
undefined4 __thiscall FUN_0058c120(void *this,void *param_1)

{
  bool bVar1;
  bool bVar2;
  int iVar3;
  int iVar4;
  uint uVar5;
  undefined3 extraout_var;
  
  if ((*(int *)((int)this + 0x10) != 0) && (*(int *)((int)this + 0x14) != 0)) {
    iVar3 = FUN_0058c2d0(this,param_1);
    iVar4 = FUN_0058c3c0(this,(int)param_1);
    if ((iVar4 == 0) || (iVar3 == 0)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    iVar3 = FUN_0058c500(this,param_1);
    if ((iVar3 == 0) || (!bVar1)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    uVar5 = FUN_0058c580(this,param_1);
    if ((uVar5 == 0) || (!bVar1)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    iVar3 = FUN_0058c810(this,param_1);
    if ((iVar3 == 0) || (!bVar1)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    bVar2 = FUN_0058c8b0(this,param_1);
    if ((CONCAT31(extraout_var,bVar2) != 0) && (bVar1)) {
      return 1;
    }
  }
  return 0;
}

