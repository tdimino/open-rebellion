
LRESULT FUN_006028c0(int *param_1,uint param_2,uint param_3,uint param_4)

{
  HWND pHVar1;
  undefined4 *puVar2;
  HDC pHVar3;
  HPALETTE hPal;
  HWND pHVar4;
  LRESULT LVar5;
  void *pvVar6;
  WPARAM wParam;
  undefined4 extraout_ECX;
  LPARAM lParam;
  tagPAINTSTRUCT local_4c;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_006562bb;
  local_c = ExceptionList;
  if (param_2 < 0x10) {
    if (param_2 == 0xf) {
      ExceptionList = &local_c;
      pHVar3 = BeginPaint((HWND)param_1[6],&local_4c);
      (**(code **)(*param_1 + 0x24))
                (pHVar3,CONCAT31((int3)((uint)extraout_ECX >> 8),(char)param_1[0x31]) & 0xffffff01);
      EndPaint((HWND)param_1[6],&local_4c);
      param_1[0x31] = param_1[0x31] & 0xfffeffff;
      ExceptionList = local_c;
      return 0;
    }
    if (param_2 == 1) {
      ExceptionList = &local_c;
      LVar5 = FUN_006007b0(param_1,1,param_3,param_4);
      pHVar3 = GetDC((HWND)param_1[6]);
      if (DAT_006be5a0 != (HPALETTE)0x0) {
        hPal = SelectPalette(pHVar3,DAT_006be5a0,0);
        RealizePalette(pHVar3);
        SelectPalette(pHVar3,hPal,0);
      }
      ReleaseDC((HWND)param_1[6],pHVar3);
      FUN_00601380((void *)param_1[8],(int)param_1);
      ExceptionList = local_c;
      return LVar5;
    }
    if (param_2 == 3) {
      param_1[0x43] = param_4 & 0xffff;
      param_1[0x44] = param_4 >> 0x10;
      return 0;
    }
LAB_00602afd:
    ExceptionList = &local_c;
    LVar5 = DefWindowProcA((HWND)param_1[6],param_2,param_3,param_4);
    ExceptionList = local_c;
    return LVar5;
  }
  if (param_2 < 0x15) {
    if (param_2 == 0x14) {
      return 1;
    }
    if (param_2 == 0x10) {
      ExceptionList = &local_c;
      if (param_1[0x40] != 0) {
        ExceptionList = &local_c;
        FUN_00602020(param_1[0x40]);
        if ((undefined4 *)param_1[0x40] != (undefined4 *)0x0) {
          (*(code *)**(undefined4 **)param_1[0x40])(1);
        }
        param_1[0x40] = 0;
      }
      if ((*(byte *)(param_1 + 0x31) & 1) == 0) {
        ExceptionList = local_c;
        return 0;
      }
      ReleaseCapture();
      if (DAT_006be5bc == (HWND)0x0) {
        ExceptionList = local_c;
        return 0;
      }
      FUN_00601080(DAT_006be5bc);
      DAT_006be5bc = (HWND)0x0;
      ExceptionList = local_c;
      return 0;
    }
    goto LAB_00602afd;
  }
  if (param_2 < 0x202) {
    if (param_2 == 0x201) {
LAB_00602b3c:
      ExceptionList = &local_c;
      FUN_006035f0(param_1,param_4);
      if ((param_1[0x31] & 0x10000U) != 0) {
        param_1[0x3e] = 0;
      }
      if (param_1[0x3e] == 0) {
        ExceptionList = local_c;
        return 0;
      }
      FUN_006030c0(param_1,1);
      DAT_006be5bc = (HWND)FUN_00601080((HWND)param_1[6]);
      InvalidateRect((HWND)param_1[6],(RECT *)0x0,0);
      PostMessageA(*(HWND *)(param_1[8] + 0x18),0x40e,param_2 << 0x10 | param_1[9] & 0xffffU,param_4
                  );
      if ((short)param_1[0x37] != 0) {
        FUN_00610c30((int)(param_1 + 0x38));
      }
      if (param_1[0x3f] == 0) {
        ExceptionList = local_c;
        return 0;
      }
      param_1[0x42] = 0;
      if (param_1[0x40] != 0) {
        FUN_00602020(param_1[0x40]);
        if ((undefined4 *)param_1[0x40] != (undefined4 *)0x0) {
          (*(code *)**(undefined4 **)param_1[0x40])(1);
        }
        param_1[0x40] = 0;
      }
      pvVar6 = (void *)FUN_00618b70(0x3c);
      uStack_4 = 0;
      if (pvVar6 != (void *)0x0) {
        pvVar6 = FUN_0060f1e0(pvVar6,param_1[0x3f],param_1);
        param_1[0x40] = (int)pvVar6;
        ExceptionList = local_c;
        return 0;
      }
      param_1[0x40] = 0;
      ExceptionList = local_c;
      return 0;
    }
    if (param_2 != 0x111) goto LAB_00602afd;
    if (param_3 != param_1[9]) {
      return 0;
    }
    ExceptionList = &local_c;
    FUN_006030f0(param_1,1);
    pHVar1 = (HWND)param_1[6];
    pHVar4 = GetCapture();
    if ((pHVar4 == pHVar1) && (ReleaseCapture(), DAT_006be5bc != (HWND)0x0)) {
      FUN_00601080(DAT_006be5bc);
      DAT_006be5bc = (HWND)0x0;
    }
    InvalidateRect((HWND)param_1[6],(RECT *)0x0,0);
    if (param_1[0x40] == 0) goto LAB_00602d08;
    FUN_00602020(param_1[0x40]);
    puVar2 = (undefined4 *)param_1[0x40];
  }
  else {
    if (param_2 != 0x202) {
      if (param_2 == 0x203) goto LAB_00602b3c;
      if (param_2 == 0x311) {
        if ((HWND)param_1[6] == (HWND)0x0) {
          return 0;
        }
        ExceptionList = &local_c;
        InvalidateRect((HWND)param_1[6],(RECT *)0x0,0);
        ExceptionList = local_c;
        return 0;
      }
      goto LAB_00602afd;
    }
    if (param_1[0x3e] == 0) {
      return 0;
    }
    pHVar1 = (HWND)param_1[6];
    ExceptionList = &local_c;
    pHVar4 = GetCapture();
    if (pHVar4 != pHVar1) {
      ExceptionList = local_c;
      return 0;
    }
    FUN_006030f0(param_1,1);
    pHVar1 = (HWND)param_1[6];
    pHVar4 = GetCapture();
    if (pHVar4 == pHVar1) {
      ReleaseCapture();
    }
    if (DAT_006be5bc != (HWND)0x0) {
      FUN_00601080(DAT_006be5bc);
      DAT_006be5bc = (HWND)0x0;
    }
    FUN_006035f0(param_1,param_4);
    if (param_1[0x3e] != 0) {
      if ((param_1[0x31] & 8U) != 0) {
        param_1[0x31] = param_1[0x31] ^ 4;
      }
      lParam = 0;
      pHVar1 = *(HWND *)(param_1[8] + 0x18);
      wParam = FUN_00602510((int)param_1);
      PostMessageA(pHVar1,0x111,wParam,lParam);
    }
    InvalidateRect((HWND)param_1[6],(RECT *)0x0,0);
    if (param_1[0x40] == 0) goto LAB_00602d08;
    FUN_00602020(param_1[0x40]);
    puVar2 = (undefined4 *)param_1[0x40];
  }
  if (puVar2 != (undefined4 *)0x0) {
    (**(code **)*puVar2)(1);
  }
  param_1[0x40] = 0;
LAB_00602d08:
  param_1[0x42] = 0;
  ExceptionList = local_c;
  return 0;
}

