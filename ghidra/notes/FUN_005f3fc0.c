
LPBYTE __cdecl FUN_005f3fc0(LPCSTR param_1,LPCSTR param_2)

{
  LSTATUS LVar1;
  uint uVar2;
  LPBYTE lpData;
  LPBYTE pBVar3;
  DWORD local_c;
  HKEY local_8;
  DWORD local_4;
  
  local_8 = (HKEY)0x0;
  lpData = (LPBYTE)0x0;
  local_4 = 0;
  local_c = 0;
  LVar1 = RegOpenKeyExA((HKEY)0x80000002,param_1,0,0x20019,&local_8);
  if (LVar1 == 0) {
    LVar1 = RegQueryValueExA(local_8,param_2,(LPDWORD)0x0,&local_4,(LPBYTE)0x0,&local_c);
    if ((LVar1 == 0) && (local_c != 0)) {
      lpData = (LPBYTE)FUN_00618b70(local_c);
      if (lpData != (LPBYTE)0x0) {
        pBVar3 = lpData;
        for (uVar2 = local_c >> 2; uVar2 != 0; uVar2 = uVar2 - 1) {
          pBVar3[0] = '\0';
          pBVar3[1] = '\0';
          pBVar3[2] = '\0';
          pBVar3[3] = '\0';
          pBVar3 = pBVar3 + 4;
        }
        for (uVar2 = local_c & 3; uVar2 != 0; uVar2 = uVar2 - 1) {
          *pBVar3 = '\0';
          pBVar3 = pBVar3 + 1;
        }
        LVar1 = RegQueryValueExA(local_8,param_2,(LPDWORD)0x0,&local_4,lpData,&local_c);
        if (LVar1 != 0) {
          FUN_00618b60(lpData);
          lpData = (LPBYTE)0x0;
        }
      }
    }
    RegCloseKey(local_8);
  }
  return lpData;
}

