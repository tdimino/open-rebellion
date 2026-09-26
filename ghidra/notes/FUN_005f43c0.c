
int FUN_005f43c0(int param_1,int *param_2)

{
  int iVar1;
  int iVar2;
  int iVar3;
  
  iVar2 = 0;
  iVar3 = 0;
  FUN_005f42c0(param_1);
  iVar1 = FUN_005f3fa0(param_1);
  while ((iVar1 != 0 &&
         ((((((iVar1 = FUN_005f3f80(param_1), (char)iVar1 == '0' ||
              (iVar1 = FUN_005f3f80(param_1), (char)iVar1 == '1')) ||
             (iVar1 = FUN_005f3f80(param_1), (char)iVar1 == '2')) ||
            ((iVar1 = FUN_005f3f80(param_1), (char)iVar1 == '3' ||
             (iVar1 = FUN_005f3f80(param_1), (char)iVar1 == '4')))) ||
           ((iVar1 = FUN_005f3f80(param_1), (char)iVar1 == '5' ||
            ((iVar1 = FUN_005f3f80(param_1), (char)iVar1 == '6' ||
             (iVar1 = FUN_005f3f80(param_1), (char)iVar1 == '7')))))) ||
          ((iVar1 = FUN_005f3f80(param_1), (char)iVar1 == '8' ||
           (iVar1 = FUN_005f3f80(param_1), (char)iVar1 == '9'))))))) {
    iVar2 = 1;
    iVar1 = FUN_005f3f80(param_1);
    iVar3 = (char)iVar1 + -0x30 + iVar3 * 10;
    FUN_005f3e50(param_1);
    iVar1 = FUN_005f3fa0(param_1);
  }
  if (iVar2 != 0) {
    *param_2 = iVar3;
  }
  return iVar2;
}

