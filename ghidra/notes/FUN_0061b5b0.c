
undefined4 __cdecl FUN_0061b5b0(uint param_1)

{
  int iVar1;
  int iVar2;
  HANDLE hObject;
  BOOL BVar3;
  undefined *puVar4;
  
  iVar1 = FUN_00624730(param_1);
  if (iVar1 != -1) {
    if ((param_1 == 1) || (param_1 == 2)) {
      iVar1 = FUN_00624730(1);
      iVar2 = FUN_00624730(2);
      if (iVar1 == iVar2) goto LAB_0061b606;
    }
    hObject = (HANDLE)FUN_00624730(param_1);
    BVar3 = CloseHandle(hObject);
    if (BVar3 == 0) {
      puVar4 = (undefined *)GetLastError();
      goto LAB_0061b608;
    }
  }
LAB_0061b606:
  puVar4 = (undefined *)0x0;
LAB_0061b608:
  FUN_00624690(param_1);
  *(undefined1 *)((&DAT_006bed20)[(int)param_1 >> 5] + 4 + (param_1 & 0x1f) * 0x24) = 0;
  if (puVar4 != (undefined *)0x0) {
    FUN_0061c1d0(puVar4);
    return 0xffffffff;
  }
  return 0;
}

