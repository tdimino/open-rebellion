
void __fastcall FUN_0041fd00(int param_1)

{
  LPRECT lprc;
  int iVar1;
  void *this;
  LPCSTR pCVar2;
  int iVar3;
  int cchText;
  LPRECT ptVar4;
  tagRECT *lprc_00;
  UINT UVar5;
  tagRECT tStack_10;
  
  GetTextMetricsA(*(HDC *)(param_1 + 0x11c),(LPTEXTMETRICA)(param_1 + 200));
  lprc = (LPRECT)(param_1 + 0xb8);
  SetRect(lprc,0,0,*(int *)(param_1 + 0x38),*(int *)(param_1 + 0x3c));
  iVar1 = param_1 + 0xa0;
  UVar5 = 0x2410;
  iVar3 = -1;
  ptVar4 = lprc;
  pCVar2 = (LPCSTR)FUN_00583c40(iVar1);
  DrawTextA(*(HDC *)(param_1 + 0x11c),pCVar2,iVar3,ptVar4,UVar5);
  *(undefined4 *)(param_1 + 0x130) = *(undefined4 *)(param_1 + 0x38);
  if (*(int *)(param_1 + 0xc4) <= *(int *)(param_1 + 0x3c)) {
    FUN_0060a650(param_1 + 0x94);
    return;
  }
  if (*(int **)(param_1 + 0x98) != (int *)0x0) {
    FUN_0060b4b0(*(int **)(param_1 + 0x98));
    FUN_0060a630(param_1 + 0x94);
  }
  UVar5 = 0x2410;
  cchText = -1;
  iVar3 = *(int *)(param_1 + 0x38) - *(int *)(*(int *)(param_1 + 0x98) + 0x30);
  *(int *)(param_1 + 0x130) = iVar3;
  *(int *)(param_1 + 0xc0) = iVar3;
  ptVar4 = lprc;
  pCVar2 = (LPCSTR)FUN_00583c40(iVar1);
  DrawTextA(*(HDC *)(param_1 + 0x11c),pCVar2,cchText,ptVar4,UVar5);
  this = (void *)(param_1 + 0x94);
  FUN_0060a670(this,*(int *)(param_1 + 0xc4) - *(int *)(param_1 + 0xbc));
  tStack_10.right = *(LONG *)(param_1 + 0xc0);
  tStack_10.left = lprc->left;
  tStack_10.top = *(LONG *)(param_1 + 0xbc);
  iVar3 = *(int *)(param_1 + 0x174);
  tStack_10.bottom = *(int *)(param_1 + 0xc4);
  if (iVar3 != 0) {
    lprc_00 = &tStack_10;
    UVar5 = 0x2410;
    pCVar2 = (LPCSTR)FUN_00583c40(iVar1);
    DrawTextA(*(HDC *)(param_1 + 0x11c),pCVar2,iVar3,lprc_00,UVar5);
    FUN_0060a690(this,tStack_10.bottom,tStack_10.bottom + *(int *)(param_1 + 0x3c));
    return;
  }
  FUN_0060a690(this,0,*(int *)(param_1 + 0x3c));
  return;
}

