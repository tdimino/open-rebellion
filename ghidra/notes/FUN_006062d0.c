
int __thiscall FUN_006062d0(void *this,int param_1)

{
  LPCSTR lpchText;
  HDC hdc;
  int iVar1;
  int cchText;
  tagRECT tStack_10;
  
  iVar1 = *(int *)((int)this + 0xd4) + param_1;
  lpchText = (LPCSTR)FUN_00583c40((int)this + 0x98);
  cchText = 0;
  hdc = GetDC(*(HWND *)((int)this + 0x18));
  tStack_10.left = 0;
  tStack_10.top = 0;
  tStack_10.right = 0;
  tStack_10.bottom = 0;
  FUN_006002c0(this,hdc);
  if (0 < iVar1) {
    do {
      if (cchText < 0) goto LAB_00606358;
      cchText = cchText + 1;
      DrawTextA(hdc,lpchText,cchText,&tStack_10,0xc24);
      if (*(int *)((int)this + 0xd0) < cchText) {
        cchText = *(int *)((int)this + 0xd0) + 1;
        iVar1 = tStack_10.right;
      }
    } while (tStack_10.right < iVar1);
  }
  if (0 < cchText) {
    cchText = cchText + -1;
  }
LAB_00606358:
  FUN_006002f0(this,hdc);
  ReleaseDC(*(HWND *)((int)this + 0x18),hdc);
  return cchText;
}

