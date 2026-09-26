
int __thiscall FUN_00531230(void *this,void *param_1)

{
  bool bVar1;
  undefined3 extraout_var;
  int iVar2;
  
  bVar1 = FUN_0053a010((int)this);
  iVar2 = CONCAT31(extraout_var,bVar1);
  if (iVar2 != 0) {
    FUN_004f9540(this,param_1);
    if (*(int *)((int)param_1 + 4) == 1) {
      FUN_005315e0(this,param_1);
      FUN_00531710(this,param_1);
      FUN_00531840(this,param_1);
      FUN_00531970(this,param_1);
      FUN_005319f0(this,param_1);
      FUN_00531a70(this,param_1);
      FUN_00531af0(this,param_1);
      FUN_00531b70(this,param_1);
      FUN_00531c00(this,param_1);
      FUN_00531c90(this,param_1);
      FUN_00531d20(this,param_1);
      FUN_00531db0(this,param_1);
      FUN_00531e40(this,param_1);
      FUN_00531ed0(this,param_1);
      FUN_00531f60(this,param_1);
      FUN_00531ff0(this,param_1);
      FUN_00532080(this,param_1);
      FUN_00532110(this,param_1);
      FUN_005321a0(this,param_1);
      FUN_00532230(this,param_1);
      FUN_005322c0(this,param_1);
    }
    else if (*(int *)((int)param_1 + 4) == 2) {
      FUN_004f9d40(this,param_1);
      FUN_004f9ca0(this,param_1);
      return iVar2;
    }
  }
  return iVar2;
}

