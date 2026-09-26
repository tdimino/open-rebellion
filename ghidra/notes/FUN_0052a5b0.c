
int __thiscall FUN_0052a5b0(void *this,void *param_1)

{
  bool bVar1;
  undefined3 extraout_var;
  int iVar2;
  
  bVar1 = FUN_0053a010((int)this);
  iVar2 = CONCAT31(extraout_var,bVar1);
  if (iVar2 != 0) {
    FUN_004f9540(this,param_1);
    if (*(int *)((int)param_1 + 4) == 1) {
      FUN_0052aed0(this,param_1);
      FUN_0052af50(this,param_1);
      FUN_0052afd0(this,param_1);
      FUN_0052b050(this,param_1);
      FUN_0052b170(this,param_1);
      FUN_0052b1f0(this,param_1);
      FUN_0052b270(this,param_1);
      FUN_0052b2f0(this,param_1);
      FUN_0052b410(this,param_1);
      FUN_0052b530(this,param_1);
      FUN_0052b650(this,param_1);
    }
    else if (*(int *)((int)param_1 + 4) == 2) {
      FUN_004f9d40(this,param_1);
      FUN_0052b410(this,param_1);
      FUN_0052b530(this,param_1);
      return iVar2;
    }
  }
  return iVar2;
}

