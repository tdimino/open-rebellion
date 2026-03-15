
int FUN_0051be90(int *param_1,uint param_2,int param_3,int param_4,void *param_5)

{
  int iVar1;
  int iVar2;
  
  iVar2 = 1;
  if (param_3 <= param_4) {
    do {
      iVar1 = FUN_0055bed0(param_2,param_3);
      if ((iVar1 == 0) || (iVar2 == 0)) {
        iVar2 = 0;
      }
      else {
        iVar2 = 1;
      }
      if ((iVar1 != 0) && (*(int *)(iVar1 + 0x20) == param_3)) {
        iVar1 = FUN_0051bf30(param_1,*(int *)(iVar1 + 0x24),iVar1 + 0x28,param_5);
        if ((iVar1 == 0) || (iVar2 == 0)) {
          iVar2 = 0;
        }
        else {
          iVar2 = 1;
        }
      }
      param_3 = param_3 + 1;
    } while (param_3 <= param_4);
  }
  return iVar2;
}

