
int __thiscall FUN_00504710(void *this,void *param_1)

{
  bool bVar1;
  undefined3 extraout_var;
  
  bVar1 = FUN_0053a010((int)this);
  if (CONCAT31(extraout_var,bVar1) != 0) {
    FUN_00558140(this,param_1);
    if (*(int *)((int)param_1 + 4) == 1) {
      FUN_00504850(this,param_1);
      FUN_005048e0(this,param_1);
    }
  }
  return CONCAT31(extraout_var,bVar1);
}

