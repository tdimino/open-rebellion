
bool FUN_00561280(uint *param_1,int param_2,void *param_3)

{
  bool bVar1;
  void *this;
  uint uVar2;
  int iVar3;
  int iVar4;
  undefined3 extraout_var;
  bool bVar5;
  
  this = (void *)FUN_00505f60(param_1);
  bVar5 = this != (void *)0x0;
  bVar1 = bVar5;
  if (((this != (void *)0x0) && (param_2 == 0)) && (uVar2 = FUN_00509ac0((int)this), uVar2 != 3)) {
    if (((*(byte *)((int)this + 0x50) & 8) == 0) &&
       (uVar2 != (*(uint *)((int)this + 0x24) >> 6 & 3))) {
      uVar2 = 2 - (uVar2 != 1);
    }
    iVar3 = FUN_00509a20(this,1);
    iVar4 = FUN_00509a20(this,2);
    iVar3 = FUN_0053ec10(uVar2,iVar3,iVar4);
    bVar1 = FUN_00561910(iVar3,uVar2,param_3);
    if ((CONCAT31(extraout_var,bVar1) == 0) || (bVar1 = true, !bVar5)) {
      bVar1 = false;
    }
  }
  return bVar1;
}

