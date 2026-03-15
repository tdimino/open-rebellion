
int __thiscall FUN_00511ec0(void *this,int param_1,int param_2,int param_3)

{
  bool bVar1;
  undefined3 extraout_var;
  
  bVar1 = FUN_0053a010((int)this);
  if (CONCAT31(extraout_var,bVar1) != 0) {
    FUN_004f88d0(this,s__SystemControlKindBattleWonNotif_006a9b7c,s_ControlKindBattleWon_006a9ba0,
                 param_2,param_1,(char *)0x0,param_3);
  }
  return CONCAT31(extraout_var,bVar1);
}

