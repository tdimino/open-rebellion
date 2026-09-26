
int FUN_0051b750(void *param_1)

{
  bool bVar1;
  bool bVar2;
  undefined3 extraout_var;
  int iVar3;
  undefined3 extraout_var_00;
  int iVar4;
  void *local_4;
  
  local_4 = (void *)0x0;
  bVar1 = FUN_005187a0(1,(int *)&local_4);
  bVar1 = CONCAT31(extraout_var,bVar1) != 0;
  if (local_4 != (void *)0x0) {
    iVar3 = FUN_0052fff0(local_4,param_1);
    if ((iVar3 == 0) || (!bVar1)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
  }
  bVar2 = FUN_005187a0(2,(int *)&local_4);
  if ((CONCAT31(extraout_var_00,bVar2) == 0) || (!bVar1)) {
    iVar3 = 0;
  }
  else {
    iVar3 = 1;
  }
  if (local_4 != (void *)0x0) {
    iVar4 = FUN_0052fff0(local_4,param_1);
    if ((iVar4 != 0) && (iVar3 != 0)) {
      return 1;
    }
    iVar3 = 0;
  }
  return iVar3;
}

