
bool FUN_00601340(void)

{
  HWND hWnd;

  hWnd = GetCapture();
  if (hWnd == (HWND)0x0) {
    hWnd = (HWND)FUN_00601320();
    if (hWnd == (HWND)0x0) goto LAB_0060136d;
  }
  SendMessageA(hWnd,0x40d,0,0);
LAB_0060136d:
  return hWnd == (HWND)0x0;
}
