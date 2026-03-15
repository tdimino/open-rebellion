
undefined4 __cdecl
FUN_0055ebc0(int param_1,int param_2,int param_3,int param_4,undefined4 *param_5,int *param_6,
            undefined4 *param_7,undefined4 *param_8)

{
  int iVar1;
  int iVar2;
  
  *param_5 = 0;
  *param_6 = 0;
  *param_7 = 0;
  iVar2 = param_1 - param_2;
  *param_8 = 0;
  iVar1 = (uint)(param_4 != 0) + iVar2 + (uint)(param_3 != 0);
  if (iVar1 != 0) {
    iVar1 = FUN_0053e290(iVar1 + -1);
    if (iVar1 < iVar2) {
      *param_5 = 1;
      *param_6 = iVar1;
      return 1;
    }
    if (iVar1 == iVar2) {
      if (param_3 != 0) {
        *param_7 = 1;
        return 1;
      }
      if (param_4 != 0) {
        *param_8 = 1;
        return 1;
      }
    }
    else if (param_4 != 0) {
      *param_8 = 1;
    }
  }
  return 1;
}

