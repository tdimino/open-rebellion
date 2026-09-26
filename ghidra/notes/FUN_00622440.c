
int __cdecl FUN_00622440(LCID param_1,LCTYPE param_2,LPWSTR param_3,int param_4,UINT param_5)

{
  int iVar1;
  LPSTR lpLCData;
  
  if (DAT_006beb9c == 0) {
    iVar1 = GetLocaleInfoW(0,1,(LPWSTR)0x0,0);
    if (iVar1 == 0) {
      iVar1 = GetLocaleInfoA(0,1,(LPSTR)0x0,0);
      if (iVar1 == 0) {
        return 0;
      }
      DAT_006beb9c = 2;
    }
    else {
      DAT_006beb9c = 1;
    }
  }
  if (DAT_006beb9c == 1) {
    iVar1 = GetLocaleInfoW(param_1,param_2,param_3,param_4);
    return iVar1;
  }
  if (DAT_006beb9c != 2) {
    return DAT_006beb9c;
  }
  if (param_5 == 0) {
    param_5 = DAT_006be850;
  }
  iVar1 = GetLocaleInfoA(param_1,param_2,(LPSTR)0x0,0);
  if (iVar1 != 0) {
    lpLCData = (LPSTR)FUN_00619170(iVar1);
    if (lpLCData == (LPSTR)0x0) {
      return 0;
    }
    iVar1 = GetLocaleInfoA(param_1,param_2,lpLCData,iVar1);
    if (iVar1 != 0) {
      if (param_4 == 0) {
        iVar1 = MultiByteToWideChar(param_5,1,lpLCData,-1,(LPWSTR)0x0,0);
        if (iVar1 != 0) {
          FUN_00618e60(lpLCData);
          return iVar1;
        }
      }
      else {
        iVar1 = MultiByteToWideChar(param_5,1,lpLCData,-1,param_3,param_4);
        if (iVar1 != 0) {
          FUN_00618e60(lpLCData);
          return iVar1;
        }
      }
    }
    FUN_00618e60(lpLCData);
    return 0;
  }
  return 0;
}

