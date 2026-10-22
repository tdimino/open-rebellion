
void __fastcall FUN_00603710(void *param_1)

{
  HWND hWnd;
  WPARAM wParam;
  LPARAM lParam;
  
  *(undefined4 *)((int)param_1 + 0xf8) = 1;
  if (*(short *)((int)param_1 + 0xdc) != 0) {
    FUN_00610c30((int)param_1 + 0xe0);
  }
  FUN_006030c0(param_1,1);
  if (*(HWND *)((int)param_1 + 0x18) != (HWND)0x0) {
    InvalidateRect(*(HWND *)((int)param_1 + 0x18),(RECT *)0x0,0);
  }
  lParam = 0;
  hWnd = *(HWND *)(*(int *)((int)param_1 + 0x20) + 0x18);
  wParam = FUN_00602510((int)param_1);
  PostMessageA(hWnd,0x111,wParam,lParam);
  return;
}

