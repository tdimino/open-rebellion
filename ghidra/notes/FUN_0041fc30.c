
void __fastcall FUN_0041fc30(void *param_1)

{
  HGDIOBJ pvVar1;
  tagSIZE tStack_8;
  
  if (*(int *)((int)param_1 + 0x18) != 0) {
    *(uint *)((int)param_1 + 0x138) = *(uint *)((int)param_1 + 0x138) | 8;
    FUN_005f58b0((void *)((int)param_1 + 0x160));
    *(undefined4 *)((int)param_1 + 0x170) = 0;
    *(uint *)((int)param_1 + 0x13c) = 0;
    *(undefined4 *)((int)param_1 + 0x174) = 0;
    FUN_00420580(param_1,(uint *)((int)param_1 + 0x13c));
    SetCaretPos(*(int *)((int)param_1 + 0x140),*(int *)((int)param_1 + 0x144));
    if (*(HGDIOBJ *)((int)param_1 + 0xb0) != (HGDIOBJ)0x0) {
      pvVar1 = SelectObject(*(HDC *)((int)param_1 + 0x11c),*(HGDIOBJ *)((int)param_1 + 0xb0));
      if (*(int *)((int)param_1 + 0x120) == 0) {
        *(HGDIOBJ *)((int)param_1 + 0x120) = pvVar1;
      }
    }
    GetTextExtentPoint32A(*(HDC *)((int)param_1 + 0x11c),(LPCSTR)&lpString_006a7f40,1,&tStack_8);
    *(LONG *)((int)param_1 + 0x128) = tStack_8.cx;
    FUN_0041fd00((int)param_1);
    InvalidateRect(*(HWND *)((int)param_1 + 0x18),(RECT *)0x0,0);
  }
  return;
}

