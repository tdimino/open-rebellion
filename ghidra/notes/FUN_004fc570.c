
int __thiscall FUN_004fc570(void *this,int param_1,int param_2)

{
  bool bVar1;
  undefined3 extraout_var;
  int iVar2;
  
  bVar1 = FUN_0053a010((int)this);
  iVar2 = CONCAT31(extraout_var,bVar1);
  if (iVar2 != 0) {
    FUN_004f8880(this,s_GameObjDeployedNotif_006a9428,s_Deployed_006a9440,param_1,(char *)0x0,
                 param_2);
    iVar2 = FUN_0053fdd0(0x107,this,param_1,param_2);
  }
  return iVar2;
}

