
undefined4 __cdecl FUN_00559e10(int *param_1,int *param_2)

{
  int *piVar1;
  bool bVar2;
  int iVar3;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  int iVar4;
  int iVar5;
  
  piVar1 = param_2;
  iVar3 = FUN_0053e130(*param_1,*param_2);
  iVar4 = 0;
  iVar5 = 0;
  param_2 = (int *)0x0;
  if (0 < iVar3) {
    do {
      if ((int)param_2 < *param_1) {
        bVar2 = FUN_0053e2f0((((*piVar1 - iVar4) - iVar5) + *param_1) * DAT_006bb3ec);
        if (CONCAT31(extraout_var,bVar2) != 0) {
          iVar4 = iVar4 + 1;
        }
      }
      if ((int)param_2 < *piVar1) {
        bVar2 = FUN_0053e2f0((((*param_1 - iVar4) - iVar5) + *piVar1) * DAT_006bb3ec);
        if (CONCAT31(extraout_var_00,bVar2) != 0) {
          iVar5 = iVar5 + 1;
        }
      }
      param_2 = (int *)((int)param_2 + 1);
    } while ((int)param_2 < iVar3);
  }
  if ((iVar4 == 0) && (iVar5 == 0)) {
    if (*param_1 == 0) {
      if (*piVar1 != 0) {
        iVar5 = 1;
      }
    }
    else {
      iVar4 = 1;
    }
  }
  *param_1 = *param_1 - iVar4;
  iVar5 = *piVar1 - iVar5;
  *piVar1 = iVar5;
  if (iVar5 < *param_1) {
    *param_1 = iVar5;
  }
  return 1;
}

