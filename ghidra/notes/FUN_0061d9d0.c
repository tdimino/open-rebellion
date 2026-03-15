
void __cdecl FUN_0061d9d0(int param_1)

{
  LPCRITICAL_SECTION lpCriticalSection;
  
  if (*(int *)(&DAT_006ad0b0 + param_1 * 4) == 0) {
    lpCriticalSection = (LPCRITICAL_SECTION)FUN_00619170(0x18);
    if (lpCriticalSection == (LPCRITICAL_SECTION)0x0) {
      __amsg_exit(0x11);
    }
    FUN_0061d9d0(0x11);
    if (*(int *)(&DAT_006ad0b0 + param_1 * 4) == 0) {
      InitializeCriticalSection(lpCriticalSection);
      *(LPCRITICAL_SECTION *)(&DAT_006ad0b0 + param_1 * 4) = lpCriticalSection;
    }
    else {
      FUN_00618e60((undefined *)lpCriticalSection);
    }
    FUN_0061da50(0x11);
  }
  EnterCriticalSection(*(LPCRITICAL_SECTION *)(&DAT_006ad0b0 + param_1 * 4));
  return;
}

