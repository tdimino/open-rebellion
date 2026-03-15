
LRESULT FUN_006083c0(int *param_1,uint param_2,uint param_3,int param_4)

{
  uint uVar1;
  POINT pt;
  bool bVar2;
  int *piVar3;
  void *pvVar4;
  HDC hdc;
  uint uVar5;
  int *piVar6;
  int iVar7;
  LPCSTR lpchText;
  int *piVar8;
  int *piVar9;
  int iVar10;
  WPARAM wParam;
  BOOL BVar11;
  undefined4 *puVar12;
  DWORD lParam;
  uint uVar13;
  LRESULT LVar14;
  HWND pHVar15;
  int iVar16;
  code *pcVar17;
  short sVar18;
  HWND pHVar19;
  undefined4 uVar20;
  tagRECT *lprc;
  LRESULT local_9c;
  byte *local_98;
  tagRECT tStack_94;
  HPALETTE local_84;
  undefined4 uStack_80;
  tagPOINT local_7c;
  int iStack_74;
  byte *pbStack_70;
  HDC pHStack_6c;
  undefined4 uStack_68;
  undefined4 local_64;
  undefined4 uStack_60;
  tagRECT tStack_5c;
  tagPAINTSTRUCT local_4c;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006567ac;
  local_c = ExceptionList;
  pHVar19 = (HWND)param_1[6];
  local_9c = 0;
  if (param_2 < 0x10) {
    if (param_2 == 0xf) {
      ExceptionList = &local_c;
      hdc = BeginPaint(pHVar19,&local_4c);
      pHStack_6c = hdc;
      local_84 = SelectPalette(hdc,DAT_006be5a0,0);
      if (param_1[0x29] != 0) {
        if (param_1[0x3c] == 0) {
          local_9c = param_1[0x31];
          local_98 = (byte *)0x0;
        }
        else {
          local_9c = 0;
          local_98 = (byte *)param_1[0x31];
        }
        uVar13 = param_1[0xe];
        uVar1 = param_1[0xf];
        iStack_74 = uVar13 + local_9c;
        pbStack_70 = local_98 + uVar1;
        uStack_80 = CONCAT22((short)local_98,(short)local_9c);
        if ((void *)param_1[0x2a] == (void *)0x0) {
          uVar5 = FUN_005fcfe0((void *)param_1[0x2b],0,0,uVar13,uVar1,0);
        }
        else {
          uVar5 = FUN_005fcc30((void *)param_1[0x2a],(uint *)param_1[0x2b],0,0,0,0,(undefined4 *)0x0
                               ,0);
        }
        FUN_005fc710((void *)param_1[0x29],(uint *)param_1[0x2b],0,0,
                     CONCAT31((int3)(uVar5 >> 8),(char)param_1[0x35]),local_9c,local_98,uVar13,uVar1
                    );
        FUN_005fc140((void *)param_1[0x2b],hdc,'@',0xcc0020,0,0,0,0,param_1[0xe],param_1[0xf]);
        (**(code **)(*(int *)param_1[0x28] + 8))();
        piVar6 = (int *)FUN_00609d10(param_1,uStack_80);
        SetBkMode(hdc,1);
        pcVar17 = SetTextColor_exref;
        for (; ((piVar6 != (int *)0x0 && (piVar6[10] <= iStack_74)) &&
               (piVar6[0xb] <= (int)pbStack_70)); piVar6 = (int *)(**(code **)(*piVar6 + 0xc))()) {
          iVar7 = FUN_00583c40((int)(piVar6 + 5));
          if (iVar7 != 0) {
            if ((piVar6[0xf] & 1U) == 0) {
              if ((piVar6[0xf] & 2U) == 0) {
                if (piVar6[0x16] == 0) {
                  iVar7 = param_1[0x36];
                }
                else {
                  iVar7 = piVar6[0x18];
                }
              }
              else if (piVar6[0x16] == 0) {
                iVar7 = param_1[0x38];
              }
              else {
                iVar7 = piVar6[0x19];
              }
            }
            else if (piVar6[0x16] == 0) {
              iVar7 = param_1[0x37];
            }
            else {
              iVar7 = piVar6[0x17];
            }
            (*pcVar17)(hdc,iVar7);
            if (piVar6[0xe] != 0) {
              (**(code **)(*param_1 + 0x18))(piVar6[0xe]);
            }
            uVar13 = piVar6[0x14];
            if (uVar13 == 0) {
              uVar13 = param_1[0x39];
            }
            FUN_006002c0(param_1,hdc);
            tStack_94.left = piVar6[0x10] + param_1[0x3a] + piVar6[0xc];
            tStack_94.top = piVar6[0x11] + param_1[0x3b] + piVar6[0xd];
            tStack_94.right = piVar6[0x12] + param_1[0x3a] + piVar6[0xc];
            tStack_94.bottom = piVar6[0x13] + param_1[0x3b] + piVar6[0xd];
            if ((uVar13 & 0x10) != 0) {
              uVar13 = uVar13 | 0x100;
            }
            if ((uVar13 & 3) == 1) {
              tStack_94.left = tStack_94.left + 2;
              tStack_94.right = tStack_94.right + -2;
            }
            else if ((uVar13 & 3) == 2) {
              tStack_94.left = tStack_94.left + 3;
            }
            else {
              tStack_94.right = tStack_94.right + -3;
            }
            lprc = &tStack_94;
            iVar7 = -1;
            lpchText = (LPCSTR)FUN_00583c40((int)(piVar6 + 5));
            hdc = pHStack_6c;
            DrawTextA(pHStack_6c,lpchText,iVar7,lprc,uVar13);
            FUN_006002f0(param_1,hdc);
            pcVar17 = SetTextColor_exref;
          }
        }
      }
      SelectPalette(hdc,local_84,0);
      EndPaint(pHVar19,&local_4c);
      ExceptionList = local_c;
      return 0;
    }
    if (param_2 == 1) {
      ExceptionList = &local_c;
      if (param_1[0x41] == 0) {
        ExceptionList = &local_c;
        local_84 = (HPALETTE)FUN_00618b70(0x28);
        local_4 = 0;
        if (local_84 == (HPALETTE)0x0) {
          pvVar4 = (void *)0x0;
        }
        else {
          pvVar4 = FUN_0060db00(local_84,param_1[8]);
        }
        param_1[0x41] = (int)pvVar4;
      }
      local_4 = 0xffffffff;
      local_84 = (HPALETTE)FUN_00618b70(0x28);
      local_4 = 1;
      if (local_84 != (HPALETTE)0x0) {
        pvVar4 = FUN_005fbda0(local_84,param_1[0xe],param_1[0xf],(uint *)param_1[0x29]);
        param_1[0x44] = (int)pvVar4;
        ExceptionList = local_c;
        return 0;
      }
      param_1[0x44] = 0;
      ExceptionList = local_c;
      return 0;
    }
switchD_00608acf_default:
    ExceptionList = &local_c;
    LVar14 = FUN_006007b0(param_1,param_2,param_3,param_4);
    ExceptionList = local_c;
    return LVar14;
  }
  if (param_2 < 0x2c) {
    if (param_2 == 0x2b) {
      return 0;
    }
    if (param_2 == 0x14) {
      return 1;
    }
    goto switchD_00608acf_default;
  }
  if (0x111 < param_2) {
    if (0x215 < param_2) {
      if (param_2 != 0x40d) {
        if (param_2 == 0x40e) {
          ExceptionList = &local_c;
          PostMessageA(*(HWND *)(param_1[8] + 0x18),0x40e,param_3,param_4);
          ExceptionList = local_c;
          return 0;
        }
        goto switchD_00608acf_default;
      }
LAB_006091c4:
      ExceptionList = &local_c;
      if ((*(byte *)(param_1 + 0x3d) & 1) != 0) {
        ExceptionList = &local_c;
        FUN_0060de50();
        param_1[0x3d] = param_1[0x3d] & 0xfffffffe;
        *(undefined2 *)(param_1 + 0x3f) = 0xfc18;
        *(undefined2 *)((int)param_1 + 0xfe) = 0xfc18;
        InvalidateRect((HWND)param_1[0x45],(RECT *)0x0,0);
      }
      pHVar15 = GetCapture();
      if (pHVar15 != pHVar19) {
        ExceptionList = local_c;
        return 0;
      }
      ReleaseCapture();
      ExceptionList = local_c;
      return 0;
    }
    if (param_2 == 0x215) goto LAB_006091c4;
    sVar18 = (short)param_4;
    switch(param_2) {
    case 0x200:
      local_7c.y = (LONG)param_4._2_2_;
      local_7c.x = (LONG)sVar18;
      ExceptionList = &local_c;
      pHVar15 = GetCapture();
      if (((pHVar15 == pHVar19) && ((param_3 & 1) != 0)) && ((param_1[0x3d] & 0x20001U) == 0)) {
        param_1[0x3d] = param_1[0x3d] | 1;
        if (((short)param_1[0x3f] == -1000) && (*(short *)((int)param_1 + 0xfe) == -1000)) {
          param_1[0x3f] = param_4;
        }
        UpdateWindow((HWND)param_1[0x45]);
        FUN_0060dc80((undefined4 *)param_1[0x41]);
        FUN_0060dbb0((void *)param_1[0x41],(uint *)param_1[0x44]);
        FUN_0060dcb0((void *)param_1[0x41],(int)(short)param_1[0x3f],
                     (int)*(short *)((int)param_1 + 0xfe));
        SetRect(&tStack_5c,0,0,param_1[0xe],param_1[0xf]);
        pt.y = (int)(short)((uint)param_1[0x3f] >> 0x10);
        pt.x = (int)(short)param_1[0x3f];
        BVar11 = PtInRect(&tStack_5c,pt);
        if (BVar11 == 0) {
          for (piVar6 = (int *)(**(code **)(*(int *)param_1[0x28] + 8))();
              (piVar6 != (int *)0x0 && ((*(byte *)(piVar6 + 0xf) & 1) == 0));
              piVar6 = (int *)(**(code **)(*piVar6 + 0xc))()) {
          }
        }
        FUN_0060dce0((undefined4 *)param_1[0x41]);
      }
      if ((*(byte *)(param_1 + 0x3d) & 1) == 0) {
        SetCursor((HCURSOR)param_1[0x2c]);
        if ((param_1[0x3d] & 0x40000U) == 0) {
          ExceptionList = local_c;
          return 0;
        }
        puVar12 = (undefined4 *)FUN_00609d80(param_1,&uStack_68,param_4);
        iVar10 = FUN_00609d10(param_1,*puVar12);
        iVar7 = param_1[0x40];
        if (iVar10 == iVar7) {
          ExceptionList = local_c;
          return 0;
        }
        if (iVar7 != 0) {
          *(uint *)(iVar7 + 0x3c) = *(uint *)(iVar7 + 0x3c) & 0xfffffffd;
          InvalidateRect(pHVar19,(RECT *)(param_1[0x40] + 0x40),0);
        }
        param_1[0x40] = iVar10;
        if (iVar10 == 0) {
          ExceptionList = local_c;
          return 0;
        }
        *(uint *)(iVar10 + 0x3c) = *(uint *)(iVar10 + 0x3c) | 2;
        InvalidateRect(pHVar19,(RECT *)(param_1[0x40] + 0x40),0);
        ExceptionList = local_c;
        return 0;
      }
      ClientToScreen(pHVar19,&local_7c);
      ScreenToClient((HWND)param_1[0x45],&local_7c);
      FUN_0060df00(&local_7c);
      if (param_1[0x32] == 0) {
        SetCursor((HCURSOR)param_1[0x2c]);
        ExceptionList = local_c;
        return 0;
      }
      if ((HCURSOR)param_1[0x2d] != (HCURSOR)0x0) {
        SetCursor((HCURSOR)param_1[0x2d]);
        ExceptionList = local_c;
        return 0;
      }
      SetCursor((HCURSOR)param_1[0x2c]);
      ExceptionList = local_c;
      return 0;
    case 0x201:
    case 0x204:
      goto switchD_00608acf_caseD_201;
    case 0x202:
      ExceptionList = &local_c;
      piVar6 = (int *)FUN_00609d80(param_1,&local_64,param_4);
      iVar7 = *piVar6;
      pcVar17 = PostMessageA_exref;
      if ((param_1[0x3d] & 1U) != 0) {
        iVar10 = (int)sVar18 - (int)(short)param_1[0x3f];
        iVar16 = (int)param_4._2_2_ - (int)*(short *)((int)param_1 + 0xfe);
        if ((0x18 < iVar10 * iVar10 + iVar16 * iVar16) &&
           (((sVar18 < 0 || (param_1[0xe] < (int)sVar18)) ||
            ((param_4 < 0 ||
             ((param_1[0xf] < (int)param_4._2_2_ || ((param_1[0x3d] & 0x100000U) == 0)))))))) {
          uVar13 = param_1[9];
          lParam = GetMessagePos();
          pcVar17 = PostMessageA_exref;
          PostMessageA(*(HWND *)(param_1[8] + 0x18),0x111,uVar13 & 0xffff | 0x29a0000,lParam);
          param_1[0x3d] = param_1[0x3d] & 0xffffffef;
        }
      }
      if ((*(byte *)(param_1 + 0x3d) & 0x10) == 0) goto LAB_00609151;
      iVar10 = FUN_00609d10(param_1,iVar7);
      if ((param_1[0x3d] & 0x10000U) != 0) {
        param_3 = 0;
      }
      if ((param_3 & 8) == 0) {
        if ((param_3 & 4) == 0) {
          iVar7 = FUN_00609d10(param_1,iVar7);
          if (iVar7 != 0) {
            uVar13 = *(uint *)(iVar7 + 0x3c);
            FUN_006094b0(param_1);
            *(uint *)(iVar7 + 0x3c) = *(uint *)(iVar7 + 0x3c) | 1;
            if (((uVar13 & 1) == 0) || (param_2 != 0x201)) {
              uVar13 = param_1[9] & 0xffffU | 0x29b0000;
            }
            else {
              uVar13 = param_1[9] & 0xffffU | 0x29d0000;
            }
            goto LAB_00609114;
          }
          FUN_006094b0(param_1);
          uVar13 = param_1[9];
          uVar20 = *(undefined4 *)(param_1[8] + 0x18);
        }
        else {
          if ((param_1[0x3d] & 2U) == 0) {
            param_1[0x3e] = iVar7;
          }
          FUN_00609590(param_1,param_1[0x3e],iVar7);
          uVar13 = param_1[9];
          uVar20 = *(undefined4 *)(param_1[8] + 0x18);
        }
        (*pcVar17)(uVar20,0x111,uVar13 & 0xffff | 0x29b0000,0);
      }
      else {
        if (iVar10 == 0) goto LAB_00609151;
        *(uint *)(iVar10 + 0x3c) = *(uint *)(iVar10 + 0x3c) ^ 1;
        uVar13 = param_1[9] & 0xffffU | 0x29b0000;
LAB_00609114:
        (*pcVar17)(*(undefined4 *)(param_1[8] + 0x18),0x111,uVar13,0);
        FUN_00609a00(param_1);
      }
LAB_00609151:
      pHVar15 = GetCapture();
      if (pHVar15 == pHVar19) {
        ReleaseCapture();
      }
      param_1[0x3d] = param_1[0x3d] & 0xffffffef;
      ExceptionList = local_c;
      return 0;
    case 0x203:
      ExceptionList = &local_c;
      PostMessageA(*(HWND *)(param_1[8] + 0x18),0x111,param_1[9] & 0xffffU | 0x3090000,param_4);
      ExceptionList = local_c;
      return 0;
    default:
      goto switchD_00608acf_default;
    }
  }
  if (param_2 == 0x111) {
    ExceptionList = &local_c;
    LVar14 = (**(code **)(*(int *)param_1[8] + 0x14))((int *)param_1[8],0x111,param_3,param_4);
    ExceptionList = local_c;
    return LVar14;
  }
  if (param_2 != 0x100) goto switchD_00608acf_default;
  ExceptionList = &local_c;
  piVar6 = (int *)(**(code **)(*(int *)param_1[0x28] + 8))();
  if ((piVar6 == (int *)0x0) || ((param_1[0x3d] & 0x200000U) != 0)) {
    LVar14 = (**(code **)(*(int *)param_1[8] + 0x14))((int *)param_1[8],0x100,param_3,param_4);
    ExceptionList = local_c;
    return LVar14;
  }
  do {
    piVar8 = piVar6;
    if ((*(byte *)(piVar6 + 0xf) & 1) != 0) break;
    piVar6 = (int *)(**(code **)(*piVar6 + 0xc))();
    piVar8 = (int *)0x0;
  } while (piVar6 != (int *)0x0);
  if (piVar8 == (int *)0x0) {
    piVar8 = (int *)(**(code **)(*(int *)param_1[0x28] + 8))();
  }
  bVar2 = true;
  switch(param_3) {
  case 0xd:
    if (piVar8 == (int *)0x0) {
      piVar6 = (int *)(**(code **)(*(int *)param_1[0x28] + 8))();
    }
    else {
      bVar2 = false;
      piVar8[0xf] = piVar8[0xf] | 1;
      PostMessageA(*(HWND *)(param_1[8] + 0x18),0x111,param_1[9] & 0xffffU | 0x3090000,param_4);
    }
    break;
  default:
    local_9c = (**(code **)(*(int *)param_1[8] + 0x14))((int *)param_1[8],0x100,param_3,param_4);
    goto LAB_006089f4;
  case 0x10:
  case 0x11:
  case 0x12:
    local_9c = FUN_006007b0(param_1,0x100,param_3,param_4);
LAB_006089f4:
    bVar2 = false;
    break;
  case 0x21:
    if (piVar8 == (int *)0x0) {
      piVar8 = (int *)(**(code **)(*(int *)param_1[0x28] + 8))();
    }
    else {
      piVar6 = piVar8;
      for (iVar7 = param_1[0xf] / param_1[0x34]; 0 < iVar7; iVar7 = iVar7 + -1) {
        iVar10 = (**(code **)(*piVar6 + 0x10))();
        if (iVar10 == 0) {
          iVar7 = 0;
        }
        else {
          piVar6 = (int *)(**(code **)(*piVar6 + 0x10))();
        }
      }
    }
    break;
  case 0x22:
    if (piVar8 == (int *)0x0) {
      piVar8 = (int *)(**(code **)(*(int *)param_1[0x28] + 8))();
    }
    else {
      piVar6 = piVar8;
      for (iVar7 = param_1[0xf] / param_1[0x34]; 0 < iVar7; iVar7 = iVar7 + -1) {
        iVar10 = (**(code **)(*piVar6 + 0xc))();
        if (iVar10 == 0) {
          iVar7 = 0;
        }
        else {
          piVar6 = (int *)(**(code **)(*piVar6 + 0xc))();
        }
      }
    }
    break;
  case 0x23:
    piVar9 = piVar8;
    if (piVar8 == (int *)0x0) {
      piVar9 = (int *)(**(code **)(*(int *)param_1[0x28] + 8))();
    }
    while (piVar3 = piVar9, piVar3 != (int *)0x0) {
      piVar9 = (int *)(**(code **)(*piVar3 + 0xc))();
      piVar6 = piVar3;
    }
    break;
  case 0x24:
    piVar6 = (int *)(**(code **)(*(int *)param_1[0x28] + 8))();
    break;
  case 0x26:
    if (piVar8 == (int *)0x0) {
      piVar6 = (int *)(**(code **)(*(int *)param_1[0x28] + 8))();
    }
    else {
      iVar7 = (**(code **)(*piVar8 + 0x10))();
      if (iVar7 == 0) {
        bVar2 = false;
        piVar6 = piVar8;
      }
      else {
        piVar6 = (int *)(**(code **)(*piVar8 + 0x10))();
      }
    }
    break;
  case 0x28:
    if (piVar8 == (int *)0x0) {
      piVar9 = (int *)(**(code **)(*(int *)param_1[0x28] + 8))();
      while (piVar3 = piVar9, piVar3 != (int *)0x0) {
        piVar9 = (int *)(**(code **)(*piVar3 + 0xc))();
        piVar6 = piVar3;
      }
    }
    else {
      iVar7 = (**(code **)(*piVar8 + 0xc))();
      if (iVar7 == 0) {
        bVar2 = false;
        piVar6 = piVar8;
      }
      else {
        piVar6 = (int *)(**(code **)(*piVar8 + 0xc))();
      }
    }
  }
  if (bVar2) {
    if (piVar8 != (int *)0x0) {
      piVar8[0xf] = piVar8[0xf] & 0xfffffffe;
    }
    if (piVar6 == (int *)0x0) {
      local_9c = (**(code **)(*(int *)param_1[8] + 0x14))((int *)param_1[8],0x100,param_3,param_4);
    }
    else {
      FUN_00609500(param_1,piVar6[3],1);
      FUN_00609ed0(param_1,piVar6[3]);
    }
  }
  ExceptionList = local_c;
  return local_9c;
switchD_00608acf_caseD_201:
  if ((param_1[0x3d] & 0x200000U) != 0) {
    ExceptionList = &local_c;
    LVar14 = (**(code **)(*(int *)param_1[8] + 0x14))((int *)param_1[8],param_2,param_3,param_4);
    ExceptionList = local_c;
    return LVar14;
  }
  if ((param_2 == 0x204) && ((param_1[0x3d] & 1U) != 0)) {
    return 0;
  }
  ExceptionList = &local_c;
  PostMessageA(*(HWND *)(param_1[8] + 0x18),0x40e,param_3,param_4);
  piVar6 = (int *)FUN_00609d80(param_1,&uStack_60,param_4);
  iVar7 = *piVar6;
  if (param_2 == 0x201) {
    FUN_00601080(pHVar19);
    param_1[0x3f] = param_4;
  }
  SetFocus((HWND)param_1[6]);
  if ((param_1[0x3d] & 0x10000U) != 0) {
    param_3 = 0;
  }
  iVar10 = FUN_00609d10(param_1,iVar7);
  if ((iVar10 != 0) && ((*(byte *)(iVar10 + 0x3c) & 1) != 0)) {
    param_1[0x3d] = param_1[0x3d] | 0x10;
    goto LAB_00608cb8;
  }
  if ((param_3 & 8) == 0) {
    if ((param_3 & 4) == 0) {
      iVar10 = FUN_00609d10(param_1,iVar7);
      if (iVar10 != 0) {
        uVar13 = *(uint *)(iVar10 + 0x3c);
        FUN_006094b0(param_1);
        *(uint *)(iVar10 + 0x3c) = *(uint *)(iVar10 + 0x3c) | 1;
        if (((uVar13 & 1) == 0) || (param_2 != 0x201)) {
          wParam = param_1[9] & 0xffffU | 0x29b0000;
        }
        else {
          wParam = param_1[9] & 0xffffU | 0x29d0000;
        }
        goto LAB_00608c73;
      }
      FUN_006094b0(param_1);
      uVar13 = param_1[9];
      pHVar19 = *(HWND *)(param_1[8] + 0x18);
    }
    else {
      if ((*(byte *)(param_1 + 0x3d) & 2) == 0) {
        param_1[0x3e] = iVar7;
      }
      FUN_00609590(param_1,param_1[0x3e],iVar7);
      uVar13 = param_1[9];
      pHVar19 = *(HWND *)(param_1[8] + 0x18);
    }
    PostMessageA(pHVar19,0x111,uVar13 & 0xffff | 0x29b0000,0);
  }
  else {
    if (iVar10 == 0) goto LAB_00608cb8;
    *(uint *)(iVar10 + 0x3c) = *(uint *)(iVar10 + 0x3c) ^ 1;
    wParam = param_1[9] & 0xffffU | 0x29b0000;
LAB_00608c73:
    PostMessageA(*(HWND *)(param_1[8] + 0x18),0x111,wParam,0);
    FUN_00609a00(param_1);
  }
LAB_00608cb8:
  param_1[0x3d] = param_1[0x3d] | 2;
  param_1[0x3e] = iVar7;
  ExceptionList = local_c;
  return 0;
}

