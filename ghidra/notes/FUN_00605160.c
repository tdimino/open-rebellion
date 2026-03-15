
LRESULT FUN_00605160(void *param_1,uint param_2,uint param_3,uint param_4)

{
  size_t sVar1;
  bool bVar2;
  ushort uVar3;
  uint uVar4;
  HWND pHVar5;
  BOOL BVar6;
  int iVar7;
  HDC hdc;
  HDC hdc_00;
  HGDIOBJ h;
  LPCSTR pCVar8;
  HPALETTE hPal;
  uint uVar9;
  LRESULT LVar10;
  void *pvVar11;
  tagRECT *ptVar12;
  COLORREF CVar13;
  int iVar14;
  COLORREF CVar15;
  HWND pHVar16;
  uint uVar17;
  UINT UVar18;
  undefined4 *puVar19;
  WPARAM wParam;
  uint local_b8;
  undefined4 local_b0 [3];
  tagRECT local_a4;
  HGDIOBJ pvStack_94;
  uint local_90;
  HWND local_8c;
  HPALETTE pHStack_88;
  tagPAINTSTRUCT local_84;
  tagTEXTMETRICA tStack_44;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00656516;
  local_c = ExceptionList;
  pHVar16 = *(HWND *)((int)param_1 + 0x18);
  local_8c = pHVar16;
  if (param_2 < 7) {
    if (param_2 == 6) {
      uVar4 = param_3 & 0xffff;
      if (uVar4 == 0) {
        if ((*(uint *)((int)param_1 + 0x94) & 0x10000000) != 0) {
          ExceptionList = &local_c;
          DestroyCaret();
          HideCaret(pHVar16);
          *(uint *)((int)param_1 + 0x94) = *(uint *)((int)param_1 + 0x94) & 0xefffffff;
          ExceptionList = local_c;
          return 0;
        }
      }
      else if ((uVar4 != 0) && (uVar4 < 3)) {
        ExceptionList = &local_c;
        SetFocus(pHVar16);
        ExceptionList = local_c;
        return 0;
      }
      return 0;
    }
    if (param_2 == 1) {
      ExceptionList = &local_c;
      FUN_00605fc0(param_1);
      goto LAB_00605ea8;
    }
    goto LAB_00605cd6;
  }
  if (param_2 < 0x10) {
    if (param_2 != 0xf) {
      if (param_2 == 7) {
        ExceptionList = &local_c;
        pHVar5 = GetFocus();
        if (pHVar5 != pHVar16) {
          ExceptionList = local_c;
          return 0;
        }
        *(uint *)((int)param_1 + 0x94) = *(uint *)((int)param_1 + 0x94) | 0x10000000;
        CreateCaret(pHVar16,(HBITMAP)0x0,2,*(int *)((int)param_1 + 0x3c));
        BVar6 = ShowCaret(pHVar16);
        if (BVar6 == 0) {
          GetLastError();
        }
        FUN_006060f0(param_1);
        if (*(int *)((int)param_1 + 0xb4) < 0) {
          ExceptionList = local_c;
          return 0;
        }
        InvalidateRect(*(HWND *)((int)param_1 + 0x18),(RECT *)0x0,0);
        ExceptionList = local_c;
        return 0;
      }
      if (param_2 == 8) {
        if ((*(uint *)((int)param_1 + 0x94) & 0x10000000) != 0) {
          ExceptionList = &local_c;
          *(uint *)((int)param_1 + 0x94) = *(uint *)((int)param_1 + 0x94) & 0xeffeffff;
          DestroyCaret();
          HideCaret(pHVar16);
          InvalidateRect(*(HWND *)((int)param_1 + 0x18),(RECT *)0x0,0);
          ExceptionList = local_c;
          return 0;
        }
        return 0;
      }
      goto LAB_00605cd6;
    }
    ExceptionList = &local_c;
    local_90 = FUN_005f3070((int)param_1 + 0x98);
    local_90 = local_90 & 0xffff;
    uVar4 = ((int)*(uint *)((int)param_1 + 0xb4) < 0) - 1 & *(uint *)((int)param_1 + 0xb4);
    local_b8 = local_90;
    if (-1 < (int)*(uint *)((int)param_1 + 0xb8)) {
      local_b8 = *(uint *)((int)param_1 + 0xb8);
    }
    uVar9 = uVar4;
    if ((int)local_b8 < (int)uVar4) {
      uVar9 = local_b8;
      local_b8 = uVar4;
    }
    iVar14 = *(int *)((int)param_1 + 0xe0);
    local_a4.left = 0;
    local_a4.top = 0;
    iVar7 = FUN_005fc0e0(*(int **)((int)param_1 + 0xac));
    local_a4.right = iVar7 - iVar14;
    local_a4.bottom = *(int *)((int)param_1 + 0x3c) - iVar14;
    hdc = BeginPaint(pHVar16,&local_84);
    hdc_00 = CreateCompatibleDC(hdc);
    FUN_006002c0(param_1,hdc_00);
    h = (HGDIOBJ)FUN_005fc130(*(int *)((int)param_1 + 0xac));
    pvStack_94 = SelectObject(hdc_00,h);
    pHStack_88 = SelectPalette(hdc_00,DAT_006be5a0,0);
    GetTextMetricsA(hdc_00,&tStack_44);
    if ((*(uint *)((int)param_1 + 0x94) & 1) == 0) {
      pvVar11 = *(void **)((int)param_1 + 0xb0);
      if ((pvVar11 == (void *)0x0) || ((*(uint *)((int)param_1 + 0x94) & 4) == 0))
      goto LAB_0060540c;
      uVar4 = *(uint *)((int)param_1 + 0xdc);
      puVar19 = *(undefined4 **)((int)param_1 + 0xd8);
      uVar17 = 0;
      iVar14 = 0;
LAB_006053f2:
      FUN_005fcc30(pvVar11,*(uint **)((int)param_1 + 0xac),*(int *)((int)param_1 + 0xd4),0,iVar14,
                   uVar17,puVar19,uVar4);
    }
    else {
      pvVar11 = *(void **)((int)param_1 + 0xa8);
      if (pvVar11 != (void *)0x0) {
        uVar4 = *(uint *)((int)param_1 + 0xdc);
        puVar19 = *(undefined4 **)((int)param_1 + 0xd8);
        uVar17 = *(uint *)((int)param_1 + 200);
        iVar14 = *(int *)((int)param_1 + 0xc4);
        goto LAB_006053f2;
      }
    }
    SetBkMode(hdc_00,1);
LAB_0060540c:
    if (((*(uint *)((int)param_1 + 0x94) & 0x10000000) == 0) ||
       (*(int *)((int)param_1 + 0xb4) == *(int *)((int)param_1 + 0xb8))) {
      iVar14 = *(int *)((int)param_1 + 0xe0);
      CVar15 = *(COLORREF *)((int)param_1 + 0xe8);
      CVar13 = *(COLORREF *)((int)param_1 + 0xe4);
      iVar7 = -1;
      UVar18 = 0x824;
      ptVar12 = &local_a4;
      pCVar8 = (LPCSTR)FUN_00583c40((int)param_1 + 0x98);
      FUN_00600fd0(hdc_00,pCVar8,ptVar12,CVar13,CVar15,UVar18,iVar14,iVar7);
    }
    else {
      FUN_005f3de0(local_b0,(int)param_1 + 0x98);
      local_4 = 1;
      FUN_005f3f90((int)local_b0);
      if ((uVar9 != local_b8) && (0 < (int)uVar9)) {
        local_a4.right = *(LONG *)((int)param_1 + 0xbc);
        iVar14 = *(int *)((int)param_1 + 0xe0);
        CVar15 = *(COLORREF *)((int)param_1 + 0xe8);
        CVar13 = *(COLORREF *)((int)param_1 + 0xe4);
        UVar18 = 0x824;
        ptVar12 = &local_a4;
        local_a4.left = 0;
        uVar4 = uVar9;
        pCVar8 = (LPCSTR)FUN_00583c40((int)param_1 + 0x98);
        FUN_00600fd0(hdc_00,pCVar8,ptVar12,CVar13,CVar15,UVar18,iVar14,uVar4);
      }
      local_a4.right = *(int *)((int)param_1 + 0xc0);
      local_a4.left = *(LONG *)((int)param_1 + 0xbc);
      uVar4 = uVar9;
      if (0 < (int)uVar9) {
        do {
          FUN_005f3e50((int)local_b0);
          uVar4 = uVar4 - 1;
        } while (uVar4 != 0);
      }
      iVar7 = local_b8 - uVar9;
      SetBkMode(hdc_00,2);
      SetBkColor(hdc_00,*(COLORREF *)((int)param_1 + 0xe4));
      SetTextColor(hdc_00,*(COLORREF *)((int)param_1 + 0xe8));
      ptVar12 = &local_a4;
      UVar18 = 0x824;
      iVar14 = iVar7;
      pCVar8 = (LPCSTR)FUN_00583c40((int)local_b0);
      DrawTextA(hdc_00,pCVar8,iVar14,ptVar12,UVar18);
      SetBkMode(hdc_00,1);
      if ((int)local_b8 < (int)local_90) {
        if (0 < iVar7) {
          do {
            FUN_005f3e50((int)local_b0);
            iVar7 = iVar7 + -1;
          } while (iVar7 != 0);
        }
        local_a4.left = *(LONG *)((int)param_1 + 0xc0);
        iVar14 = *(int *)((int)param_1 + 0xe0);
        iVar7 = FUN_005fc0e0(*(int **)((int)param_1 + 0xac));
        CVar15 = *(COLORREF *)((int)param_1 + 0xe8);
        local_a4.right = iVar7 - iVar14;
        iVar7 = -1;
        CVar13 = *(COLORREF *)((int)param_1 + 0xe4);
        UVar18 = 0x24;
        ptVar12 = &local_a4;
        pCVar8 = (LPCSTR)FUN_00583c40((int)local_b0);
        FUN_00600fd0(hdc_00,pCVar8,ptVar12,CVar13,CVar15,UVar18,iVar14,iVar7);
      }
      local_4 = 0xffffffff;
      FUN_005f3e00(local_b0);
    }
    hPal = SelectPalette(hdc,DAT_006be5a0,0);
    BitBlt(hdc,0,0,*(int *)((int)param_1 + 0xd8),*(int *)((int)param_1 + 0xdc),hdc_00,
           *(int *)((int)param_1 + 0xd4),0,0xcc0020);
    SelectPalette(hdc,hPal,0);
    SelectPalette(hdc_00,pHStack_88,0);
    SelectObject(hdc_00,pvStack_94);
    DeleteDC(hdc_00);
    EndPaint(local_8c,&local_84);
    ExceptionList = local_c;
    return 0;
  }
  if (0x100 < param_2) {
    if (param_2 < 0x201) {
      if (param_2 == 0x200) {
        if ((*(uint *)((int)param_1 + 0x94) & 0x10000) != 0) {
          ExceptionList = &local_c;
          iVar14 = FUN_006062d0(param_1,(int)(short)param_4);
          if ((param_3 & 1) == 0) {
            *(uint *)((int)param_1 + 0x94) = *(uint *)((int)param_1 + 0x94) & 0xfffeffff;
            *(undefined4 *)((int)param_1 + 0xb8) = 0xffffffff;
            *(undefined4 *)((int)param_1 + 0xb4) = 0xffffffff;
          }
          else {
            *(int *)((int)param_1 + 0xcc) = iVar14;
            *(int *)((int)param_1 + 0xb8) = iVar14;
          }
          FUN_006060f0(param_1);
          InvalidateRect(*(HWND *)((int)param_1 + 0x18),(RECT *)0x0,0);
          ExceptionList = local_c;
          return 0;
        }
        return 0;
      }
      if (param_2 == 0x102) {
        ExceptionList = &local_c;
        if ((-1 < *(int *)((int)param_1 + 0xb4)) &&
           (ExceptionList = &local_c, *(int *)((int)param_1 + 0xb8) == -1)) {
          ExceptionList = &local_c;
          uVar4 = FUN_005f3070((int)param_1 + 0x98);
          *(uint *)((int)param_1 + 0xb8) = uVar4 & 0xffff;
        }
        uVar4 = FUN_0061a050(param_3);
        if (uVar4 == 0) {
          ExceptionList = local_c;
          return 0;
        }
        if ((*(uint *)((int)param_1 + 0x94) & 0x10000000) != 0) {
          if (*(int *)((int)param_1 + 0xb4) == *(int *)((int)param_1 + 0xb8)) {
            if (((*(uint *)((int)param_1 + 0x94) & 0x20000000) != 0) &&
               (sVar1 = *(size_t *)((int)param_1 + 0xcc), 0 < (int)sVar1)) {
              uVar4 = FUN_005f3070((int)param_1 + 0x98);
              if ((int)sVar1 < (int)(uVar4 & 0xffff)) {
                FUN_005f3c30((void *)((int)param_1 + 0x98),sVar1,sVar1 + 1);
              }
            }
          }
          else {
            FUN_00605f20((int)param_1);
          }
          if (*(int *)((int)param_1 + 0xcc) == -1) {
            uVar4 = FUN_005f3070((int)param_1 + 0x98);
            *(uint *)((int)param_1 + 0xcc) = uVar4 & 0xffff;
          }
          if ((*(int *)((int)param_1 + 0xa4) == 0) ||
             (*(int *)((int)param_1 + 0xcc) < *(int *)((int)param_1 + 0xa4))) {
            FUN_005f3ad0((void *)((int)param_1 + 0x98),(char *)&param_3,
                         *(size_t *)((int)param_1 + 0xcc));
            *(int *)((int)param_1 + 0xcc) = *(int *)((int)param_1 + 0xcc) + 1;
          }
          FUN_00605fc0(param_1);
          FUN_006060f0(param_1);
          InvalidateRect(*(HWND *)((int)param_1 + 0x18),(RECT *)0x0,0);
          PostMessageA(*(HWND *)(*(int *)((int)param_1 + 0x20) + 0x18),0x408,
                       *(WPARAM *)((int)param_1 + 0x24),0);
        }
        if ((*(uint *)((int)param_1 + 0x94) & 0x10000) != 0) {
          *(uint *)((int)param_1 + 0x94) = *(uint *)((int)param_1 + 0x94) & 0xfffeffff;
          ExceptionList = local_c;
          return 0;
        }
        ExceptionList = local_c;
        return 0;
      }
    }
    else {
      if (param_2 == 0x201) {
        uVar4 = param_4 & 0xffff;
        ExceptionList = &local_c;
        if (*(int *)((int)param_1 + 0xb4) != *(int *)((int)param_1 + 0xb8)) {
          ExceptionList = &local_c;
          *(undefined4 *)((int)param_1 + 0xb8) = 0xffffffff;
          *(undefined4 *)((int)param_1 + 0xb4) = 0xffffffff;
          InvalidateRect(pHVar16,(RECT *)0x0,0);
          UpdateWindow(*(HWND *)((int)param_1 + 0x18));
        }
        iVar14 = FUN_006062d0(param_1,uVar4);
        *(int *)((int)param_1 + 0xcc) = iVar14;
        *(int *)((int)param_1 + 0xb8) = iVar14;
        *(int *)((int)param_1 + 0xb4) = iVar14;
        *(uint *)((int)param_1 + 0x94) = *(uint *)((int)param_1 + 0x94) | 0x10000;
        FUN_00601080(*(HWND *)((int)param_1 + 0x18));
        SetFocus(*(HWND *)((int)param_1 + 0x18));
        goto LAB_00605ea8;
      }
      if (param_2 == 0x202) {
        ExceptionList = &local_c;
        *(uint *)((int)param_1 + 0x94) = *(uint *)((int)param_1 + 0x94) & 0xfffeffff;
        pHVar16 = *(HWND *)((int)param_1 + 0x18);
        pHVar5 = GetCapture();
        if (pHVar5 == pHVar16) {
          ReleaseCapture();
          ExceptionList = local_c;
          return 0;
        }
        ExceptionList = local_c;
        return 0;
      }
      if (param_2 == 0x203) {
        ExceptionList = &local_c;
        FUN_005f3de0(local_b0,(int)param_1 + 0x98);
        iVar14 = 0;
        bVar2 = true;
        local_4 = 0;
        *(undefined4 *)((int)param_1 + 0xb4) = 0;
        *(undefined4 *)((int)param_1 + 0xb8) = 0;
        do {
          iVar7 = FUN_005f3fa0((int)local_b0);
          if (iVar7 == 0) break;
          iVar7 = FUN_005f3f80((int)local_b0);
          uVar4 = FUN_00619ff0(iVar7);
          if ((uVar4 == 0) && (uVar4 = FUN_0061a020(iVar7), uVar4 == 0)) {
            if (bVar2) {
              if (iVar14 <= *(int *)((int)param_1 + 0xcc)) {
                *(int *)((int)param_1 + 0xb4) = iVar14;
              }
              bVar2 = false;
            }
          }
          else if (*(int *)((int)param_1 + 0xcc) < iVar14) {
            *(int *)((int)param_1 + 0xb8) = iVar14;
          }
          else {
            bVar2 = true;
          }
          iVar14 = iVar14 + 1;
          FUN_005f3e50((int)local_b0);
        } while (*(int *)((int)param_1 + 0xb8) == 0);
        iVar14 = *(int *)((int)param_1 + 0xb8);
        if (iVar14 == 0) {
          *(int *)((int)param_1 + 0xb8) = *(int *)((int)param_1 + 0xd0);
        }
        else if (iVar14 < *(int *)((int)param_1 + 0xd0)) {
          *(int *)((int)param_1 + 0xb8) = iVar14 + 1;
        }
        *(undefined4 *)((int)param_1 + 0xcc) = *(undefined4 *)((int)param_1 + 0xb8);
        FUN_006060f0(param_1);
        InvalidateRect(*(HWND *)((int)param_1 + 0x18),(RECT *)0x0,0);
        local_4 = 0xffffffff;
        FUN_005f3e00(local_b0);
        ExceptionList = local_c;
        return 0;
      }
    }
LAB_00605cd6:
    ExceptionList = &local_c;
    LVar10 = DefWindowProcA(pHVar16,param_2,param_3,param_4);
    ExceptionList = local_c;
    return LVar10;
  }
  if (param_2 != 0x100) {
    if (param_2 == 0x14) {
      ExceptionList = &local_c;
      LVar10 = DefWindowProcA(pHVar16,0x14,param_3,param_4);
      ExceptionList = local_c;
      return LVar10;
    }
    goto LAB_00605cd6;
  }
  ExceptionList = &local_c;
  if ((-1 < *(int *)((int)param_1 + 0xb4)) &&
     (ExceptionList = &local_c, *(int *)((int)param_1 + 0xb8) == -1)) {
    ExceptionList = &local_c;
    uVar4 = FUN_005f3070((int)param_1 + 0x98);
    *(uint *)((int)param_1 + 0xb8) = uVar4 & 0xffff;
  }
  switch(param_3) {
  case 8:
    if (*(int *)((int)param_1 + 0xb4) == *(int *)((int)param_1 + 0xb8)) {
      iVar14 = *(int *)((int)param_1 + 0xcc);
      if (0 < iVar14) {
        FUN_005f3c30((void *)((int)param_1 + 0x98),iVar14 - 1,iVar14);
        *(int *)((int)param_1 + 0xcc) = *(int *)((int)param_1 + 0xcc) + -1;
        goto LAB_00605706;
      }
      if (iVar14 == -1) {
        pvVar11 = (void *)((int)param_1 + 0x98);
        uVar4 = FUN_005f3070((int)pvVar11);
        uVar4 = uVar4 & 0xffff;
        uVar9 = FUN_005f3070((int)pvVar11);
        FUN_005f3c30(pvVar11,(uVar9 & 0xffff) - 1,uVar4);
        PostMessageA(*(HWND *)(*(int *)((int)param_1 + 0x20) + 0x18),0x408,
                     *(WPARAM *)((int)param_1 + 0x24),0);
      }
    }
    else {
      FUN_00605f20((int)param_1);
LAB_00605706:
      PostMessageA(*(HWND *)(*(int *)((int)param_1 + 0x20) + 0x18),0x408,
                   *(WPARAM *)((int)param_1 + 0x24),0);
    }
    InvalidateRect(*(HWND *)((int)param_1 + 0x18),(RECT *)0x0,0);
    FUN_00605fc0(param_1);
    break;
  default:
    LVar10 = (**(code **)(**(int **)((int)param_1 + 0x20) + 0x14))
                       (*(int **)((int)param_1 + 0x20),0x100,param_3,param_4);
    ExceptionList = local_c;
    return LVar10;
  case 0xd:
    PostMessageA(*(HWND *)(*(int *)((int)param_1 + 0x20) + 0x18),0x407,
                 *(WPARAM *)((int)param_1 + 0x24),(LPARAM)param_1);
    ExceptionList = local_c;
    return 0;
  case 0x23:
    uVar3 = GetKeyState(0x10);
    if ((uVar3 & 0xfffe) == 0) {
      if (-1 < *(int *)((int)param_1 + 0xb4)) {
        InvalidateRect(*(HWND *)((int)param_1 + 0x18),(RECT *)0x0,0);
      }
      *(undefined4 *)((int)param_1 + 0xb8) = 0xffffffff;
      *(undefined4 *)((int)param_1 + 0xb4) = 0xffffffff;
    }
    else if (*(int *)((int)param_1 + 0xcc) < *(int *)((int)param_1 + 0xd0)) {
      *(int *)((int)param_1 + 0xb4) = *(int *)((int)param_1 + 0xcc);
      *(int *)((int)param_1 + 0xb8) = *(int *)((int)param_1 + 0xd0);
      InvalidateRect(*(HWND *)((int)param_1 + 0x18),(RECT *)0x0,0);
      *(undefined4 *)((int)param_1 + 0xcc) = *(undefined4 *)((int)param_1 + 0xd0);
      break;
    }
    *(undefined4 *)((int)param_1 + 0xcc) = *(undefined4 *)((int)param_1 + 0xd0);
    break;
  case 0x24:
    uVar3 = GetKeyState(0x10);
    if ((uVar3 & 0xfffe) == 0) {
      if (-1 < *(int *)((int)param_1 + 0xb4)) {
        InvalidateRect(*(HWND *)((int)param_1 + 0x18),(RECT *)0x0,0);
      }
      *(undefined4 *)((int)param_1 + 0xb8) = 0xffffffff;
      *(undefined4 *)((int)param_1 + 0xb4) = 0xffffffff;
    }
    else if (0 < *(int *)((int)param_1 + 0xcc)) {
      *(int *)((int)param_1 + 0xb8) = *(int *)((int)param_1 + 0xcc);
      *(undefined4 *)((int)param_1 + 0xb4) = 0;
      InvalidateRect(*(HWND *)((int)param_1 + 0x18),(RECT *)0x0,0);
      *(undefined4 *)((int)param_1 + 0xcc) = 0;
      break;
    }
    *(undefined4 *)((int)param_1 + 0xcc) = 0;
    break;
  case 0x25:
    uVar4 = *(uint *)((int)param_1 + 0xcc);
    if ((int)uVar4 < 1) {
      if (uVar4 == 0xffffffff) {
        uVar4 = FUN_005f3070((int)param_1 + 0x98);
        uVar4 = uVar4 & 0xffff;
        goto LAB_0060590a;
      }
    }
    else {
LAB_0060590a:
      *(uint *)((int)param_1 + 0xcc) = uVar4 - 1;
    }
    uVar3 = GetKeyState(0x10);
    if ((uVar3 & 0xfffe) != 0) {
      iVar14 = *(int *)((int)param_1 + 0xb4);
      iVar7 = *(int *)((int)param_1 + 0xcc);
      if (iVar7 == iVar14 + -1) {
        *(int *)((int)param_1 + 0xb4) = iVar7;
        InvalidateRect(*(HWND *)((int)param_1 + 0x18),(RECT *)0x0,0);
      }
      else if (iVar7 == *(int *)((int)param_1 + 0xb8) + -1) {
        *(int *)((int)param_1 + 0xb8) = iVar7;
        if (iVar7 == iVar14) {
          *(undefined4 *)((int)param_1 + 0xb4) = 0xffffffff;
          *(undefined4 *)((int)param_1 + 0xb8) = 0xffffffff;
        }
        InvalidateRect(*(HWND *)((int)param_1 + 0x18),(RECT *)0x0,0);
      }
      else if (iVar14 == *(int *)((int)param_1 + 0xb8)) {
        *(int *)((int)param_1 + 0xb4) = iVar7;
        *(int *)((int)param_1 + 0xb8) = iVar7 + 1;
        InvalidateRect(*(HWND *)((int)param_1 + 0x18),(RECT *)0x0,0);
      }
      break;
    }
    if (*(int *)((int)param_1 + 0xb4) != -1) {
      pHVar16 = *(HWND *)((int)param_1 + 0x18);
LAB_00605a9f:
      InvalidateRect(pHVar16,(RECT *)0x0,0);
    }
    goto LAB_00605aa5;
  case 0x27:
    iVar14 = *(int *)((int)param_1 + 0xcc);
    if ((-1 < iVar14) && (uVar4 = FUN_005f3070((int)param_1 + 0x98), iVar14 < (int)(uVar4 & 0xffff))
       ) {
      *(int *)((int)param_1 + 0xcc) = iVar14 + 1;
    }
    uVar3 = GetKeyState(0x10);
    if ((uVar3 & 0xfffe) != 0) {
      iVar14 = *(int *)((int)param_1 + 0xb8);
      iVar7 = *(int *)((int)param_1 + 0xcc);
      if (iVar7 == iVar14 + 1) {
        *(int *)((int)param_1 + 0xb8) = iVar7;
        InvalidateRect(*(HWND *)((int)param_1 + 0x18),(RECT *)0x0,0);
      }
      else if (iVar7 == *(int *)((int)param_1 + 0xb4) + 1) {
        *(int *)((int)param_1 + 0xb4) = iVar7;
        if (iVar7 == iVar14) {
          *(undefined4 *)((int)param_1 + 0xb8) = 0xffffffff;
          *(undefined4 *)((int)param_1 + 0xb4) = 0xffffffff;
        }
        InvalidateRect(*(HWND *)((int)param_1 + 0x18),(RECT *)0x0,0);
      }
      else if (*(int *)((int)param_1 + 0xb4) == iVar14) {
        *(int *)((int)param_1 + 0xb8) = iVar7;
        *(int *)((int)param_1 + 0xb4) = iVar7 + -1;
        InvalidateRect(*(HWND *)((int)param_1 + 0x18),(RECT *)0x0,0);
      }
      break;
    }
    if (*(int *)((int)param_1 + 0xb4) != -1) {
      pHVar16 = *(HWND *)((int)param_1 + 0x18);
      goto LAB_00605a9f;
    }
LAB_00605aa5:
    *(undefined4 *)((int)param_1 + 0xb8) = 0xffffffff;
    *(undefined4 *)((int)param_1 + 0xb4) = 0xffffffff;
    break;
  case 0x2d:
    *(uint *)((int)param_1 + 0x94) = *(uint *)((int)param_1 + 0x94) ^ 0x20000000;
    ExceptionList = local_c;
    return 0;
  case 0x2e:
    if (*(int *)((int)param_1 + 0xb4) == *(int *)((int)param_1 + 0xb8)) {
      sVar1 = *(size_t *)((int)param_1 + 0xcc);
      if (-1 < (int)sVar1) {
        FUN_005f3c30((void *)((int)param_1 + 0x98),sVar1,sVar1 + 1);
        wParam = *(WPARAM *)((int)param_1 + 0x24);
        pHVar16 = *(HWND *)(*(int *)((int)param_1 + 0x20) + 0x18);
        goto LAB_006057d2;
      }
    }
    else {
      FUN_00605f20((int)param_1);
      wParam = *(WPARAM *)((int)param_1 + 0x24);
      pHVar16 = *(HWND *)(*(int *)((int)param_1 + 0x20) + 0x18);
LAB_006057d2:
      PostMessageA(pHVar16,0x408,wParam,0);
    }
    InvalidateRect(*(HWND *)((int)param_1 + 0x18),(RECT *)0x0,0);
    FUN_00605fc0(param_1);
  }
LAB_00605ea8:
  FUN_006060f0(param_1);
  ExceptionList = local_c;
  return 0;
}

