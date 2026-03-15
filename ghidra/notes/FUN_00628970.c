
LPCWSTR FUN_00628970(void)

{
  LPCWSTR lpWideCharStr;
  int iVar1;
  byte *lpMultiByteStr;
  int *piVar2;
  
  lpWideCharStr = (LPCWSTR)*DAT_006be7ac;
  piVar2 = DAT_006be7ac;
  if (lpWideCharStr == (LPCWSTR)0x0) {
    return (LPCWSTR)0x0;
  }
  while (((iVar1 = WideCharToMultiByte(1,0,lpWideCharStr,-1,(LPSTR)0x0,0,(LPCSTR)0x0,(LPBOOL)0x0),
          iVar1 != 0 &&
          (lpMultiByteStr = (byte *)FUN_00619170(iVar1), lpMultiByteStr != (byte *)0x0)) &&
         (iVar1 = WideCharToMultiByte(1,0,(LPCWSTR)*piVar2,-1,(LPSTR)lpMultiByteStr,iVar1,
                                      (LPCSTR)0x0,(LPBOOL)0x0), iVar1 != 0))) {
    FUN_006289f0(lpMultiByteStr,0);
    lpWideCharStr = (LPCWSTR)piVar2[1];
    piVar2 = piVar2 + 1;
    if (lpWideCharStr == (LPCWSTR)0x0) {
      return lpWideCharStr;
    }
  }
  return (LPCWSTR)0xffffffff;
}

