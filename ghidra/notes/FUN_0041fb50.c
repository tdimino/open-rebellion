
void __thiscall FUN_0041fb50(void *this,int param_1)

{
  HWND hWnd;
  BOOL BVar1;
  tagTEXTMETRICA local_38;
  
  if ((~*(uint *)((int)this + 0x138) & 1) != (uint)(param_1 == 0)) {
    if (param_1 == 0) {
      HideCaret(*(HWND *)((int)this + 0x18));
      *(uint *)((int)this + 0x138) = *(uint *)((int)this + 0x138) & 0xfffffffe;
    }
    else {
      hWnd = *(HWND *)((int)this + 0x18);
      *(uint *)((int)this + 0x138) = *(uint *)((int)this + 0x138) | 1;
      *(undefined4 *)((int)this + 0x140) = *(undefined4 *)((int)this + 300);
      *(undefined4 *)((int)this + 0x144) = 0;
      *(undefined4 *)((int)this + 0x13c) = 0;
      if (hWnd != (HWND)0x0) {
        if (*(HDC *)((int)this + 0x11c) != (HDC)0x0) {
          GetTextMetricsA(*(HDC *)((int)this + 0x11c),&local_38);
          BVar1 = CreateCaret(hWnd,(HBITMAP)0x0,2,local_38.tmHeight);
          *(BOOL *)((int)this + 0x118) = BVar1;
          SetCaretPos(*(int *)((int)this + 0x140),*(int *)((int)this + 0x144));
          ShowCaret(hWnd);
        }
        SetFocus(hWnd);
        return;
      }
    }
  }
  return;
}

