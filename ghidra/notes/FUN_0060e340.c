
void __thiscall FUN_0060e340(void *this,COLORREF param_1)

{
  HBRUSH pHVar1;
  LOGBRUSH local_c;
  
  if (param_1 != *(COLORREF *)((int)this + 0xa4)) {
    local_c.lbColor = param_1;
    *(COLORREF *)((int)this + 0xa4) = param_1;
    local_c.lbStyle = 0;
    local_c.lbHatch = 0;
    if (*(HGDIOBJ *)((int)this + 0xb0) != (HGDIOBJ)0x0) {
      DeleteObject(*(HGDIOBJ *)((int)this + 0xb0));
    }
    pHVar1 = CreateBrushIndirect(&local_c);
    *(HBRUSH *)((int)this + 0xb0) = pHVar1;
    if (*(HWND *)((int)this + 0x18) != (HWND)0x0) {
      InvalidateRect(*(HWND *)((int)this + 0x18),(RECT *)0x0,0);
    }
  }
  return;
}

