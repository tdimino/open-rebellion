
int FUN_00611bf0(void)

{
  HANDLE pvVar1;
  int iVar2;
  undefined4 *puVar3;
  DWORD DStack_4;
  
  iVar2 = 0;
  if ((DAT_006be688 != (HANDLE)0x0) || (DAT_006be6a8 != 0)) {
    return 7;
  }
  InitializeCriticalSection((LPCRITICAL_SECTION)&lpCriticalSection_006be690);
  DAT_006be6a8 = 1;
  DAT_006be6b0 = 0;
  puVar3 = &DAT_006be630;
  do {
    pvVar1 = CreateEventA((LPSECURITY_ATTRIBUTES)0x0,0,0,(LPCSTR)0x0);
    *puVar3 = pvVar1;
    if (pvVar1 == (HANDLE)0x0) {
      iVar2 = 1;
    }
    puVar3 = puVar3 + 1;
  } while ((int)puVar3 < 0x6be638);
  if (iVar2 == 0) {
    DAT_006be688 = CreateThread((LPSECURITY_ATTRIBUTES)0x0,0x4000,
                                (LPTHREAD_START_ROUTINE)&lpStartAddress_006119f0,(LPVOID)0x0,0,
                                &DStack_4);
    if (DAT_006be688 == (HANDLE)0x0) {
      iVar2 = 1;
    }
    else {
      SetThreadPriority(DAT_006be688,1);
    }
    if (iVar2 == 0) {
      return 0;
    }
  }
  FUN_00611cb0();
  return iVar2;
}

