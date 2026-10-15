
void __fastcall FUN_005ffe80(int param_1)

{
  BOOL BVar1;
  
  BVar1 = IsWindowEnabled(*(HWND *)(param_1 + 0x18));
  if (BVar1 != 0) {
    EnableWindow(*(HWND *)(param_1 + 0x18),0);
  }
  return;
}

