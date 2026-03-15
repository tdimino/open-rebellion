
bool FUN_00562680(uint *param_1,void *param_2)

{
  uint *puVar1;
  void *pvVar2;
  bool bVar3;
  int iVar4;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  bool bVar5;
  
  puVar1 = param_1;
  iVar4 = FUN_00505190(param_1);
  pvVar2 = param_2;
  bVar5 = iVar4 != 0;
  param_1 = (uint *)0x0;
  if (iVar4 != 0) {
    bVar3 = FUN_00562710(*(uint *)(iVar4 + 0x24) >> 6 & 3,(int *)&param_1,(uint)param_2);
    if ((CONCAT31(extraout_var,bVar3) == 0) || (!bVar5)) {
      bVar5 = false;
    }
    else {
      bVar5 = true;
    }
    if (param_1 != (uint *)0x0) {
      bVar3 = FUN_0055fe70(puVar1,param_1,pvVar2);
      if ((CONCAT31(extraout_var_00,bVar3) != 0) && (bVar5)) {
        return true;
      }
      bVar5 = false;
    }
  }
  return bVar5;
}

