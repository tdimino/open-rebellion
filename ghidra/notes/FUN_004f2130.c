
void __thiscall FUN_004f2130(void *this,uint *param_1,uint *param_2,int param_3)

{
  bool bVar1;
  undefined3 extraout_var;
  
  bVar1 = FUN_0053a010((int)this);
  if (CONCAT31(extraout_var,bVar1) != 0) {
    FUN_004f8aa0(this,s_CharacterCommandingKeyNotif_006a8c88,s_Commanding_006a8ca4,param_2,param_1,
                 (char *)0x0,param_3);
    FUN_0053fe40(0x362,this,param_1,param_2,param_3);
  }
  return;
}

