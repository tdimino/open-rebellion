
int __thiscall FUN_004fefa0(void *this,void *param_1)

{
  bool bVar1;
  undefined3 extraout_var;
  
  bVar1 = FUN_0053a010((int)this);
  if (CONCAT31(extraout_var,bVar1) != 0) {
    FUN_004f9540(this,param_1);
    if (*(int *)((int)param_1 + 4) == 1) {
      FUN_004ff180(this,param_1);
      FUN_004ff210(this,param_1);
      FUN_004ff2a0(this,param_1);
      FUN_004ff330(this,param_1);
      FUN_004ff3c0(this,param_1);
      FUN_004ff450(this,param_1);
      FUN_004ff4e0(this,param_1);
    }
  }
  return CONCAT31(extraout_var,bVar1);
}

