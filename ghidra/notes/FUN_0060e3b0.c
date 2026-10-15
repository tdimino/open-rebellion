
void __thiscall FUN_0060e3b0(void *this,COLORREF param_1)

{
  HPEN pHVar1;
  
  if (param_1 != *(COLORREF *)((int)this + 0xa8)) {
    if (*(HGDIOBJ *)((int)this + 0xb4) != (HGDIOBJ)0x0) {
      DeleteObject(*(HGDIOBJ *)((int)this + 0xb4));
    }
    pHVar1 = CreatePen(0,1,param_1);
    *(HPEN *)((int)this + 0xb4) = pHVar1;
    if (*(HWND *)((int)this + 0x18) != (HWND)0x0) {
      InvalidateRect(*(HWND *)((int)this + 0x18),(RECT *)0x0,0);
    }
  }
  return;
}

