
int __thiscall FUN_00536800(void *this,uint *param_1,uint *param_2,void *param_3)

{
  bool bVar1;
  undefined3 extraout_var;
  int iVar2;
  uint uVar3;
  
  bVar1 = FUN_0053a010((int)this);
  iVar2 = CONCAT31(extraout_var,bVar1);
  if (iVar2 != 0) {
    FUN_004f8aa0(this,s_RoleMissionKeyNotif_006aa930,s_Mission_006aa944,param_2,param_1,(char *)0x0,
                 (int)param_3);
    iVar2 = FUN_0053fe40(0x380,this,param_1,param_2,(int)param_3);
    uVar3 = FUN_0053f9f0(0x321,(int *)this,param_1,param_2,param_3);
    if ((uVar3 != 0) && (iVar2 != 0)) {
      return 1;
    }
    iVar2 = 0;
  }
  return iVar2;
}

