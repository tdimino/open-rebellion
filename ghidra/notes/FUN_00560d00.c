
bool FUN_00560d00(void *param_1,int param_2,void *param_3)

{
  bool bVar1;
  int iVar2;
  int iVar3;
  undefined3 extraout_var;
  
  bVar1 = true;
  if (param_2 != 0) {
    iVar2 = FUN_00509ae0(param_1,2);
    iVar3 = FUN_00509ae0(param_1,1);
    iVar2 = FUN_0053ec10(param_2,iVar3,iVar2);
    bVar1 = FUN_00561910(iVar2,param_2,param_3);
    bVar1 = CONCAT31(extraout_var,bVar1) != 0;
  }
  return bVar1;
}

