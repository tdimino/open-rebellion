
void __thiscall FUN_00502ae0(void *this,undefined4 param_1,undefined4 param_2,int param_3)

{
  bool bVar1;
  undefined3 extraout_var;
  
  bVar1 = FUN_0053a010((int)this);
  if (CONCAT31(extraout_var,bVar1) != 0) {
    FUN_004f8980(this,s_CapShipSpeedCHAllocatedNotif_006a96fc,s_SpeedCHAllocated_006a971c,param_2,
                 param_1,(char *)0x0,param_3);
    FUN_0053fcf0(0x1c4,this,param_1,param_2,param_3);
  }
  return;
}

