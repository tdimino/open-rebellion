
void FUN_00406820(void)

{
  HWND hWnd;
  LPCSTR lpText;
  LPCSTR lpCaption;
  UINT uType;
  
  uType = 0x41010;
  lpCaption = &DAT_006b0e00;
  lpText = &DAT_006b09f8;
  hWnd = GetActiveWindow();
  MessageBoxA(hWnd,lpText,lpCaption,uType);
  return;
}

