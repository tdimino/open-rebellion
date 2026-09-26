
undefined4 __thiscall FUN_006071e0(void *this,int param_1,int param_2)

{
  undefined4 uVar1;
  HDC hdc;
  HGDIOBJ pvVar2;
  HGDIOBJ pvVar3;
  int xLeft;
  int yTop;
  tagRECT tStack_20;
  tagRECT tStack_10;
  
  uVar1 = 0;
  if (*(int *)((int)this + 0xe0) != 0) {
    if ((*(byte *)((int)this + 0xf4) & 1) != 0) {
      MapWindowPoints(*(HWND *)((int)this + 0x18),*(HWND *)(*(int *)((int)this + 0x20) + 0x18),
                      (LPPOINT)&param_1,1);
      SetWindowPos(*(HWND *)((int)this + 0x18),(HWND)0x0,param_1 - *(int *)((int)this + 0xe4),
                   param_2 - *(int *)((int)this + 0xe8),0,0,5);
      return 1;
    }
    if (*(int *)((int)this + 0x20) != 0) {
      hdc = GetWindowDC(*(HWND *)(*(int *)((int)this + 0x20) + 0x18));
      pvVar2 = GetStockObject(5);
      pvVar2 = SelectObject(hdc,pvVar2);
      pvVar3 = GetStockObject(6);
      pvVar3 = SelectObject(hdc,pvVar3);
      SetROP2(hdc,7);
      SetRect(&tStack_20,*(int *)((int)this + 0xe4),*(int *)((int)this + 0xe8),
              *(int *)((int)this + 0xe4) + *(int *)((int)this + 0x30),
              *(int *)((int)this + 0xe8) + *(int *)((int)this + 0x34));
      Rectangle(hdc,tStack_20.left,tStack_20.top,tStack_20.right,tStack_20.bottom);
      xLeft = param_1 + (*(int *)((int)this + 0x28) - *(int *)((int)this + 0xec));
      yTop = (*(int *)((int)this + 0x2c) - *(int *)((int)this + 0xf0)) + param_2;
      *(int *)((int)this + 0xe4) = xLeft;
      *(int *)((int)this + 0xe8) = yTop;
      SetRect(&tStack_10,xLeft,yTop,xLeft + *(int *)((int)this + 0x30),
              yTop + *(int *)((int)this + 0x34));
      Rectangle(hdc,tStack_10.left,tStack_10.top,tStack_10.right,tStack_10.bottom);
      SelectObject(hdc,pvVar2);
      SelectObject(hdc,pvVar3);
      ReleaseDC(*(HWND *)(*(int *)((int)this + 0x20) + 0x18),hdc);
    }
    uVar1 = 1;
  }
  return uVar1;
}

