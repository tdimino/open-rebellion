
DWORD __cdecl FUN_00624280(uint param_1)

{
  HANDLE hFile;
  BOOL BVar1;
  DWORD DVar2;
  DWORD *pDVar3;
  int iVar4;
  DWORD DVar5;
  
  if (DAT_006bee20 <= param_1) {
LAB_0062430c:
    pDVar3 = FUN_0061c250();
    *pDVar3 = 9;
    return 0xffffffff;
  }
  iVar4 = (param_1 & 0x1f) * 0x24;
  if ((*(byte *)((&DAT_006bed20)[(int)param_1 >> 5] + 4 + iVar4) & 1) == 0) goto LAB_0062430c;
  FUN_00624780(param_1);
  if ((*(byte *)((&DAT_006bed20)[(int)param_1 >> 5] + 4 + iVar4) & 1) != 0) {
    hFile = (HANDLE)FUN_00624730(param_1);
    BVar1 = FlushFileBuffers(hFile);
    if (BVar1 == 0) {
      DVar2 = GetLastError();
    }
    else {
      DVar2 = 0;
    }
    DVar5 = 0;
    if (DVar2 == 0) goto LAB_006242fd;
    pDVar3 = FUN_0061c260();
    *pDVar3 = DVar2;
  }
  pDVar3 = FUN_0061c250();
  *pDVar3 = 9;
  DVar5 = 0xffffffff;
LAB_006242fd:
  FUN_006247f0(param_1);
  return DVar5;
}

