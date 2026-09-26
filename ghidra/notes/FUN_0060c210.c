
LRESULT FUN_0060c210(int *param_1,uint param_2,uint param_3,HWND param_4)

{
  int iVar1;
  int iVar2;
  void **ppvVar3;
  ushort uVar4;
  LRESULT LVar5;
  void *pvVar6;
  int iVar7;
  int iVar8;
  int iVar9;
  HDC hdc;
  HDC hdc_00;
  HGDIOBJ pvVar10;
  HPALETTE pHVar11;
  int iVar12;
  LPCSTR lpchText;
  uint uVar13;
  HWND pHVar14;
  BOOL BVar15;
  undefined2 extraout_var;
  undefined2 extraout_var_00;
  code *pcVar16;
  int *piVar17;
  undefined4 unaff_retaddr;
  tagMSG local_68;
  tagPAINTSTRUCT local_4c;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00656af6;
  local_c = ExceptionList;
  piVar17 = (int *)0x0;
  if (param_2 < 0x10) {
    if (param_2 == 0xf) {
      pHVar14 = (HWND)param_1[6];
      ExceptionList = &local_c;
      hdc = BeginPaint(pHVar14,&local_4c);
      pHVar11 = DAT_006be5a0;
      if (param_1[0x3c] != 0) {
        hdc_00 = CreateCompatibleDC(hdc);
        pvVar10 = (HGDIOBJ)FUN_005fc130(param_1[0x3c]);
        pvVar10 = SelectObject(hdc_00,pvVar10);
        pcVar16 = SelectPalette_exref;
        SelectPalette(hdc_00,pHVar11,0);
        pHVar11 = SelectPalette(hdc,pHVar11,0);
        FUN_005fcc30((void *)param_1[0x3b],(uint *)param_1[0x3c],0,0,param_1[10],param_1[0xb],
                     (undefined4 *)param_1[0xe],param_1[0xf]);
        if (param_1[0x38] != 0) {
          pvVar6 = (void *)FUN_0060bd00(param_1[0x38]);
          if (pvVar6 != (void *)0x0) {
            FUN_005fd0f0(pvVar6,(uint *)param_1[0x3c],0,0);
          }
          iVar12 = FUN_00583c40(param_1[0x38] + 0x14);
          if (iVar12 != 0) {
            local_68.wParam = param_1[0xe];
            local_68.hwnd = (HWND)0x0;
            local_68.message = 0;
            local_68.lParam = param_1[0xf];
            FUN_006002c0(param_1,hdc_00);
            SetBkMode(hdc_00,1);
            SetTextColor(hdc_00,param_1[0x25]);
            iVar12 = param_1[0x38];
            local_68.hwnd =
                 (HWND)((int)&(local_68.hwnd)->unused + *(int *)(iVar12 + 0x30) + param_1[0x27]);
            local_68.message = local_68.message + *(int *)(iVar12 + 0x34) + param_1[0x28];
            local_68.wParam = local_68.wParam + *(int *)(iVar12 + 0x30) + param_1[0x27];
            local_68.lParam = local_68.lParam + *(int *)(iVar12 + 0x34) + param_1[0x28];
            lpchText = (LPCSTR)FUN_00583c40(iVar12 + 0x14);
            DrawTextA(hdc_00,lpchText,-1,(LPRECT)&local_68,param_1[0x26]);
            FUN_006002f0(param_1,hdc_00);
            pcVar16 = SelectPalette_exref;
          }
        }
        BitBlt(hdc,0,0,param_1[0xe],param_1[0xf],hdc_00,0,0,0xcc0020);
        (*pcVar16)(hdc,pHVar11,0);
        (*pcVar16)(hdc_00,unaff_retaddr,0);
        SelectObject(hdc_00,pvVar10);
        DeleteDC(hdc_00);
      }
      EndPaint(pHVar14,&local_4c);
      ExceptionList = local_c;
      return 0;
    }
    if (param_2 == 1) {
      iVar12 = param_1[7];
      if ((short)param_1[0x2e] == 0) {
        return 0;
      }
      ExceptionList = &local_c;
      pvVar6 = (void *)FUN_00618b70(0x28);
      local_4 = 0;
      if (pvVar6 != (void *)0x0) {
        piVar17 = FUN_005fbd20(pvVar6,iVar12,CONCAT22(extraout_var,(short)param_1[0x2e]),2);
      }
      iVar1 = param_1[0xe];
      local_4 = 0xffffffff;
      iVar7 = FUN_005fc0e0(piVar17);
      iVar8 = FUN_005fc0e0(piVar17);
      iVar9 = FUN_005fc0f0(piVar17);
      iVar2 = param_1[0xf];
      if (piVar17 != (int *)0x0) {
        FUN_005fbfa0(piVar17);
        FUN_00618b60((undefined *)piVar17);
      }
      pvVar6 = (void *)FUN_00618b70(0x114);
      local_4 = 1;
      if (pvVar6 == (void *)0x0) {
        pvVar6 = (void *)0x0;
      }
      else {
        pvVar6 = FUN_00602150(pvVar6,iVar12,iVar1 - iVar7,(iVar2 - iVar9) / 2,iVar8,iVar9,param_1,
                              1000,CONCAT22((short)((uint)param_1[0x3c] >> 0x10),
                                            (short)param_1[0x2e]),
                              CONCAT22(extraout_var_00,*(undefined2 *)((int)param_1 + 0xba)),
                              (uint *)param_1[0x3c],0,0);
      }
      local_4 = 0xffffffff;
      FUN_005ffce0(pvVar6,0);
      FUN_005f4f10(param_1 + 0x1b,(int)pvVar6);
      ExceptionList = local_c;
      return 0;
    }
    if (param_2 == 2) {
      ExceptionList = &local_c;
      FUN_0060cbf0((int)param_1);
      LVar5 = DefWindowProcA((HWND)param_1[6],2,param_3,(LPARAM)param_4);
      ExceptionList = local_c;
      return LVar5;
    }
  }
  else if (param_2 < 0x22) {
    if (param_2 == 0x21) {
      return 1;
    }
    ppvVar3 = &local_c;
    if (param_2 == 0x18) {
joined_r0x0060c7c8:
      ExceptionList = ppvVar3;
      if (param_3 != 0) {
        ExceptionList = local_c;
        return 0;
      }
      FUN_0060cbf0((int)param_1);
      ExceptionList = local_c;
      return 0;
    }
  }
  else if (param_2 < 0x112) {
    if (param_2 == 0x111) {
      uVar13 = param_3 & 0xffff;
      if (uVar13 == 1000) {
        ExceptionList = &local_c;
        PostMessageA((HWND)param_1[6],0x111,0x3ea,0);
      }
      else {
        if (uVar13 == 0x3e9) {
          ExceptionList = &local_c;
          FUN_0060cbf0((int)param_1);
          ExceptionList = local_c;
          return 0;
        }
        if (uVar13 == 0x3ea) {
          ExceptionList = &local_c;
          if (param_1[0x3a] != 0) {
            ExceptionList = &local_c;
            FUN_0060cbf0((int)param_1);
            ExceptionList = local_c;
            return 0;
          }
          while ((uVar4 = GetAsyncKeyState(1), (uVar4 & 0x8000) != 0 ||
                 (pHVar14 = GetCapture(), pHVar14 != (HWND)0x0))) {
            BVar15 = PeekMessageA(&local_68,(HWND)0x0,0,0,0);
            if (BVar15 != 0) {
              GetMessageA(&local_68,(HWND)0x0,0,0);
              TranslateMessage(&local_68);
              DispatchMessageA(&local_68);
            }
          }
          FUN_00601080((HWND)param_1[6]);
          FUN_0060cac0((int)param_1);
          ExceptionList = local_c;
          return 0;
        }
      }
      ExceptionList = local_c;
      return 0;
    }
    switch(param_2) {
    case 0x100:
    case 0x102:
    case 0x104:
      ExceptionList = &local_c;
      LVar5 = (**(code **)(*(int *)param_1[8] + 0x14))((int *)param_1[8],param_2,param_3,param_4);
      ExceptionList = local_c;
      return LVar5;
    case 0x101:
    case 0x105:
      ExceptionList = &local_c;
      iVar12 = FUN_0060c810(param_1,param_3);
      if (iVar12 == 0) {
        LVar5 = (**(code **)(*(int *)param_1[8] + 0x14))((int *)param_1[8],param_2,param_3,param_4);
        ExceptionList = local_c;
        return LVar5;
      }
      ExceptionList = local_c;
      return 0;
    }
  }
  else if (param_2 == 0x405) {
    if ((short)param_3 != 0x3e9) {
      return 0;
    }
    ExceptionList = &local_c;
    param_3 = IsChild((HWND)param_1[6],param_4);
    ppvVar3 = ExceptionList;
    goto joined_r0x0060c7c8;
  }
  ExceptionList = &local_c;
  LVar5 = FUN_006007b0(param_1,param_2,param_3,(LPARAM)param_4);
  ExceptionList = local_c;
  return LVar5;
}

