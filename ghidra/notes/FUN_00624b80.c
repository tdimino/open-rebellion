
LPSTR __cdecl FUN_00624b80(LPSTR param_1,WCHAR param_2)

{
  LPSTR pCVar1;
  DWORD *pDVar2;
  
  pCVar1 = param_1;
  if (param_1 == (LPSTR)0x0) {
    return param_1;
  }
  if (DAT_006be840 == 0) {
    if ((ushort)param_2 < 0x100) {
      *param_1 = (CHAR)param_2;
      return (LPSTR)0x1;
    }
  }
  else {
    param_1 = (LPSTR)0x0;
    pCVar1 = (LPSTR)WideCharToMultiByte(DAT_006be850,0x220,&param_2,1,pCVar1,DAT_006af3ac,
                                        (LPCSTR)0x0,(LPBOOL)&param_1);
    if ((pCVar1 != (LPSTR)0x0) && (param_1 == (LPSTR)0x0)) {
      return pCVar1;
    }
  }
  pDVar2 = FUN_0061c250();
  *pDVar2 = 0x2a;
  return (LPSTR)0xffffffff;
}

