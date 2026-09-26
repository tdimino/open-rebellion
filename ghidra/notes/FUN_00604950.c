
void __fastcall FUN_00604950(void *param_1)

{
  LPPOINT lpPoint;
  HWND hWnd;
  bool bVar1;
  int iVar2;
  HDC hdc;
  HPALETTE hPal;
  
  bVar1 = false;
  hWnd = *(HWND *)((int)param_1 + 0x18);
  lpPoint = (LPPOINT)((int)param_1 + 200);
  GetCursorPos(lpPoint);
  ScreenToClient(hWnd,lpPoint);
  if (*(int *)((int)param_1 + 0xa4) == 1) {
    if ((lpPoint->x <= *(int *)((int)param_1 + 0xb8)) ||
       (*(int *)((int)param_1 + 0xc0) <= lpPoint->x)) goto LAB_00604a06;
    iVar2 = *(int *)((int)param_1 + 0xa0) / 2;
    if ((*(int *)((int)param_1 + 0xcc) <= *(int *)((int)param_1 + 0xbc) + iVar2) ||
       (*(int *)((int)param_1 + 0xc4) - iVar2 <= *(int *)((int)param_1 + 0xcc))) goto LAB_00604a06;
  }
  else {
    if (*(int *)((int)param_1 + 0xa4) != 2) goto LAB_00604a06;
    iVar2 = *(int *)((int)param_1 + 0x9c) / 2;
    if ((((lpPoint->x <= *(int *)((int)param_1 + 0xb8) + iVar2) ||
         (*(int *)((int)param_1 + 0xc0) - iVar2 <= lpPoint->x)) ||
        (*(int *)((int)param_1 + 0xcc) <= *(int *)((int)param_1 + 0xbc))) ||
       (*(int *)((int)param_1 + 0xc4) <= *(int *)((int)param_1 + 0xcc))) goto LAB_00604a06;
  }
  bVar1 = true;
LAB_00604a06:
  if (bVar1) {
    hdc = GetDC(hWnd);
    hPal = SelectPalette(hdc,DAT_006be5a0,0);
    FUN_00604a50(param_1,hdc);
    SelectPalette(hdc,hPal,0);
    ReleaseDC(hWnd,hdc);
  }
  return;
}

