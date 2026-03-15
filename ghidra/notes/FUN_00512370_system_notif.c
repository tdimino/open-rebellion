
void __thiscall FUN_00512370(void *this,int param_1,int param_2)

{
  bool bVar1;
  undefined3 extraout_var;
  
  bVar1 = FUN_0053a010((int)this);
  if (CONCAT31(extraout_var,bVar1) != 0) {
    FUN_004f8880(this,s_SystemAssaultNotif_006a9dc4,s_Assault_006a94b8,param_1,(char *)0x0,param_2);
    FUN_0053fcf0(0x150,this,*(uint *)((int)this + 0x24) >> 6 & 3,param_1,param_2);
  }
  return;
}

