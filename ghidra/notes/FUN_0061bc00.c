
DWORD __cdecl FUN_0061bc00(uint param_1,LONG param_2,DWORD param_3)

{
  HANDLE hFile;
  DWORD *pDVar1;
  DWORD DVar2;
  undefined *puVar3;
  
  hFile = (HANDLE)FUN_00624730(param_1);
  if (hFile == (HANDLE)0xffffffff) {
    pDVar1 = FUN_0061c250();
    *pDVar1 = 9;
    return 0xffffffff;
  }
  DVar2 = SetFilePointer(hFile,param_2,(PLONG)0x0,param_3);
  if (DVar2 == 0xffffffff) {
    puVar3 = (undefined *)GetLastError();
  }
  else {
    puVar3 = (undefined *)0x0;
  }
  if (puVar3 != (undefined *)0x0) {
    FUN_0061c1d0(puVar3);
    return 0xffffffff;
  }
  *(byte *)((&DAT_006bed20)[(int)param_1 >> 5] + 4 + (param_1 & 0x1f) * 0x24) =
       *(byte *)((&DAT_006bed20)[(int)param_1 >> 5] + 4 + (param_1 & 0x1f) * 0x24) & 0xfd;
  return DVar2;
}

