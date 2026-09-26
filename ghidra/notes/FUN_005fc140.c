
undefined4 __thiscall
FUN_005fc140(void *this,HDC param_1,char param_2,DWORD param_3,int param_4,int param_5,int param_6,
            int param_7,DWORD param_8,DWORD param_9)

{
  BITMAPINFOHEADER *pbmi;
  int iVar1;
  int iVar2;
  DWORD w;
  DWORD h;
  HDC hdc;
  uint uVar3;
  HPALETTE pHVar4;
  HGDIOBJ pvVar5;
  HPALETTE hPal;
  HBITMAP h_00;
  undefined1 auStack_18 [24];
  
  iVar2 = param_7;
  iVar1 = param_6;
  h_00 = (HBITMAP)0x0;
  if (*(int *)this == 0) {
    return 0;
  }
  w = FUN_005fc0e0(this);
  h = FUN_005fc0f0(this);
  if (param_8 == 0) {
    param_8 = w;
  }
  if (param_9 == 0) {
    param_9 = h;
  }
  if (param_6 < 0) {
    param_6 = 0;
    param_8 = param_8 + iVar1;
    param_4 = param_4 - iVar1;
  }
  iVar1 = param_4;
  if (param_7 < 0) {
    param_7 = 0;
    param_9 = param_9 + iVar2;
    param_5 = param_5 - iVar2;
  }
  iVar2 = param_5;
  if (param_4 < 0) {
    param_4 = 0;
    param_8 = param_8 + iVar1;
    param_6 = param_6 - iVar1;
  }
  if (param_5 < 0) {
    param_5 = 0;
    param_9 = param_9 + iVar2;
    param_7 = param_7 - iVar2;
  }
  if (param_2 != '\x10') {
    if (param_2 != '@') {
      SetStretchBltMode(param_1,3);
      SetDIBitsToDevice(param_1,param_8,param_9,w,h,param_6,param_7,0,h,*(void **)((int)this + 4),
                        *(BITMAPINFO **)this,0);
      return 1;
    }
    hdc = CreateCompatibleDC(param_1);
    uVar3 = FUN_005fc100(this);
    if ((uVar3 == 8) && (*(int *)((int)this + 0x20) == 0)) {
      pHVar4 = FUN_005fc3f0(this);
      *(HPALETTE *)((int)this + 0x20) = pHVar4;
    }
    if (*(char *)((int)this + 0xc) == '\x10') {
      pvVar5 = (HGDIOBJ)FUN_005fc130((int)this);
      pvVar5 = SelectObject(hdc,pvVar5);
      pHVar4 = SelectPalette(hdc,*(HPALETTE *)((int)this + 0x20),0);
      BitBlt(param_1,param_4,param_5,param_8,param_9,hdc,param_6,param_7,param_3);
      SelectPalette(hdc,pHVar4,0);
      SelectObject(hdc,pvVar5);
    }
    else {
      pbmi = *(BITMAPINFOHEADER **)this;
      if (pbmi != (BITMAPINFOHEADER *)0x0) {
        h_00 = CreateDIBitmap(param_1,pbmi,4,*(void **)((int)this + 4),(BITMAPINFO *)pbmi,0);
      }
      pvVar5 = (HGDIOBJ)0x0;
      if (h_00 != (HBITMAP)0x0) {
        GetObjectA(h_00,0x18,auStack_18);
        pvVar5 = SelectObject(hdc,h_00);
        pHVar4 = SelectPalette(hdc,*(HPALETTE *)((int)this + 0x20),0);
        if (DAT_006be4c0 == (HPALETTE)0x0) {
          hPal = (HPALETTE)0x0;
        }
        else {
          hPal = SelectPalette(param_1,DAT_006be4c0,0);
          RealizePalette(param_1);
        }
        BitBlt(param_1,param_4,param_5,param_8,param_9,hdc,param_6,param_7,param_3);
        if (DAT_006be4c0 != (HPALETTE)0x0) {
          SelectPalette(param_1,hPal,0);
        }
        SelectPalette(hdc,pHVar4,0);
        pvVar5 = SelectObject(hdc,pvVar5);
      }
      DeleteObject(pvVar5);
    }
    DeleteDC(hdc);
  }
  return 1;
}

