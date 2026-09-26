
int __thiscall FUN_004fbe90(void *this,undefined4 param_1,undefined4 param_2,int param_3)

{
  bool bVar1;
  undefined3 extraout_var;
  
  bVar1 = FUN_0053a010((int)this);
  if (CONCAT31(extraout_var,bVar1) != 0) {
    FUN_004f8c60(this,s__GameObjDestroyedReasonNotif__006a9050,s_DestroyedReason_006a9070,
                 &DAT_006b120c,&DAT_006b120c,(char *)0x0,param_3);
  }
  return CONCAT31(extraout_var,bVar1);
}

