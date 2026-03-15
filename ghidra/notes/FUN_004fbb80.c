
void __thiscall FUN_004fbb80(void *this,int param_1,int param_2,int param_3)

{
  bool bVar1;
  undefined3 extraout_var;
  
  bVar1 = FUN_0053a010((int)this);
  if (CONCAT31(extraout_var,bVar1) != 0) {
    FUN_004f8a50(this,s_GameObjNameNotif_006a8f54,&DAT_006a8f68,param_2,param_1,(char *)0x0,param_3)
    ;
    FUN_0053ff20(0x500,this,param_1,param_2,param_3);
  }
  return;
}

