
void FUN_00611cb0(void)

{
  DWORD DVar1;
  undefined4 *puVar2;
  
  DAT_006be6b0 = 1;
  if (DAT_006be688 != (HANDLE)0x0) {
    if (DAT_006be634 != (HANDLE)0x0) {
      SetEvent(DAT_006be634);
    }
    DVar1 = WaitForSingleObject(DAT_006be688,0x5dc);
    if ((DVar1 == 0x102) && (DAT_006be6ac != 0)) {
      WaitForSingleObject(DAT_006be688,15000);
    }
    CloseHandle(DAT_006be688);
    DAT_006be688 = (HANDLE)0x0;
  }
  puVar2 = &DAT_006be630;
  do {
    if ((HANDLE)*puVar2 != (HANDLE)0x0) {
      CloseHandle((HANDLE)*puVar2);
      *puVar2 = 0;
    }
    puVar2 = puVar2 + 1;
  } while ((int)puVar2 < 0x6be638);
  if (DAT_006be6a8 != 0) {
    DeleteCriticalSection((LPCRITICAL_SECTION)&lpCriticalSection_006be690);
    DAT_006be6a8 = 0;
  }
  return;
}

