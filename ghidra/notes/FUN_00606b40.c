
bool __thiscall FUN_00606b40(void *this,UINT param_1)

{
  HDC hdc;
  HGDIOBJ h;
  uint in_stack_00000018;
  COLORREF in_stack_0000001c;
  UINT in_stack_00000020;
  CHAR aCStack_100 [256];
  
  hdc = *(HDC *)((int)this + 0xd0);
  if (hdc != (HDC)0x0) {
    SetTextColor(hdc,in_stack_0000001c);
    h = (HGDIOBJ)FUN_006002b0(in_stack_00000018);
    SelectObject(*(HDC *)((int)this + 0xd0),h);
    FUN_00601060(param_1,aCStack_100,0xff);
    DrawTextA(*(HDC *)((int)this + 0xd0),aCStack_100,-1,(LPRECT)&stack0x00000008,in_stack_00000020);
  }
  return hdc != (HDC)0x0;
}

