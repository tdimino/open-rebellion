
void __thiscall FUN_0041fe70(void *this,HDC param_1)

{
  bool bVar1;
  int iVar2;
  COLORREF color;
  LPCSTR pCVar3;
  uint uVar4;
  int iVar5;
  int Y;
  int iVar6;
  int c;
  tagSIZE *ptVar7;
  int iStack_34;
  HGDIOBJ local_2c;
  tagSIZE tStack_20;
  tagSIZE tStack_18;
  undefined4 auStack_c [3];
  
  Y = 0;
  iVar6 = *(int *)((int)this + 300);
  iVar2 = *(int *)((int)this + 0xdc) * 4;
  bVar1 = false;
  FUN_005f3e30((void *)((int)this + 0x10c),(int)this + 0xa0);
  FUN_005f3e30((void *)((int)this + 0x100),(int)this + 0xa0);
  local_2c = (HGDIOBJ)0x0;
  if (*(HGDIOBJ *)((int)this + 0xb0) != (HGDIOBJ)0x0) {
    local_2c = SelectObject(param_1,*(HGDIOBJ *)((int)this + 0xb0));
  }
  color = SetTextColor(param_1,*(COLORREF *)((int)this + 0xb4));
  SetBkMode(param_1,1);
  iVar5 = 0;
  if (0 < *(int *)((int)this + 0x174)) {
    do {
      FUN_005f3e70((void *)((int)this + 0x10c),&tStack_18);
      FUN_005f3e00(&tStack_18.cx);
      iVar5 = iVar5 + 1;
    } while (iVar5 < *(int *)((int)this + 0x174));
  }
  iStack_34 = *(int *)((int)this + 0x174);
  FUN_005f3e10((void *)((int)this + 0x100),(int)this + 0x10c);
  iVar5 = FUN_005f3fa0((int)this + 0x10c);
  while (iVar5 != 0) {
    iVar5 = FUN_00420930((int)this + 0x100,(void *)((int)this + 0x10c));
    if (iVar5 == 0) {
      iVar5 = FUN_005f3f80((int)this + 0x10c);
      if (iVar5 == 9) {
        iVar6 = iVar6 + 1 + (iVar2 - (iVar6 + 1) % iVar2);
        if (*(int *)((int)this + 0x130) < iVar6) goto LAB_00420128;
      }
      else if ((iVar5 == 10) ||
              ((iVar5 == 0x20 &&
               (iVar6 = iVar6 + *(int *)((int)this + 0x128), *(int *)((int)this + 0x130) < iVar6))))
      {
LAB_00420128:
        iVar6 = *(int *)((int)this + 300);
        Y = Y + *(int *)((int)this + 200);
      }
      iStack_34 = iStack_34 + 1;
      if ((!bVar1) && (*(int *)((int)this + 0x13c) <= iStack_34)) {
        *(int *)((int)this + 0x140) = iVar6;
        *(int *)((int)this + 0x144) = Y;
        SetCaretPos(iVar6,Y);
        bVar1 = true;
      }
    }
    else {
      ptVar7 = &tStack_20;
      c = iVar5;
      pCVar3 = (LPCSTR)FUN_00583c40((int)this + 0x100);
      GetTextExtentPoint32A(param_1,pCVar3,c,ptVar7);
      if (*(int *)((int)this + 0x130) < iVar6 + tStack_20.cx) {
        if (*(int *)((int)this + 0x130) - *(int *)((int)this + 300) < tStack_20.cx) {
          do {
            if (iVar5 < 2) break;
            iVar5 = iVar5 + -1;
            FUN_005f3ef0((void *)((int)this + 0x10c),auStack_c);
            FUN_005f3e00(auStack_c);
            ptVar7 = &tStack_20;
            iVar6 = iVar5;
            pCVar3 = (LPCSTR)FUN_00583c40((int)this + 0x100);
            GetTextExtentPoint32A(param_1,pCVar3,iVar6,ptVar7);
          } while (*(int *)((int)this + 0x130) - *(int *)((int)this + 300) < tStack_20.cx);
        }
        if ((iStack_34 == *(int *)((int)this + 0x13c)) && (bVar1)) {
          *(int *)((int)this + 0x140) = *(int *)((int)this + 300);
          iVar6 = *(int *)((int)this + 200) + Y;
          *(int *)((int)this + 0x144) = iVar6;
          SetCaretPos(*(int *)((int)this + 300),iVar6);
        }
        iVar6 = *(int *)((int)this + 300);
        Y = Y + *(int *)((int)this + 200);
      }
      iStack_34 = iStack_34 + iVar5;
      if ((!bVar1) && (*(int *)((int)this + 0x13c) <= iStack_34)) {
        uVar4 = *(int *)((int)this + 0x13c) + (iVar5 - iStack_34);
        ptVar7 = &tStack_18;
        uVar4 = uVar4 & ((int)uVar4 < 1) - 1;
        pCVar3 = (LPCSTR)FUN_00583c40((int)this + 0x100);
        GetTextExtentPoint32A(param_1,pCVar3,uVar4,ptVar7);
        *(int *)((int)this + 0x144) = Y;
        *(LONG *)((int)this + 0x140) = tStack_18.cx + iVar6;
        SetCaretPos(tStack_18.cx + iVar6,Y);
        bVar1 = true;
      }
      if (*(int *)((int)this + 0x3c) < *(int *)((int)this + 200) + Y) break;
      pCVar3 = (LPCSTR)FUN_00583c40((int)this + 0x100);
      TextOutA(param_1,iVar6,Y,pCVar3,iVar5);
      iVar6 = iVar6 + tStack_20.cx;
    }
    iVar5 = FUN_005f3e50((int)this + 0x10c);
    FUN_005f3e10((void *)((int)this + 0x100),iVar5);
    iVar5 = FUN_005f3fa0((int)this + 0x10c);
  }
  if (local_2c != (HGDIOBJ)0x0) {
    SelectObject(param_1,local_2c);
  }
  SetTextColor(param_1,color);
  return;
}

