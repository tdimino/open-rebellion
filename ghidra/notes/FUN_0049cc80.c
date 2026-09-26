
undefined4 __fastcall FUN_0049cc80(int *param_1)

{
  int iVar1;
  int iVar2;
  int *piVar3;
  
  iVar2 = 0;
  if (param_1[0x1f] != 1) {
    if (param_1[0x1f] == 2) {
      if ((param_1[1] & 0x80000000U) != 0) {
        if ((param_1[1] & 0x20000000U) != 0) {
          piVar3 = param_1 + 0x24;
          for (iVar2 = 0x1a; iVar2 != 0; iVar2 = iVar2 + -1) {
            *piVar3 = 0;
            piVar3 = piVar3 + 1;
          }
          iVar2 = thunk_FUN_005f5060((int)(param_1 + 8));
          param_1[0x1d] = iVar2;
          while (iVar2 != 0) {
            FUN_0049db10(param_1,(int *)(param_1[0x1d] + 0x60));
            iVar2 = *(int *)(param_1[0x1d] + 0x10);
            param_1[0x1d] = iVar2;
          }
        }
        FUN_00619730();
        param_1[1] = param_1[1] & 0x5fffffff;
      }
      param_1[0x1f] = 1;
      return 1;
    }
    param_1[0x1f] = 1;
    return 0;
  }
  if (param_1[0x1d] == 0) {
    iVar1 = thunk_FUN_005f5060((int)(param_1 + 8));
    param_1[0x1d] = iVar1;
    if (iVar1 == 0) goto LAB_0049cd53;
  }
  do {
    if (iVar2 != 0) break;
    iVar2 = FUN_004c6770((void *)param_1[0x1d],*param_1);
    if (iVar2 != 0) {
      param_1[1] = param_1[1] | 0xa0000000;
    }
    iVar1 = *(int *)(param_1[0x1d] + 0x10);
    param_1[0x1d] = iVar1;
  } while (iVar1 != 0);
  if (param_1[0x1d] != 0) {
    return 0;
  }
LAB_0049cd53:
  param_1[0x1f] = 2;
  return 0;
}

