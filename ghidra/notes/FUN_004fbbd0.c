
void __thiscall FUN_004fbbd0(void *this,int param_1,int param_2,int param_3)

{
  bool bVar1;
  undefined3 extraout_var;
  
  bVar1 = FUN_0053a010((int)this);
  if (CONCAT31(extraout_var,bVar1) != 0) {
    FUN_004f88d0(this,s_GameObjControlKindNotif_006a8f70,s_ControlKind_006a8f88,param_2,param_1,
                 (char *)0x0,param_3);
    FUN_0053fcf0(0x100,this,param_1,param_2,param_3);
  }
  return;
}

