
void __thiscall FUN_005038e0(void *this,undefined4 param_1,undefined4 param_2,int param_3)

{
  bool bVar1;
  undefined3 extraout_var;
  
  bVar1 = FUN_0053a010((int)this);
  if (CONCAT31(extraout_var,bVar1) != 0) {
    FUN_004f8980(this,s_FightSquadSquadSizeDamageNotif_006a9800,s_SquadSizeDamage_006a9820,param_2,
                 param_1,(char *)0x0,param_3);
    FUN_0053fcf0(0x1a0,this,param_1,param_2,param_3);
  }
  return;
}

