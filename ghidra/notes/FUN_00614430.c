
undefined4 FUN_00614430(HWND param_1,int param_2,int param_3)

{
  uint uVar1;
  DWORD dwExStyle;
  HMENU pHVar2;
  DWORD dwStyle;
  tagRECT tStack_20;
  int iStack_10;
  int iStack_c;
  
  uVar1 = GetWindowLongA(param_1,-0x10);
  SetWindowLongA(param_1,-0x10,uVar1 & 0x7f3dffff | 0xc20000);
  SetRect(&tStack_20,0,0,param_2,param_3);
  dwExStyle = GetWindowLongA(param_1,-0x14);
  pHVar2 = GetMenu(param_1);
  uVar1 = (uint)(pHVar2 != (HMENU)0x0);
  dwStyle = GetWindowLongA(param_1,-0x10);
  AdjustWindowRectEx(&tStack_20,dwStyle,uVar1,dwExStyle);
  SetWindowPos(param_1,(HWND)0x0,0,0,tStack_20.right - tStack_20.left,
               tStack_20.bottom - tStack_20.top,0x16);
  SetWindowPos(param_1,(HWND)0xfffffffe,0,0,0,0,0x13);
  SystemParametersInfoA(0x30,0,&iStack_10,0);
  GetWindowRect(param_1,&tStack_20);
  if (tStack_20.left < iStack_10) {
    tStack_20.left = iStack_10;
  }
  if (tStack_20.top < iStack_c) {
    tStack_20.top = iStack_c;
  }
  SetWindowPos(param_1,(HWND)0x0,tStack_20.left,tStack_20.top,0,0,0x15);
  GetWindowRect(param_1,(LPRECT)&lpRect_006be6b8);
  return 0;
}

