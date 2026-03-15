
LRESULT FUN_0041efe0(int *param_1,uint param_2,HWND param_3,uint param_4)

{
  uint *puVar1;
  HWND hWnd;
  HDC pHVar2;
  HDC pHVar3;
  BOOL BVar4;
  HWND pHVar5;
  LPCSTR pCVar6;
  uint uVar7;
  LRESULT LVar8;
  int iVar9;
  uint uVar10;
  tagSIZE *ptVar11;
  tagSIZE local_e8;
  tagSIZE local_e0;
  LRESULT local_d8;
  tagRECT local_d4;
  tagRECT local_c4;
  undefined4 local_b4 [3];
  undefined4 local_a8 [3];
  undefined4 local_9c [3];
  undefined4 local_90 [3];
  tagTEXTMETRICA local_84;
  tagPAINTSTRUCT local_4c;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062b8eb;
  local_c = ExceptionList;
  hWnd = (HWND)param_1[6];
  local_d8 = 0;
  if (param_2 < 6) {
    if (param_2 == 5) {
      ExceptionList = &local_c;
      FUN_0041fc30(param_1);
      ExceptionList = local_c;
      return local_d8;
    }
    if (param_2 == 1) {
      ExceptionList = &local_c;
      FUN_0060a490(param_1 + 0x25,param_1[7],param_1,(short)param_1[0x4d],
                   *(undefined2 *)((int)param_1 + 0x136),param_1[0xf],0,param_1[0xf],0,(uint *)0x0);
      pHVar2 = GetDC(hWnd);
      pHVar3 = CreateCompatibleDC(pHVar2);
      param_1[0x47] = (int)pHVar3;
      ReleaseDC(hWnd,pHVar2);
      ExceptionList = local_c;
      return local_d8;
    }
LAB_0041f956:
    ExceptionList = &local_c;
    LVar8 = FUN_006007b0(param_1,param_2,(uint)param_3,param_4);
    ExceptionList = local_c;
    return LVar8;
  }
  if (param_2 < 0x10) {
    if (param_2 == 0xf) {
      ExceptionList = &local_c;
      pHVar2 = BeginPaint(hWnd,&local_4c);
      local_e8.cx = (LONG)SelectPalette(pHVar2,DAT_006be5a0,0);
      FUN_005fc140((void *)param_1[0x2b],pHVar2,'@',0xcc0020,0,0,0,0,0,0);
      FUN_0041fe70(param_1,pHVar2);
      SetCaretPos(param_1[0x50],param_1[0x51]);
      SelectPalette(pHVar2,(HPALETTE)local_e8.cx,0);
      EndPaint(hWnd,&local_4c);
      ExceptionList = local_c;
      return local_d8;
    }
    if (param_2 == 7) {
      if ((*(byte *)(param_1 + 0x4e) & 1) == 0) {
        return 0;
      }
      if (param_3 == hWnd) {
        return 0;
      }
      ExceptionList = &local_c;
      if ((HDC)param_1[0x47] != (HDC)0x0) {
        ExceptionList = &local_c;
        GetTextMetricsA((HDC)param_1[0x47],&local_84);
        BVar4 = CreateCaret((HWND)param_1[6],(HBITMAP)0x0,2,local_84.tmHeight);
        param_1[0x46] = BVar4;
      }
      SetCaretPos(param_1[0x50],param_1[0x51]);
      ShowCaret(hWnd);
      ExceptionList = local_c;
      return local_d8;
    }
    if (param_2 == 8) {
      ExceptionList = &local_c;
      DestroyCaret();
      param_1[0x46] = 0;
      ExceptionList = local_c;
      return local_d8;
    }
    goto LAB_0041f956;
  }
  if (param_2 < 0x15) {
    if (param_2 == 0x14) {
      return 1;
    }
    if (param_2 == 0x10) {
      return 0;
    }
    goto LAB_0041f956;
  }
  if (0x100 < param_2) {
    if (param_2 < 0x201) {
      if (param_2 == 0x200) {
        if ((param_1[0x4e] & 1U) == 0) {
          return 0;
        }
        if (((uint)param_3 & 1) == 0) {
          return 0;
        }
        local_e0.cx = param_4 & 0xffff;
        local_e0.cy = param_4 >> 0x10;
        ExceptionList = &local_c;
        if ((param_1[0x4e] & 4U) != 0) {
          ExceptionList = &local_c;
          GetCaretPos((LPPOINT)&local_e8);
          FUN_005ae1e0();
          HideCaret(hWnd);
          param_1[0x4e] = param_1[0x4e] & 0xfffffffb;
        }
        param_1[0x4e] = param_1[0x4e] | 2;
        FUN_005ae1e0();
        ExceptionList = local_c;
        return local_d8;
      }
      if (param_2 == 0x102) {
        if (param_3 == (HWND)0x9) {
          return 0;
        }
        ExceptionList = &local_c;
        uVar7 = FUN_0061a050((int)param_3);
        if (uVar7 == 0) {
          ExceptionList = local_c;
          return local_d8;
        }
        if ((param_1[0x4e] & 1U) == 0) {
          ExceptionList = local_c;
          return local_d8;
        }
        if ((param_1[0x4e] & 8U) == 0) {
          FUN_005f3c30(param_1 + 0x28,param_1[0x4f] - 1,param_1[0x4f]);
        }
        FUN_005f3ad0(param_1 + 0x28,(char *)&param_3,param_1[0x4f]);
        GetTextExtentPoint32A((HDC)param_1[0x47],(LPCSTR)&param_3,1,&local_e0);
        iVar9 = param_1[0x50];
        param_1[0x50] = iVar9 + local_e0.cx;
        SetCaretPos(iVar9 + local_e0.cx,param_1[0x51]);
        if (param_1[0xf] < param_1[0x51] + param_1[0x32]) {
          FUN_00420200(param_1,1);
          param_1[0x51] = param_1[0x51] - param_1[0x32];
        }
        param_1[0x4f] = param_1[0x4f] + 1;
        FUN_0041fd00((int)param_1);
        InvalidateRect(hWnd,(RECT *)0x0,0);
        ExceptionList = local_c;
        return local_d8;
      }
    }
    else {
      if (param_2 == 0x201) {
        if ((*(byte *)(param_1 + 0x4e) & 1) == 0) {
          return 0;
        }
        ExceptionList = &local_c;
        pHVar5 = GetFocus();
        if (pHVar5 != hWnd) {
          SetFocus(hWnd);
        }
        uVar7 = param_4 >> 0x10;
        uVar10 = param_4 & 0xffff;
        if ((*(byte *)(param_1 + 0x4e) & 2) != 0) {
          SetRect(&local_d4,param_1[0x53],param_1[0x54],param_1[0x56],param_1[0x57]);
          FUN_0041fb20(&local_d4.left);
          InvalidateRect(hWnd,&local_d4,0);
          UpdateWindow(hWnd);
          param_1[0x4e] = param_1[0x4e] & 0xfffffffd;
        }
        param_1[0x50] = uVar10;
        param_1[0x51] = uVar7;
        FUN_00420580(param_1,(uint *)(param_1 + 0x4f));
        SetCaretPos(param_1[0x50],param_1[0x51]);
        ShowCaret(hWnd);
        param_1[0x4e] = param_1[0x4e] | 4;
        FUN_00601080(hWnd);
        ExceptionList = local_c;
        return local_d8;
      }
      if (param_2 == 0x202) {
        if ((param_1[0x4e] & 1U) == 0) {
          return 0;
        }
        local_e0.cx = param_4 & 0xffff;
        local_e0.cy = param_4 >> 0x10;
        ExceptionList = &local_c;
        if ((param_1[0x4e] & 2U) != 0) {
          ExceptionList = &local_c;
          FUN_005ae1e0();
        }
        ReleaseCapture();
        ExceptionList = local_c;
        return local_d8;
      }
    }
    goto LAB_0041f956;
  }
  if (param_2 != 0x100) {
    if (param_2 == 0x18) {
      if ((param_3 == (HWND)0x0) || ((*(byte *)(param_1 + 0x4e) & 1) == 0)) {
        ExceptionList = &local_c;
        HideCaret(hWnd);
      }
      else {
        ExceptionList = &local_c;
        if ((HDC)param_1[0x47] != (HDC)0x0) {
          ExceptionList = &local_c;
          GetTextMetricsA((HDC)param_1[0x47],&local_84);
          BVar4 = CreateCaret((HWND)param_1[6],(HBITMAP)0x0,2,local_84.tmHeight);
          param_1[0x46] = BVar4;
          SetCaretPos(param_1[0x50],param_1[0x51]);
          ShowCaret(hWnd);
        }
        pHVar5 = GetFocus();
        if (pHVar5 != hWnd) {
          SetFocus(hWnd);
        }
      }
      LVar8 = FUN_006007b0(param_1,0x18,(uint)param_3,param_4);
      ExceptionList = local_c;
      return LVar8;
    }
    goto LAB_0041f956;
  }
  ExceptionList = &local_c;
  FUN_005f3de0(&local_d4,(int)(param_1 + 0x28));
  puVar1 = (uint *)(param_1 + 0x4f);
  iVar9 = 0;
  local_4 = 0;
  if (0 < param_1[0x4f]) {
    do {
      FUN_005f3e70(&local_d4,local_a8);
      FUN_005f3e00(local_a8);
      iVar9 = iVar9 + 1;
    } while (iVar9 < (int)*puVar1);
  }
  switch(param_3) {
  case (HWND)0x8:
    if ((*(byte *)(param_1 + 0x4e) & 1) != 0) {
      if (0 < (int)*puVar1) {
        FUN_005f3ef0(&local_d4,local_b4);
        FUN_005f3e00(local_b4);
        ptVar11 = &local_e8;
        local_e8.cx = 0;
        iVar9 = 1;
        local_e8.cy = 0;
        pCVar6 = (LPCSTR)FUN_00583c40((int)&local_d4);
        GetTextExtentPoint32A((HDC)param_1[0x47],pCVar6,iVar9,ptVar11);
        FUN_005f3c30(param_1 + 0x28,*puVar1 - 1,*puVar1);
        *puVar1 = *puVar1 - 1;
        iVar9 = param_1[0x50];
        param_1[0x50] = iVar9 - local_e8.cx;
        if (iVar9 - local_e8.cx < param_1[0x4b]) {
          param_1[0x50] = param_1[0x4b];
        }
        SetCaretPos(param_1[0x50],param_1[0x51]);
        SetRect(&local_c4,0,param_1[0x51],param_1[0x4c],param_1[0xf]);
        InvalidateRect(hWnd,&local_c4,0);
      }
      break;
    }
  default:
    goto switchD_0041f2c9_caseD_9;
  case (HWND)0xd:
    if ((*(byte *)(param_1 + 0x4e) & 1) == 0) {
      (**(code **)(*(int *)param_1[8] + 0x14))((int *)param_1[8],0x100,param_3,param_4);
      break;
    }
    FUN_005f3ad0(param_1 + 0x28,&DAT_006a8170,*puVar1);
    if (param_1[0xf] < param_1[0x51] + param_1[0x32] * 2) {
      BVar4 = IsWindowVisible(*(HWND *)(param_1[0x26] + 0x18));
      if (BVar4 == 0) {
        FUN_0041fd00((int)param_1);
      }
      FUN_00420200(param_1,1);
    }
    else {
      param_1[0x51] = param_1[0x32] + param_1[0x51];
    }
    param_1[0x50] = param_1[0x4b];
    SetCaretPos(param_1[0x4b],param_1[0x51]);
LAB_0041f67b:
    *puVar1 = *puVar1 + 1;
    InvalidateRect(hWnd,(RECT *)0x0,0);
    break;
  case (HWND)0x21:
    goto switchD_0041f2c9_caseD_21;
  case (HWND)0x22:
    (**(code **)(param_1[0x25] + 8))();
    goto switchD_0041f2c9_caseD_21;
  case (HWND)0x23:
    param_1[0x50] = param_1[0x4c];
    FUN_00420580(param_1,puVar1);
    SetCaretPos(param_1[0x50],param_1[0x51]);
    break;
  case (HWND)0x24:
    ptVar11 = &local_e8;
    iVar9 = 1;
    pCVar6 = (LPCSTR)FUN_00583c40((int)&local_d4);
    GetTextExtentPoint32A((HDC)param_1[0x47],pCVar6,iVar9,ptVar11);
    iVar9 = param_1[0x50] - local_e8.cx;
    if (param_1[0x4b] < iVar9) {
      do {
        *puVar1 = *puVar1 - 1;
        param_1[0x50] = iVar9;
        FUN_005f3ef0(&local_d4,local_9c);
        FUN_005f3e00(local_9c);
        ptVar11 = &local_e8;
        iVar9 = 1;
        pCVar6 = (LPCSTR)FUN_00583c40((int)&local_d4);
        GetTextExtentPoint32A((HDC)param_1[0x47],pCVar6,iVar9,ptVar11);
        iVar9 = param_1[0x50] - local_e8.cx;
      } while (param_1[0x4b] < iVar9);
    }
    param_1[0x50] = param_1[0x4b];
    SetCaretPos(param_1[0x4b],param_1[0x51]);
    break;
  case (HWND)0x25:
    if ((*(byte *)(param_1 + 0x4e) & 1) != 0) {
      if (-1 < (int)(*puVar1 - 1)) {
        FUN_005f3ef0(&local_d4,local_90);
        FUN_005f3e00(local_90);
        *puVar1 = *puVar1 - 1;
        ptVar11 = &local_e0;
        iVar9 = 1;
        pCVar6 = (LPCSTR)FUN_00583c40((int)&local_d4);
        GetTextExtentPoint32A((HDC)param_1[0x47],pCVar6,iVar9,ptVar11);
        if ((param_1[0x50] - local_e0.cx < param_1[0x4b]) ||
           (iVar9 = FUN_005f3f80((int)&local_d4), iVar9 == 10)) {
          if (param_1[0x51] - param_1[0x32] < 0) {
            FUN_004201d0(param_1,1);
          }
          else {
            param_1[0x51] = param_1[0x51] - param_1[0x32];
          }
          param_1[0x50] = param_1[0x4c] - param_1[0x37] / 2;
          FUN_00420580(param_1,puVar1);
        }
        else {
          param_1[0x50] = param_1[0x50] - local_e0.cx;
        }
        SetCaretPos(param_1[0x50],param_1[0x51]);
      }
      break;
    }
    goto switchD_0041f2c9_caseD_9;
  case (HWND)0x26:
    if (param_1[0x51] - param_1[0x32] < 0) {
      FUN_004201d0(param_1,1);
      InvalidateRect(hWnd,(RECT *)0x0,0);
    }
    else {
      param_1[0x51] = param_1[0x51] - param_1[0x32] / 2;
    }
    goto LAB_0041f728;
  case (HWND)0x27:
    if ((*(byte *)(param_1 + 0x4e) & 1) != 0) {
      iVar9 = FUN_005f3fa0((int)&local_d4);
      if (iVar9 == 0) break;
      goto LAB_0041f67b;
    }
switchD_0041f2c9_caseD_9:
    (**(code **)(*(int *)param_1[8] + 0x14))((int *)param_1[8],0x100,param_3,param_4);
    break;
  case (HWND)0x28:
    iVar9 = param_1[0x32];
    if (param_1[0x51] + iVar9 * 2 < param_1[0xf]) {
      param_1[0x51] = iVar9 / 2 + iVar9 + param_1[0x51];
    }
    else {
      FUN_00420200(param_1,1);
      InvalidateRect(hWnd,(RECT *)0x0,0);
    }
LAB_0041f728:
    param_1[0x50] = param_1[0x50] + param_1[0x37] / 2;
    FUN_00420580(param_1,puVar1);
    SetCaretPos(param_1[0x50],param_1[0x51]);
    break;
  case (HWND)0x2d:
    uVar7 = param_1[0x4e];
    if ((uVar7 & 8) == 0) {
      param_1[0x4e] = uVar7 | 8;
    }
    else {
      param_1[0x4e] = uVar7 & 0xfffffff7;
    }
    break;
  case (HWND)0x2e:
    if ((*(byte *)(param_1 + 0x4e) & 1) == 0) {
      (**(code **)(*(int *)param_1[8] + 0x14))((int *)param_1[8],0x100,param_3,param_4);
    }
    else {
      FUN_005f3c30(param_1 + 0x28,*puVar1,*puVar1 + 1);
      SetRect(&local_c4,0,param_1[0x51],param_1[0x4c],param_1[0xf]);
      InvalidateRect(hWnd,&local_c4,0);
    }
  }
  local_4 = 0xffffffff;
  FUN_005f3e00(&local_d4.left);
  ExceptionList = local_c;
  return local_d8;
switchD_0041f2c9_caseD_21:
  (**(code **)(param_1[0x25] + 4))();
  goto switchD_0041f2c9_caseD_9;
}

