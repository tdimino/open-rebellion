
void __cdecl FUN_00624780(uint param_1)

{
  int iVar1;
  int iVar2;
  
  iVar2 = (param_1 & 0x1f) * 0x24;
  iVar1 = (&DAT_006bed20)[(int)param_1 >> 5] + iVar2;
  if (*(int *)(iVar1 + 8) == 0) {
    FUN_0061d9d0(0x11);
    if (*(int *)(iVar1 + 8) == 0) {
      InitializeCriticalSection((LPCRITICAL_SECTION)(iVar1 + 0xc));
      *(int *)(iVar1 + 8) = *(int *)(iVar1 + 8) + 1;
    }
    FUN_0061da50(0x11);
  }
  EnterCriticalSection((LPCRITICAL_SECTION)((&DAT_006bed20)[(int)param_1 >> 5] + 0xc + iVar2));
  return;
}

