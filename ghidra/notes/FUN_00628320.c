
uint __cdecl FUN_00628320(LPSTR param_1,LPCWSTR param_2,uint param_3)

{
  char cVar1;
  uint uVar2;
  uint uVar3;
  int iVar4;
  DWORD DVar5;
  DWORD *pDVar6;
  LPCWSTR pWVar7;
  int iVar8;
  BOOL local_4;
  
  uVar3 = param_3;
  pWVar7 = param_2;
  uVar2 = 0;
  local_4 = 0;
  if ((param_1 != (LPSTR)0x0) && (param_3 == 0)) {
    return uVar2;
  }
  if (param_1 == (LPSTR)0x0) {
    if (DAT_006be840 == 0) {
      uVar3 = FUN_006267c0(param_2);
      return uVar3;
    }
    iVar4 = WideCharToMultiByte(DAT_006be850,0x220,param_2,-1,(LPSTR)0x0,0,(LPCSTR)0x0,&local_4);
    if ((iVar4 != 0) && (local_4 == 0)) {
      return iVar4 - 1;
    }
  }
  else if (DAT_006be840 == 0) {
    if (param_3 == 0) {
      return 0;
    }
    while ((ushort)*pWVar7 < 0x100) {
      param_1[uVar2] = (CHAR)*pWVar7;
      if (*pWVar7 == L'\0') {
        return uVar2;
      }
      uVar2 = uVar2 + 1;
      pWVar7 = pWVar7 + 1;
      if (param_3 <= uVar2) {
        return uVar2;
      }
    }
  }
  else if (DAT_006af3ac == 1) {
    if (param_3 != 0) {
      uVar3 = FUN_00628510(param_2,param_3);
    }
    uVar3 = WideCharToMultiByte(DAT_006be850,0x220,pWVar7,uVar3,param_1,uVar3,(LPCSTR)0x0,&local_4);
    if ((uVar3 != 0) && (local_4 == 0)) {
      if (param_1[uVar3 - 1] != '\0') {
        return uVar3;
      }
      return uVar3 - 1;
    }
  }
  else {
    iVar4 = WideCharToMultiByte(DAT_006be850,0x220,param_2,-1,param_1,param_3,(LPCSTR)0x0,&local_4);
    if (iVar4 == 0) {
      if ((local_4 == 0) && (DVar5 = GetLastError(), DVar5 == 0x7a)) {
        uVar2 = 0;
        if (uVar3 != 0) {
          do {
            iVar4 = WideCharToMultiByte(DAT_006be850,0,pWVar7,1,(LPSTR)&param_2,DAT_006af3ac,
                                        (LPCSTR)0x0,&local_4);
            if ((iVar4 == 0) || (local_4 != 0)) goto LAB_006284f6;
            if (uVar3 < iVar4 + uVar2) {
              return uVar2;
            }
            iVar8 = 0;
            if (0 < iVar4) {
              do {
                cVar1 = *(char *)((int)&param_2 + iVar8);
                param_1[uVar2] = cVar1;
                if (cVar1 == '\0') {
                  return uVar2;
                }
                iVar8 = iVar8 + 1;
                uVar2 = uVar2 + 1;
              } while (iVar8 < iVar4);
            }
            pWVar7 = pWVar7 + 1;
          } while (uVar2 < uVar3);
        }
        return uVar2;
      }
    }
    else if (local_4 == 0) {
      return iVar4 - 1;
    }
  }
LAB_006284f6:
  pDVar6 = FUN_0061c250();
  *pDVar6 = 0x2a;
  return 0xffffffff;
}

