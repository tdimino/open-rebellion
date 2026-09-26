
uint __cdecl FUN_0061c540(LPCSTR param_1,uint param_2,uint param_3,undefined4 param_4)

{
  uint uVar1;
  HANDLE hFile;
  undefined *puVar2;
  int iVar3;
  DWORD *pDVar4;
  DWORD DVar5;
  DWORD dwCreationDisposition;
  DWORD dwFlagsAndAttributes;
  int iVar6;
  bool bVar7;
  byte local_11;
  uint local_10;
  _SECURITY_ATTRIBUTES local_c;
  
  bVar7 = (param_2 & 0x80) == 0;
  local_c.nLength = 0xc;
  local_c.lpSecurityDescriptor = (LPVOID)0x0;
  if (bVar7) {
    local_11 = 0;
  }
  else {
    local_11 = 0x10;
  }
  local_c.bInheritHandle = (BOOL)bVar7;
  if (((param_2 & 0x8000) == 0) && (((param_2 & 0x4000) != 0 || (DAT_006bec08 != 0x8000)))) {
    local_11 = local_11 | 0x80;
  }
  uVar1 = param_2 & 3;
  if (uVar1 == 0) {
    local_10 = 0x80000000;
  }
  else if (uVar1 == 1) {
    local_10 = 0x40000000;
  }
  else {
    if (uVar1 != 2) goto switchD_0061c5d8_caseD_11;
    local_10 = 0xc0000000;
  }
  switch(param_3) {
  case 0x10:
    DVar5 = 0;
    break;
  default:
    goto switchD_0061c5d8_caseD_11;
  case 0x20:
    DVar5 = 1;
    break;
  case 0x30:
    DVar5 = 2;
    break;
  case 0x40:
    DVar5 = 3;
  }
  uVar1 = param_2 & 0x700;
  if (uVar1 < 0x101) {
    if (uVar1 == 0x100) {
      dwCreationDisposition = 4;
    }
    else {
      if (uVar1 != 0) goto switchD_0061c5d8_caseD_11;
LAB_0061c646:
      dwCreationDisposition = 3;
    }
  }
  else if (uVar1 < 0x301) {
    if (uVar1 == 0x300) {
      dwCreationDisposition = 2;
    }
    else {
      if (uVar1 != 0x200) goto switchD_0061c5d8_caseD_11;
LAB_0061c666:
      dwCreationDisposition = 5;
    }
  }
  else {
    if (uVar1 < 0x501) {
      if (uVar1 != 0x500) {
        if (uVar1 != 0x400) {
switchD_0061c5d8_caseD_11:
          pDVar4 = FUN_0061c250();
          *pDVar4 = 0x16;
          pDVar4 = FUN_0061c260();
          *pDVar4 = 0;
          return 0xffffffff;
        }
        goto LAB_0061c646;
      }
    }
    else {
      if (uVar1 == 0x600) goto LAB_0061c666;
      if (uVar1 != 0x700) goto switchD_0061c5d8_caseD_11;
    }
    dwCreationDisposition = 1;
  }
  dwFlagsAndAttributes = 0x80;
  if (((param_2 & 0x100) != 0) && (((byte)param_4 & ~(byte)DAT_006be784 & 0x80) == 0)) {
    dwFlagsAndAttributes = 1;
  }
  if ((param_2 & 0x40) != 0) {
    dwFlagsAndAttributes = dwFlagsAndAttributes | 0x4000000;
    local_10 = local_10 | 0x10000;
  }
  if ((param_2 & 0x1000) != 0) {
    dwFlagsAndAttributes = dwFlagsAndAttributes | 0x100;
  }
  if ((param_2 & 0x20) == 0) {
    if ((param_2 & 0x10) != 0) {
      dwFlagsAndAttributes = dwFlagsAndAttributes | 0x10000000;
    }
  }
  else {
    dwFlagsAndAttributes = dwFlagsAndAttributes | 0x8000000;
  }
  uVar1 = FUN_00624470();
  if (uVar1 == 0xffffffff) {
    pDVar4 = FUN_0061c250();
    *pDVar4 = 0x18;
    pDVar4 = FUN_0061c260();
    *pDVar4 = 0;
    return 0xffffffff;
  }
  hFile = CreateFileA(param_1,local_10,DVar5,&local_c,dwCreationDisposition,dwFlagsAndAttributes,
                      (HANDLE)0x0);
  if (hFile != (HANDLE)0xffffffff) {
    DVar5 = GetFileType(hFile);
    if (DVar5 != 0) {
      if (DVar5 == 2) {
        local_11 = local_11 | 0x40;
      }
      else if (DVar5 == 3) {
        local_11 = local_11 | 8;
      }
      FUN_006245e0(uVar1,hFile);
      iVar6 = (uVar1 & 0x1f) * 0x24;
      *(byte *)(iVar6 + 4 + (&DAT_006bed20)[(int)uVar1 >> 5]) = local_11 | 1;
      if ((((local_11 & 0x48) == 0) && ((local_11 & 0x80) != 0)) && ((param_2 & 2) != 0)) {
        DVar5 = FUN_0061bc00(uVar1,-1,2);
        if (DVar5 == 0xffffffff) {
          pDVar4 = FUN_0061c260();
          if (*pDVar4 != 0x83) {
LAB_0061c826:
            FUN_0061b540(uVar1);
            FUN_006247f0(uVar1);
            return 0xffffffff;
          }
        }
        else {
          param_3 = param_3 & 0xffffff00;
          iVar3 = FUN_0061b950(uVar1,(char *)&param_3,1);
          if ((((iVar3 == 0) && ((char)param_3 == '\x1a')) && (iVar3 = FUN_00624820(), iVar3 == -1))
             || (DVar5 = FUN_0061bc00(uVar1,0,0), DVar5 == 0xffffffff)) goto LAB_0061c826;
        }
      }
      if (((local_11 & 0x48) == 0) && ((param_2 & 8) != 0)) {
        *(byte *)(iVar6 + 4 + (&DAT_006bed20)[(int)uVar1 >> 5]) =
             *(byte *)(iVar6 + 4 + (&DAT_006bed20)[(int)uVar1 >> 5]) | 0x20;
      }
      FUN_006247f0(uVar1);
      return uVar1;
    }
    CloseHandle(hFile);
  }
  puVar2 = (undefined *)GetLastError();
  FUN_0061c1d0(puVar2);
  FUN_006247f0(uVar1);
  return 0xffffffff;
}

