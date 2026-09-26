
UINT FUN_004534f0(int *param_1,uint param_2,HPALETTE param_3,HWND param_4)

{
  HWND hWnd;
  uint uVar1;
  POINT pt;
  POINT pt_00;
  POINT pt_01;
  POINT pt_02;
  POINT pt_03;
  POINT pt_04;
  void **ppvVar2;
  HDC hdc;
  HPALETTE hPal;
  int *piVar3;
  int iVar4;
  HWND pHVar5;
  HWND pHVar6;
  DWORD lParam;
  BOOL BVar7;
  UINT UVar8;
  int iVar9;
  RECT *pRVar10;
  HWND local_84;
  UINT local_80;
  HWND local_7c;
  int iStack_78;
  tagPOINT local_74;
  tagRECT local_6c;
  RECT RStack_5c;
  tagPAINTSTRUCT local_4c;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062fe63;
  local_c = ExceptionList;
  hWnd = (HWND)param_1[6];
  pHVar6 = *(HWND *)(param_1[8] + 0x18);
  local_80 = 0;
  local_84 = hWnd;
  local_7c = pHVar6;
  if (0x111 < param_2) {
    if (param_2 < 0x408) {
      if (param_2 == 0x407) {
        ExceptionList = &local_c;
        param_1[0x59] = param_1[0x59] & 0xfffffff7;
        FUN_00458780(param_1,param_1[0x92]);
        ExceptionList = local_c;
        return local_80;
      }
      switch(param_2) {
      case 0x200:
        goto switchD_004537c6_caseD_200;
      case 0x201:
      case 0x204:
        local_74.y = (uint)param_4 >> 0x10;
        uVar1 = param_1[0x59];
        local_74.x = (uint)param_4 & 0xffff;
        ExceptionList = &local_c;
        if ((((uVar1 & 8) != 0) && (ExceptionList = &local_c, (uVar1 & 4) == 0)) &&
           (ExceptionList = &local_c, param_1[0x92] != 0)) {
          ExceptionList = &local_c;
          param_1[0x59] = uVar1 & 0xfffffff7;
          FUN_00458780(param_1,param_1[0x92]);
        }
        SendMessageA(pHVar6,0x467,(WPARAM)local_84,0);
        SetRect(&local_6c,8,0x39,0xe6,0x11a);
        param_1[0x92] = 0;
        param_1[0x59] = param_1[0x59] & 0xfffffffb;
        pt.y = local_74.y;
        pt.x = local_74.x;
        BVar7 = PtInRect(&local_6c,pt);
        if ((BVar7 != 0) && (param_1[0x58] == 0x67)) {
          iVar4 = 0;
          pRVar10 = (RECT *)(param_1 + 0x68);
          do {
            pt_00.y = local_74.y;
            pt_00.x = local_74.x;
            BVar7 = PtInRect(pRVar10,pt_00);
            if (BVar7 != 0) {
              param_1[0x92] = (int)pRVar10;
              break;
            }
            iVar4 = iVar4 + 1;
            pRVar10 = (RECT *)&pRVar10[3].right;
          } while (iVar4 < 3);
          if (((param_1[0x92] != 0) && (((uint)param_3 & 8) == 0)) &&
             ((*(byte *)(param_1[0x92] + 0x30) & 1) == 0)) {
            FUN_00458410(param_1,(RECT *)0x0);
            FUN_004585a0(param_1,(RECT *)param_1[0x92]);
          }
        }
        FUN_00601080(local_84);
        if (param_1[0x92] != 0) {
          ExceptionList = local_c;
          return local_80;
        }
        UVar8 = FUN_004ac3a0(param_1,param_2,param_3,param_4);
        ExceptionList = local_c;
        return UVar8;
      case 0x202:
      case 0x205:
        local_74.x = (uint)param_4 & 0xffff;
        local_74.y = (uint)param_4 >> 0x10;
        ExceptionList = &local_c;
        if (param_2 == 0x202) {
          pHVar6 = hWnd;
          if (param_1[0x58] != 0x67) {
            pHVar6 = *(HWND *)(param_1[0x5d] + 0x18);
          }
          ExceptionList = &local_c;
          SetFocus(pHVar6);
        }
        pHVar6 = GetCapture();
        if (pHVar6 == hWnd) {
          ReleaseCapture();
        }
        SetRect(&local_6c,8,0x39,0xe6,0x11a);
        if ((param_1[0x59] & 2U) != 0) {
          param_1[0x59] = param_1[0x59] & 0xfffffffd;
          FUN_0060de50();
          iVar4 = param_1[0x93];
          if (iVar4 < local_74.x) {
            iVar4 = local_74.x - iVar4;
          }
          else {
            iVar4 = iVar4 - local_74.x;
          }
          iVar9 = param_1[0x94];
          if (iVar9 < local_74.y) {
            iVar9 = local_74.y - iVar9;
          }
          else {
            iVar9 = iVar9 - local_74.y;
          }
          if ((iVar4 < 4) && (iVar9 < 4)) {
            ExceptionList = local_c;
            return local_80;
          }
          uVar1 = param_1[9];
          lParam = GetMessagePos();
          PostMessageA(*(HWND *)(param_1[0x53] + 0x18),0x111,uVar1 & 0xffff | 0x29a0000,lParam);
          ExceptionList = local_c;
          return local_80;
        }
        pt_01.y = local_74.y;
        pt_01.x = local_74.x;
        BVar7 = PtInRect(&local_6c,pt_01);
        if ((BVar7 != 0) && (pRVar10 = (RECT *)param_1[0x92], pRVar10 != (RECT *)0x0)) {
          if (((uint)param_3 & 8) == 0) {
            if ((pRVar10 != (RECT *)0x0) && (((uint)param_3 & 8) == 0)) {
              if (((*(byte *)(param_1 + 0x59) & 4) == 0) ||
                 ((param_2 != 0x202 || (iVar4 = pRVar10[2].top, iVar4 == 0)))) {
                if ((pRVar10[2].top != 0) &&
                   (piVar3 = FUN_004f2f60(*(int *)(param_1[0x53] + 0x9c),(uint *)&pRVar10[1].top),
                   piVar3 != (int *)0x0)) {
                  FUN_00520570(&local_7c);
                  local_4 = 0;
                  FUN_00528720(piVar3,1,*(undefined4 *)(param_1[0x53] + 0x9c),&local_7c);
                  if (iStack_78 == -1) {
                    param_1[0x59] = param_1[0x59] | 4;
                  }
                  local_4 = 0xffffffff;
                  FUN_00619730();
                }
              }
              else if ((pRVar10[3].left & 1) != 0) {
                RStack_5c.left = *(LONG *)(iVar4 + 0x14);
                RStack_5c.top = *(LONG *)(iVar4 + 0x18);
                RStack_5c.right = *(LONG *)(iVar4 + 0x1c);
                RStack_5c.bottom = *(LONG *)(iVar4 + 0x20);
                pt_02.y = local_74.y;
                pt_02.x = local_74.x;
                BVar7 = PtInRect(&RStack_5c,pt_02);
                if ((BVar7 != 0) && (param_1[0x95] != 0)) {
                  param_1[0x59] = param_1[0x59] & 0xfffffffbU | 8;
                  FUN_00458640(param_1,(int *)param_1[0x92]);
                }
              }
              FUN_00458410(param_1,(RECT *)0x0);
              FUN_004585a0(param_1,(RECT *)param_1[0x92]);
            }
          }
          else if ((pRVar10[3].left & 1) == 0) {
            FUN_004585a0(param_1,pRVar10);
          }
          else {
            FUN_00458410(param_1,pRVar10);
          }
        }
        UVar8 = FUN_004ac3a0(param_1,param_2,param_3,param_4);
        ExceptionList = local_c;
        return UVar8;
      default:
        goto switchD_004537c6_caseD_203;
      }
    }
    if (param_2 == 0x467) {
      ExceptionList = &local_c;
      InvalidateRect(hWnd,(RECT *)(param_1 + 0x4a),0);
      if (param_4 == hWnd) {
        ExceptionList = local_c;
        return local_80;
      }
      FUN_00458410(param_1,(RECT *)0x0);
      ExceptionList = local_c;
      return local_80;
    }
switchD_004537c6_caseD_203:
    ExceptionList = &local_c;
    UVar8 = FUN_004ac3a0(param_1,param_2,param_3,param_4);
    ExceptionList = local_c;
    return UVar8;
  }
  if (param_2 != 0x111) {
    if (param_2 == 0xf) {
      ExceptionList = &local_c;
      hdc = BeginPaint(hWnd,&local_4c);
      hPal = SelectPalette(hdc,DAT_006be5a0,0);
      FUN_00607710(param_1,hdc);
      FUN_00457690(param_1,hdc);
      FUN_00601ce0((void *)param_1[0x49],hdc);
      SelectPalette(hdc,hPal,0);
      EndPaint(local_84,&local_4c);
      ExceptionList = local_c;
      return local_80;
    }
    if (param_2 == 0x10) {
      ExceptionList = &local_c;
      SendMessageA(pHVar6,0x467,0,0);
      ExceptionList = local_c;
      return local_80;
    }
    goto switchD_004537c6_caseD_203;
  }
  switch((uint)param_3 & 0xffff) {
  case 7:
    if (param_1[0x4e] != 0) {
      ExceptionList = &local_c;
      SetFocus(*(HWND *)(param_1[0x4e] + 0x18));
      ExceptionList = local_c;
      return local_80;
    }
    ppvVar2 = &local_c;
    if (param_1[0x58] == 0x67) {
      return 0;
    }
    goto LAB_004536f5;
  default:
    if ((uint)param_3 >> 0x10 == 0x29a) {
      local_74.y = (uint)param_4 >> 0x10;
      local_74.x = (uint)param_4 & 0xffff;
      ExceptionList = &local_c;
      pHVar5 = WindowFromPoint((POINT)(CONCAT44(local_74.y,param_4) & 0xffffffff0000ffff));
      if (pHVar5 != hWnd) {
        PostMessageA(pHVar6,0x111,param_1[9] & 0xffffU | 0x29a0000,(LPARAM)param_4);
      }
    }
    else if ((uint)param_3 >> 0x10 == 0x29b) {
      if (param_1[0x58] != 0x67) {
        ExceptionList = &local_c;
        PostMessageA(pHVar6,0x467,(WPARAM)hWnd,0);
      }
    }
    else {
      ExceptionList = &local_c;
      local_80 = FUN_004ac3a0(param_1,0x111,param_3,param_4);
    }
    break;
  case 100:
    ExceptionList = &local_c;
    PostMessageA(pHVar6,0x111,param_1[9] & 0xffffU | 0x4660000,0);
    break;
  case 0x65:
    ExceptionList = &local_c;
    (**(code **)(*param_1 + 0x30))();
    break;
  case 0x70:
    ExceptionList = &local_c;
    FUN_004568a0(param_1,(uint)param_3 >> 0x10);
    ppvVar2 = ExceptionList;
    if (param_1[0x58] == 0x67) {
      SetFocus((HWND)param_1[6]);
      ExceptionList = local_c;
      return local_80;
    }
LAB_004536f5:
    ExceptionList = ppvVar2;
    SetFocus(*(HWND *)(param_1[0x5d] + 0x18));
    break;
  case 0xc9:
    ExceptionList = &local_c;
    piVar3 = FUN_004a25c0(param_1);
    if ((piVar3 != (int *)0x0) && (param_1[0x53] != 0)) {
      FUN_004ece30(&local_84);
      local_4 = 1;
      iVar4 = FUN_004f6b70(piVar3,(uint *)&local_84);
      if (iVar4 != 0) {
        FUN_00429ce0((void *)param_1[0x53],(uint *)&local_84);
      }
      local_4 = 0xffffffff;
      FUN_00619730();
    }
  }
  ExceptionList = local_c;
  return local_80;
switchD_004537c6_caseD_200:
  local_74.x = (uint)param_4 & 0xffff;
  local_74.y = (uint)param_4 >> 0x10;
  ExceptionList = &local_c;
  if ((((uint)param_3 & 1) != 0) && (ExceptionList = &local_c, param_1[0x38] == 0)) {
    ExceptionList = &local_c;
    if ((*(byte *)(param_1 + 0x59) & 2) != 0) goto LAB_00453d1f;
    ExceptionList = &local_c;
    SetRect(&local_6c,8,0x39,0xe6,0x11a);
    pt_03.y = local_74.y;
    pt_03.x = local_74.x;
    BVar7 = PtInRect(&local_6c,pt_03);
    if (BVar7 != 0) {
      param_1[0x59] = param_1[0x59] | 2;
      if (((uint)param_3 & 8) != 0) {
        iVar4 = 0;
        pRVar10 = (RECT *)(param_1 + 0x68);
        do {
          pt_04.y = local_74.y;
          pt_04.x = local_74.x;
          BVar7 = PtInRect(pRVar10,pt_04);
          if ((BVar7 != 0) && ((pRVar10[3].left & 1) == 0)) {
            param_1[0x92] = (int)pRVar10;
            FUN_004585a0(param_1,pRVar10);
            break;
          }
          iVar4 = iVar4 + 1;
          pRVar10 = (RECT *)&pRVar10[3].right;
        } while (iVar4 < 3);
      }
      param_1[0x93] = local_74.x;
      param_1[0x94] = local_74.y;
      if ((((void *)param_1[0x99] != (void *)0x0) && (iVar4 = param_1[0x98], iVar4 != 0)) &&
         (iVar4 < 4)) {
        FUN_0060dbb0((void *)param_1[0x99],(uint *)param_1[iVar4 + 0x94]);
        FUN_0060dcb0((void *)param_1[0x99],3,3);
        MapWindowPoints(local_84,local_7c,&local_74,1);
        FUN_0060dce0((undefined4 *)param_1[0x99]);
        ExceptionList = local_c;
        return local_80;
      }
      param_1[0x59] = param_1[0x59] & 0xfffffffd;
      ExceptionList = local_c;
      return local_80;
    }
  }
  if ((*(byte *)(param_1 + 0x59) & 2) == 0) {
    UVar8 = FUN_004ac3a0(param_1,param_2,param_3,param_4);
    ExceptionList = local_c;
    return UVar8;
  }
LAB_00453d1f:
  MapWindowPoints(local_84,local_7c,&local_74,1);
  FUN_0060df00(&local_74);
  ExceptionList = local_c;
  return local_80;
}

