
int __cdecl
FUN_00622170(LCID param_1,DWORD param_2,byte *param_3,LPWSTR param_4,byte *param_5,int param_6,
            UINT param_7)

{
  int iVar1;
  LPWSTR cbMultiByte;
  BOOL BVar2;
  BYTE *pBVar3;
  PCNZWCH lpWideCharStr;
  int iVar4;
  int iVar5;
  int iStack_18;
  _cpinfo _Stack_14;
  
  if (DAT_006beb98 == 0) {
    iVar1 = CompareStringW(0,0,(PCNZWCH)&lpString2_0066e924,1,(PCNZWCH)&lpString2_0066e924,1);
    if (iVar1 == 0) {
      iVar1 = CompareStringA(0,0,(PCNZCH)&lpString2_0066e920,1,(PCNZCH)&lpString2_0066e920,1);
      if (iVar1 == 0) {
        return 0;
      }
      DAT_006beb98 = 2;
    }
    else {
      DAT_006beb98 = 1;
    }
  }
  cbMultiByte = param_4;
  if (0 < (int)param_4) {
    cbMultiByte = (LPWSTR)FUN_006210d0((char *)param_3,(int)param_4);
  }
  if (0 < param_6) {
    param_6 = FUN_006210d0((char *)param_5,param_6);
  }
  if (DAT_006beb98 == 2) {
    iVar1 = CompareStringA(param_1,param_2,(PCNZCH)param_3,(int)cbMultiByte,(PCNZCH)param_5,param_6)
    ;
    return iVar1;
  }
  iStack_18 = DAT_006beb98;
  if (DAT_006beb98 == 1) {
    iStack_18 = 0;
    param_4 = (LPWSTR)0x0;
    if (param_7 == 0) {
      param_7 = DAT_006be850;
    }
    if ((cbMultiByte == (LPWSTR)0x0) || (param_6 == 0)) {
      if (cbMultiByte == (LPWSTR)param_6) {
        return 2;
      }
      if (1 < param_6) {
        return 1;
      }
      if (1 < (int)cbMultiByte) {
        return 3;
      }
      BVar2 = GetCPInfo(param_7,&_Stack_14);
      if (BVar2 == 0) {
        return 0;
      }
      if (0 < (int)cbMultiByte) {
        if (_Stack_14.MaxCharSize < 2) {
          return 3;
        }
        pBVar3 = _Stack_14.LeadByte;
        while( true ) {
          if ((_Stack_14.LeadByte[0] == 0) || (pBVar3[1] == 0)) {
            return 3;
          }
          if ((*pBVar3 <= *param_3) && (*param_3 <= pBVar3[1])) break;
          _Stack_14.LeadByte[0] = pBVar3[2];
          pBVar3 = pBVar3 + 2;
        }
        return 2;
      }
      if (0 < param_6) {
        if (_Stack_14.MaxCharSize < 2) {
          return 1;
        }
        pBVar3 = _Stack_14.LeadByte;
        while( true ) {
          if ((_Stack_14.LeadByte[0] == 0) || (pBVar3[1] == 0)) {
            return 1;
          }
          if ((*pBVar3 <= *param_5) && (*param_5 <= pBVar3[1])) break;
          _Stack_14.LeadByte[0] = pBVar3[2];
          pBVar3 = pBVar3 + 2;
        }
        return 2;
      }
    }
    iVar1 = MultiByteToWideChar(param_7,9,(LPCSTR)param_3,(int)cbMultiByte,(LPWSTR)0x0,0);
    if (iVar1 == 0) {
      return 0;
    }
    lpWideCharStr = (PCNZWCH)FUN_00619170(iVar1 * 2);
    if (lpWideCharStr == (PCNZWCH)0x0) {
      return 0;
    }
    iVar4 = MultiByteToWideChar(param_7,1,(LPCSTR)param_3,(int)cbMultiByte,lpWideCharStr,iVar1);
    if ((((iVar4 != 0) &&
         (iVar4 = MultiByteToWideChar(param_7,9,(LPCSTR)param_5,param_6,(LPWSTR)0x0,0), iVar4 != 0))
        && (param_4 = (LPWSTR)FUN_00619170(iVar4 * 2), param_4 != (LPWSTR)0x0)) &&
       (iVar5 = MultiByteToWideChar(param_7,1,(LPCSTR)param_5,param_6,param_4,iVar4), iVar5 != 0)) {
      iStack_18 = CompareStringW(param_1,param_2,lpWideCharStr,iVar1,param_4,iVar4);
    }
    FUN_00618e60((undefined *)lpWideCharStr);
    FUN_00618e60((undefined *)param_4);
  }
  return iStack_18;
}

