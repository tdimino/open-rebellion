
void __fastcall FUN_006060f0(void *param_1)

{
  LONG LVar1;
  ushort uVar2;
  HDC hdc;
  LPCSTR lpchText;
  LONG LVar3;
  int iVar4;
  int iStack_14;
  tagRECT tStack_10;
  
  hdc = GetDC(*(HWND *)((int)param_1 + 0x18));
  iVar4 = (int)param_1 + 0x98;
  lpchText = (LPCSTR)FUN_00583c40(iVar4);
  iStack_14 = *(int *)((int)param_1 + 0xd4);
  tStack_10.left = 0;
  tStack_10.top = 0;
  tStack_10.right = 0;
  tStack_10.bottom = 0;
  if (lpchText != (LPCSTR)0x0) {
    FUN_006002c0(param_1,hdc);
    uVar2 = FUN_005f3040(iVar4);
    if ((int)(uint)uVar2 < *(int *)((int)param_1 + 0xcc)) {
      uVar2 = FUN_005f3040(iVar4);
      *(uint *)((int)param_1 + 0xcc) = (uint)uVar2;
    }
    DrawTextA(hdc,lpchText,*(int *)((int)param_1 + 0xcc),&tStack_10,0xc24);
    LVar1 = tStack_10.right;
    iVar4 = *(int *)((int)param_1 + 0xb8);
    if (*(int *)((int)param_1 + 0xb4) != iVar4) {
      if (iVar4 < 0) {
        iVar4 = FUN_005fc0e0(*(int **)((int)param_1 + 0xac));
        LVar3 = iVar4 - *(int *)((int)param_1 + 0xe0);
      }
      else {
        DrawTextA(hdc,lpchText,iVar4,&tStack_10,0xc24);
        LVar3 = tStack_10.right;
      }
      *(LONG *)((int)param_1 + 0xc0) = LVar3;
      iVar4 = *(int *)((int)param_1 + 0xb4);
      if (iVar4 < 1) {
        if (iVar4 == -1) {
          iVar4 = FUN_005fc0e0(*(int **)((int)param_1 + 0xac));
          *(int *)((int)param_1 + 0xbc) = iVar4 - *(int *)((int)param_1 + 0xe0);
        }
        else {
          *(undefined4 *)((int)param_1 + 0xbc) = 0;
        }
      }
      else {
        DrawTextA(hdc,lpchText,iVar4,&tStack_10,0xc24);
        *(LONG *)((int)param_1 + 0xbc) = tStack_10.right;
      }
      iVar4 = *(int *)((int)param_1 + 0xbc);
      if (*(int *)((int)param_1 + 0xc0) < iVar4) {
        *(int *)((int)param_1 + 0xbc) = *(int *)((int)param_1 + 0xc0);
        *(int *)((int)param_1 + 0xc0) = iVar4;
      }
    }
    FUN_006002f0(param_1,hdc);
    iVar4 = LVar1 - *(int *)((int)param_1 + 0xd4);
    if (*(int *)((int)param_1 + 0x38) - *(int *)((int)param_1 + 0xe0) < iVar4) {
      iStack_14 = (*(int *)((int)param_1 + 0xe0) + LVar1) - *(int *)((int)param_1 + 0x38);
      iVar4 = LVar1 - iStack_14;
    }
    else if (iVar4 < 0) {
      iStack_14 = *(int *)((int)param_1 + 0xd4) + iVar4;
      iVar4 = LVar1 - iStack_14;
    }
    if ((*(uint *)((int)param_1 + 0x94) & 0x10000000) != 0) {
      SetCaretPos(iVar4,0);
    }
  }
  if (iStack_14 != *(int *)((int)param_1 + 0xd4)) {
    *(int *)((int)param_1 + 0xd4) = iStack_14;
    if (*(HWND *)((int)param_1 + 0x18) != (HWND)0x0) {
      InvalidateRect(*(HWND *)((int)param_1 + 0x18),(RECT *)0x0,0);
    }
  }
  ReleaseDC(*(HWND *)((int)param_1 + 0x18),hdc);
  return;
}

