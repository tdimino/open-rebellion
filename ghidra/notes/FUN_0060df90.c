
undefined4 __thiscall FUN_0060df90(void *this,HDC param_1,undefined4 *param_2)

{
  HDC hdc;
  HGDIOBJ pvVar1;
  HPALETTE hPal;
  HPALETTE hPal_00;
  int cy;
  int cx;
  int x1;
  int y1;
  DWORD rop;
  
  if (*(int *)((int)this + 8) == 0) {
    return 0;
  }
  hdc = CreateCompatibleDC(param_1);
  pvVar1 = (HGDIOBJ)FUN_005fc130(*(int *)((int)this + 8));
  pvVar1 = SelectObject(hdc,pvVar1);
  hPal = FUN_005fc3f0(*(int **)((int)this + 8));
  hPal_00 = SelectPalette(hdc,hPal,0);
  rop = 0xcc0020;
  y1 = 0;
  x1 = 0;
  cy = FUN_005fc0f0(*(int **)((int)this + 8));
  cx = FUN_005fc0e0(*(int **)((int)this + 8));
  BitBlt(hdc,0,0,cx,cy,param_1,x1,y1,rop);
  SelectPalette(hdc,hPal_00,0);
  SelectObject(hdc,pvVar1);
  DeleteDC(hdc);
  DeleteObject(hPal);
  *(undefined4 *)((int)this + 0x18) = *param_2;
  *(undefined4 *)((int)this + 0x1c) = param_2[1];
  return 1;
}

