
bool __fastcall FUN_00442c20(int param_1)

{
  HINSTANCE hmod;
  BOOL BVar1;
  DWORD dwThreadId;
  
  BVar1 = FUN_00442c60();
  if (BVar1 != 0) {
    hmod = *(HINSTANCE *)(param_1 + 0x1c);
    dwThreadId = GetCurrentThreadId();
    DAT_006b28f8 = SetWindowsHookExA(7,(HOOKPROC)&lpfn_00442b90,hmod,dwThreadId);
  }
  return DAT_006b28f8 != (HHOOK)0x0;
}

