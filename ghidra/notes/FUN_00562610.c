
bool FUN_00562610(uint param_1,undefined4 *param_2,void *param_3)

{
  undefined4 *puVar1;
  void *pvVar2;
  bool bVar3;
  bool bVar4;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  
  puVar1 = param_2;
  FUN_004ece80(param_2);
  pvVar2 = param_3;
  param_2 = (undefined4 *)0x0;
  bVar3 = FUN_00562710(param_1,(int *)&param_2,(uint)param_3);
  bVar3 = CONCAT31(extraout_var,bVar3) != 0;
  if (param_2 != (undefined4 *)0x0) {
    bVar4 = FUN_0055fc80(param_2,param_1,puVar1,pvVar2);
    if ((CONCAT31(extraout_var_00,bVar4) != 0) && (bVar3)) {
      return true;
    }
    bVar3 = false;
  }
  return bVar3;
}

