
int __fastcall FUN_005f5c60(int param_1)

{
  int iVar1;
  int iVar2;
  int iVar3;
  
  iVar2 = *(int *)(param_1 + 4);
  iVar1 = 0;
  if (iVar2 == 0) {
    iVar2 = *(int *)(param_1 + 0xc);
    if (*(int *)(param_1 + 0xc) != 0) {
      while ((iVar1 = iVar2, param_1 != 0 && (iVar1 != 0))) {
        if (*(int *)(iVar1 + 8) == param_1) {
          iVar2 = iVar1;
          param_1 = 0;
        }
        else {
          iVar2 = *(int *)(iVar1 + 0xc);
          param_1 = iVar1;
        }
      }
    }
  }
  else {
    iVar3 = *(int *)(iVar2 + 8);
    iVar1 = iVar2;
    if (iVar3 != 0) {
      do {
        iVar2 = iVar3;
        iVar3 = *(int *)(iVar3 + 8);
      } while (iVar3 != 0);
      return iVar2;
    }
  }
  return iVar1;
}

