
void __thiscall FUN_00512670(void *this,int param_1,int param_2)

{
  bool bVar1;
  undefined3 extraout_var;
  
  bVar1 = FUN_0053a010((int)this);
  if (CONCAT31(extraout_var,bVar1) != 0) {
    FUN_004f8880(this,s_SystemResourceIncidentNotif_006a9ff0,s_ResourceIncident_006aa00c,param_1,
                 (char *)0x0,param_2);
    FUN_0053fdd0(0x155,this,param_1,param_2);
  }
  return;
}

