
int FUN_0051be20(int param_1,int param_2,int *param_3)

{
  int iVar1;
  int iVar2;
  int iVar3;
  
  iVar3 = 1;
  *param_3 = 0;
  for (iVar1 = thunk_FUN_005f5060(param_2); iVar1 != 0; iVar1 = *(int *)(iVar1 + 0x10)) {
    iVar2 = FUN_0051cab0((uint *)(iVar1 + 0x24));
    if ((iVar2 == 0) || (iVar3 == 0)) {
      iVar3 = 0;
    }
    else {
      iVar3 = 1;
    }
    if (iVar2 != 0) {
      iVar2 = FUN_0053b870(iVar2);
      *param_3 = *param_3 + iVar2;
    }
  }
  *param_3 = param_1 * *param_3;
  return iVar3;
}

