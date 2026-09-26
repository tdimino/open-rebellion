
undefined4 __thiscall FUN_00606ee0(void *this,undefined4 param_1,undefined4 param_2)

{
  HWND pHVar1;
  HWND pHVar2;
  HDC hdc;
  HGDIOBJ pvVar3;
  HGDIOBJ pvVar4;
  tagRECT tStack_10;
  
  pHVar1 = *(HWND *)((int)this + 0x18);
  pHVar2 = GetCapture();
  if (((*(byte *)((int)this + 0xb8) & 4) != 0) && ((pHVar2 == (HWND)0x0 || (pHVar2 == pHVar1)))) {
    *(undefined4 *)((int)this + 0xe4) = param_1;
    *(undefined4 *)((int)this + 0xe0) = 1;
    *(undefined4 *)((int)this + 0xe8) = param_2;
    if ((*(byte *)((int)this + 0xf4) & 2) != 0) {
      *(undefined4 *)((int)this + 0xec) = param_1;
      *(undefined4 *)((int)this + 0xe4) = *(undefined4 *)((int)this + 0x28);
      *(undefined4 *)((int)this + 0xf0) = param_2;
      *(undefined4 *)((int)this + 0xe8) = *(undefined4 *)((int)this + 0x2c);
      hdc = GetWindowDC(*(HWND *)(*(int *)((int)this + 0x20) + 0x18));
      pvVar3 = GetStockObject(5);
      pvVar3 = SelectObject(hdc,pvVar3);
      pvVar4 = GetStockObject(6);
      pvVar4 = SelectObject(hdc,pvVar4);
      SetROP2(hdc,7);
      SetRect(&tStack_10,*(int *)((int)this + 0xe4),*(int *)((int)this + 0xe8),
              *(int *)((int)this + 0x30) + *(int *)((int)this + 0xe4),
              *(int *)((int)this + 0x34) + *(int *)((int)this + 0xe8));
      Rectangle(hdc,tStack_10.left,tStack_10.top,tStack_10.right,tStack_10.bottom);
      SelectObject(hdc,pvVar3);
      SelectObject(hdc,pvVar4);
      ReleaseDC(*(HWND *)(*(int *)((int)this + 0x20) + 0x18),hdc);
    }
    FUN_00601080(*(HWND *)((int)this + 0x18));
    return 1;
  }
  return 0;
}

