
undefined4 __cdecl
FUN_0055c940(int param_1,int param_2,int param_3,int *param_4,undefined4 *param_5,
            undefined4 *param_6)

{
  int iVar1;
  int iVar2;
  
  *param_4 = 0;
  *param_5 = 0;
  *param_6 = 0;
  if (param_1 == 0) {
    return 1;
  }
  if (param_3 == 0) {
    if (param_2 == 0) {
      iVar2 = FUN_0053e290(DAT_006bb530);
      *param_4 = iVar2 + DAT_006bb554;
      *param_5 = 1;
      *param_6 = 0;
      return 1;
    }
    iVar1 = FUN_0053e290(DAT_006bb558);
    iVar2 = DAT_006bb520;
  }
  else {
    iVar1 = FUN_0053e290(DAT_006bb54c);
    iVar2 = DAT_006bb5a4;
  }
  *param_4 = iVar1 + iVar2;
  *param_5 = 1;
  *param_6 = 1;
  return 1;
}

