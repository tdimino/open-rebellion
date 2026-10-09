
void FUN_0041d9d0(void)

{
  int iVar1;
  
  DAT_006b14ac = SetWindowsHookExA(2,(HOOKPROC)&lpfn_0041d8f0,(HINSTANCE)0x0,0);
  DAT_006b14b0 = SetWindowsHookExA(7,lpfn_0041d950,(HINSTANCE)0x0,0);
  DAT_006b14b4 = 0;
  DAT_006b14bc = 1;
  iVar1 = FUN_00422ca0();
  if (iVar1 != 0) {
    *(undefined4 *)(iVar1 + 0xa0) = 0;
  }
  return;
}

