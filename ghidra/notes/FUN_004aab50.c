
UINT FUN_004aab50(int *param_1,uint param_2,HPALETTE param_3,HWND param_4)

{
  HWND hWnd;
  uint uVar1;
  void *this;
  int *piVar2;
  POINT pt;
  POINT pt_00;
  HDC hdc;
  HPALETTE hPal;
  int iVar3;
  undefined4 *puVar4;
  HGDIOBJ pvVar5;
  UINT_PTR UVar6;
  BOOL BVar7;
  UINT UVar8;
  uint uVar9;
  UINT unaff_EBX;
  UINT unaff_EDI;
  undefined2 uVar10;
  short sVar11;
  undefined2 uVar12;
  tagMSG local_5c;
  tagPAINTSTRUCT local_40;
  
  hWnd = (HWND)param_1[6];
  uVar10 = (undefined2)((uint)param_4 >> 0x10);
  if (param_2 < 0x19) {
    if (param_2 == 0x18) {
      if (param_3 != (HPALETTE)0x0) {
        SetWindowPos(hWnd,(HWND)0x0,0,0,0,0,0x203);
        UVar6 = SetTimer(hWnd,100,500,(TIMERPROC)0x0);
        param_1[0x4d] = UVar6;
        return 0;
      }
      if (param_1[0x46] == 0) {
        return 0;
      }
      InvalidateRect(hWnd,(RECT *)(param_1[0x46] + 0x60),0);
      if (*(int **)(param_1[0x46] + 0x3c) != (int *)0x0) {
        (**(code **)(**(int **)(param_1[0x46] + 0x3c) + 8))();
      }
      param_1[0x46] = 0;
      if (param_1[0x4d] == 0) {
        return 0;
      }
      KillTimer((HWND)param_1[6],param_1[0x4d]);
      param_1[0x4d] = 0;
      return 0;
    }
    if (param_2 == 0xf) {
      hdc = BeginPaint(hWnd,&local_40);
      hPal = SelectPalette(hdc,DAT_006be5a0,0);
      FUN_006002c0(param_1,(HDC)param_1[0x4a]);
      uVar9 = param_1[10];
      uVar1 = param_1[0xb];
      iVar3 = FUN_005fc0f0((int *)param_1[0x29]);
      uVar10 = (undefined2)iVar3;
      uVar12 = (undefined2)((uint)iVar3 >> 0x10);
      puVar4 = (undefined4 *)FUN_005fc0e0((int *)param_1[0x29]);
      FUN_005fcc30((void *)param_1[0x4b],(uint *)param_1[0x29],0,0,((short)uVar9 < 0) - 1 & uVar9,
                   ((short)uVar1 < 0) - 1 & uVar1,puVar4,CONCAT22(uVar12,uVar10));
      iVar3 = FUN_005fc0f0((int *)param_1[0x29]);
      sVar11 = (short)iVar3;
      iVar3 = FUN_005fc0e0((int *)param_1[0x29]);
      FUN_005fe050((void *)param_1[0x29],0,0,(short)iVar3,sVar11);
      pvVar5 = (HGDIOBJ)FUN_005fc130(param_1[0x29]);
      pvVar5 = SelectObject((HDC)param_1[0x4a],pvVar5);
      for (this = (void *)param_1[0x50]; this != (void *)0x0; this = *(void **)((int)this + 8)) {
        piVar2 = *(int **)((int)this + 0x3c);
        if (piVar2 != (int *)0x0) {
          local_5c.wParam = *(WPARAM *)((int)this + 0x68);
          local_5c.lParam = *(LPARAM *)((int)this + 0x6c);
          if (this == (void *)param_1[0x46]) {
            FUN_005fd0f0((void *)param_1[0x47],(uint *)param_1[0x29],*(int *)((int)this + 0x60) + 6,
                         *(int *)((int)this + 100));
          }
          else {
            FUN_005fd0f0((void *)param_1[0x48],(uint *)param_1[0x29],*(int *)((int)this + 0x60) + 6,
                         *(int *)((int)this + 100));
            (**(code **)(*piVar2 + 8))();
          }
        }
        FUN_004abe30(this,(int *)param_1[0x29]);
        FUN_004aba60(this,(HDC)param_1[0x4a],0);
      }
      if ((void *)param_1[0x46] != (void *)0x0) {
        FUN_004aba60((void *)param_1[0x46],(HDC)param_1[0x4a],1);
      }
      SelectObject((HDC)param_1[0x4a],pvVar5);
      if ((void *)param_1[0x2a] != (void *)0x0) {
        FUN_005fd0f0((void *)param_1[0x2a],(uint *)param_1[0x29],0,0);
      }
      FUN_005fc140((void *)param_1[0x29],hdc,'@',0xcc0020,0,0,0,0,0,0);
      SelectPalette(hdc,hPal,0);
      FUN_006002f0(param_1,(HDC)param_1[0x4a]);
      EndPaint(hWnd,&local_40);
      return 0;
    }
    if (param_2 == 0x10) {
      if (param_1[0x4d] == 0) {
        return 0;
      }
      KillTimer(hWnd,param_1[0x4d]);
      param_1[0x4d] = 0;
      return 0;
    }
  }
  else if (param_2 < 0x114) {
    if (param_2 == 0x113) {
      BVar7 = PeekMessageA(&local_5c,hWnd,0,0,0);
      if (BVar7 != 0) {
        return 0;
      }
      if (param_1[0x46] == 0) {
        return 0;
      }
      piVar2 = *(int **)(param_1[0x46] + 0x3c);
      if (piVar2 == (int *)0x0) {
        return 0;
      }
      (**(code **)(*piVar2 + 4))(5);
      return unaff_EBX;
    }
    if (param_2 == 0x100) {
      if (param_3 != (HPALETTE)0xd) {
        if (param_3 != (HPALETTE)0x1b) {
          (**(code **)(*(int *)param_1[8] + 0x14))
                    ((int *)param_1[8],0x100,(short)param_3,(short)param_4);
          return unaff_EDI;
        }
        if (param_1[0x52] == 0) {
          SetFocus(*(HWND *)(param_1[8] + 0x18));
          (**(code **)(*param_1 + 0x30))();
          return 0;
        }
        SetFocus(*(HWND *)(param_1[0x52] + 0x18));
        (**(code **)(*param_1 + 0x30))();
        return 0;
      }
      iVar3 = param_1[0x46];
      if (iVar3 == 0) {
        return 0;
      }
      local_5c.lParam = *(int *)(iVar3 + 0x6c);
      iVar3 = (**(code **)(*param_1 + 0x58))
                        (iVar3,(*(int *)(iVar3 + 0x68) - *(int *)(iVar3 + 0x60)) / 2 +
                               *(int *)(iVar3 + 0x60),
                         (short)((local_5c.lParam - *(int *)(iVar3 + 100)) / 2 +
                                *(int *)(iVar3 + 100)),0x202);
      if (iVar3 != 0) {
        return 0;
      }
      iVar3 = param_1[0x46];
      if (iVar3 == 0) {
        return 0;
      }
      if (*(int *)(iVar3 + 0x3c) == 0) {
        if (param_1[0x52] == 0) {
          uVar9 = param_1[9];
        }
        else {
          uVar9 = *(uint *)(param_1[0x52] + 0x24);
        }
        PostMessageA(*(HWND *)(param_1[8] + 0x18),0x111,
                     *(int *)(iVar3 + 0xc) << 0x10 | uVar9 & 0xffff,0);
        param_1[0x46] = 0;
      }
      InvalidateRect(hWnd,(RECT *)(param_1[0x46] + 0x60),0);
      return 0;
    }
  }
  else if (param_2 == 0x200) {
    for (iVar3 = param_1[0x50]; iVar3 != 0; iVar3 = *(int *)(iVar3 + 8)) {
      if (*(int *)(iVar3 + 0x58) != 0) {
        pt.y._2_2_ = 0;
        pt._0_6_ = CONCAT24(uVar10,param_4) & 0xffff0000ffff;
        BVar7 = PtInRect((RECT *)(iVar3 + 0x60),pt);
        *(BOOL *)(iVar3 + 0x54) = BVar7;
        if ((BVar7 != 0) && (iVar3 != param_1[0x46])) {
          if (param_1[0x46] != 0) {
            InvalidateRect(hWnd,(RECT *)(param_1[0x46] + 0x60),0);
          }
          InvalidateRect(hWnd,(RECT *)(iVar3 + 0x60),0);
          param_1[0x46] = iVar3;
          break;
        }
      }
    }
    iVar3 = param_1[0x46];
    if (iVar3 != 0) {
      pt_00.y._2_2_ = 0;
      pt_00._0_6_ = CONCAT24(uVar10,param_4) & 0xffff0000ffff;
      BVar7 = PtInRect((RECT *)(iVar3 + 0x60),pt_00);
      *(BOOL *)(iVar3 + 0x54) = BVar7;
      if (BVar7 == 0) {
        InvalidateRect(hWnd,(RECT *)(param_1[0x46] + 0x60),0);
        param_1[0x46] = 0;
      }
    }
    param_2 = 0x200;
  }
  else if (param_2 == 0x202) {
    if (param_1[0x46] == 0) {
      return 0;
    }
    iVar3 = (**(code **)(*param_1 + 0x58))(param_1[0x46],(int)(short)param_4,uVar10,0x202);
    if ((iVar3 == 0) && (iVar3 = param_1[0x46], iVar3 != 0)) {
      if (*(int *)(iVar3 + 0x3c) != 0) {
        return 0;
      }
      if (param_1[0x52] == 0) {
        uVar9 = param_1[9];
      }
      else {
        uVar9 = *(uint *)(param_1[0x52] + 0x24);
      }
      PostMessageA(*(HWND *)(param_1[8] + 0x18),0x111,*(int *)(iVar3 + 0xc) << 0x10 | uVar9 & 0xffff
                   ,0);
      param_1[0x46] = 0;
      return 0;
    }
    param_2 = 0x202;
  }
  UVar8 = FUN_00606650(param_1,param_2,param_3,param_4);
  return UVar8;
}

