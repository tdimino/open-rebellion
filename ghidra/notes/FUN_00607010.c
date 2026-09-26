
undefined4 __fastcall FUN_00607010(int param_1)

{
  HWND pHVar1;
  HWND pHVar2;
  BOOL BVar3;
  HDC hdc;
  HGDIOBJ pvVar4;
  HGDIOBJ pvVar5;
  int X;
  int iVar6;
  int iVar7;
  tagRECT tStack_10;
  
  pHVar1 = *(HWND *)(param_1 + 0x18);
  *(undefined4 *)(param_1 + 0xe0) = 0;
  pHVar2 = GetCapture();
  if (pHVar2 == pHVar1) {
    ReleaseCapture();
  }
  BVar3 = IsRectEmpty((RECT *)(param_1 + 0xbc));
  if (BVar3 == 0) {
    if ((*(byte *)(param_1 + 0xf4) & 1) == 0) {
      hdc = GetWindowDC(*(HWND *)(*(int *)(param_1 + 0x20) + 0x18));
      pvVar4 = GetStockObject(5);
      pvVar4 = SelectObject(hdc,pvVar4);
      pvVar5 = GetStockObject(6);
      pvVar5 = SelectObject(hdc,pvVar5);
      SetROP2(hdc,7);
      SetRect(&tStack_10,*(int *)(param_1 + 0xe4),*(int *)(param_1 + 0xe8),
              *(int *)(param_1 + 0x30) + *(int *)(param_1 + 0xe4),
              *(int *)(param_1 + 0x34) + *(int *)(param_1 + 0xe8));
      Rectangle(hdc,tStack_10.left,tStack_10.top,tStack_10.right,tStack_10.bottom);
      SelectObject(hdc,pvVar4);
      SelectObject(hdc,pvVar5);
      ReleaseDC(*(HWND *)(*(int *)(param_1 + 0x20) + 0x18),hdc);
      iVar6 = *(int *)(param_1 + 0xe4);
      iVar7 = *(int *)(param_1 + 0xe8);
      SetRectEmpty(&tStack_10);
    }
    else {
      GetWindowRect(*(HWND *)(param_1 + 0x18),&tStack_10);
      ScreenToClient(*(HWND *)(param_1 + 0xcc),(LPPOINT)&tStack_10);
      iVar6 = tStack_10.left;
      iVar7 = tStack_10.top;
    }
    X = *(int *)(param_1 + 0xbc);
    if (X <= iVar6) {
      X = iVar6;
      if (*(int *)(param_1 + 0xc4) < iVar6 + *(int *)(param_1 + 0x38)) {
        X = *(int *)(param_1 + 0xc4) - *(int *)(param_1 + 0x38);
      }
    }
    iVar6 = *(int *)(param_1 + 0xc0);
    if (iVar6 <= iVar7) {
      iVar6 = iVar7;
      if (*(int *)(param_1 + 200) < iVar7 + *(int *)(param_1 + 0x3c)) {
        iVar6 = *(int *)(param_1 + 200) - *(int *)(param_1 + 0x3c);
      }
    }
    if ((X != tStack_10.left) || (iVar6 != tStack_10.top)) {
      SetWindowPos(*(HWND *)(param_1 + 0x18),(HWND)0x0,X,iVar6,0,0,5);
    }
  }
  return 1;
}

