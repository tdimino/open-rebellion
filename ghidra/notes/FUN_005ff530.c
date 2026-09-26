
HGDIOBJ FUN_005ff530(int *param_1,uint param_2,HWND param_3,LPARAM param_4)

{
  HWND hWnd;
  HDC pHVar1;
  HGDIOBJ pvVar2;
  HWND hWnd_00;
  HPALETTE hPal;
  HGDIOBJ unaff_EBP;
  tagPAINTSTRUCT local_40;
  
  hWnd = (HWND)param_1[6];
  if (param_2 < 0x10) {
    if (param_2 == 0xf) {
      pHVar1 = BeginPaint(hWnd,&local_40);
      (**(code **)(*param_1 + 0x28))(pHVar1);
      (**(code **)(*param_1 + 0x38))(pHVar1);
      EndPaint(hWnd,(PAINTSTRUCT *)&stack0xffffffb8);
      return unaff_EBP;
    }
    if (param_2 == 2) {
      return (HGDIOBJ)0x0;
    }
  }
  else {
    if (0x135 < param_2) {
      if (param_2 != 0x30f) {
        if (param_2 != 0x311) goto LAB_005ff5e6;
        InvalidateRect(hWnd,(RECT *)0x0,0);
        for (hWnd_00 = GetWindow(hWnd,5); hWnd_00 != (HWND)0x0; hWnd_00 = GetWindow(hWnd_00,2)) {
          SendMessageA(hWnd_00,0x311,(WPARAM)param_3,param_4);
        }
        if (hWnd == param_3) {
          return (HGDIOBJ)0x0;
        }
      }
      pHVar1 = GetDC(hWnd);
      hPal = SelectPalette(pHVar1,DAT_006be5a0,0);
      pvVar2 = (HGDIOBJ)RealizePalette(pHVar1);
      SelectPalette(pHVar1,hPal,0);
      if (pvVar2 != (HGDIOBJ)0x0) {
        InvalidateRect(hWnd,(RECT *)0x0,0);
      }
      ReleaseDC(hWnd,pHVar1);
      return pvVar2;
    }
    if (param_2 == 0x135) {
      pvVar2 = GetStockObject(5);
      return pvVar2;
    }
    if (param_2 == 0x14) {
      return (HGDIOBJ)0x1;
    }
  }
LAB_005ff5e6:
  pvVar2 = (HGDIOBJ)FUN_006007b0(param_1,param_2,(uint)param_3,param_4);
  return pvVar2;
}

