
LRESULT FUN_00600310(HWND param_1,uint param_2,uint param_3,undefined4 *param_4)

{
  HWND pHVar1;
  bool bVar2;
  uint uVar3;
  undefined3 extraout_var;
  LRESULT LVar4;
  HDC hdc;
  HWND pHVar5;
  int *piVar6;
  BOOL BVar7;
  
  if ((((param_2 != 1) && (param_2 != 0x81)) && (param_2 != 0x220)) &&
     ((uVar3 = GetWindowLongA(param_1,0), uVar3 != 0 &&
      (bVar2 = FUN_006014b0(uVar3), CONCAT31(extraout_var,bVar2) == 0)))) {
    LVar4 = DefWindowProcA(param_1,param_2,param_3,(LPARAM)param_4);
    return LVar4;
  }
  if (0x1c < param_2) {
    if (param_2 == 0x82) {
      piVar6 = (int *)GetWindowLongA(param_1,0);
      if (piVar6 == (int *)0x0) {
        LVar4 = DefWindowProcA(param_1,0x82,param_3,(LPARAM)param_4);
        return LVar4;
      }
      LVar4 = (**(code **)(*piVar6 + 0x14))(piVar6,0x82,param_3,param_4);
      return LVar4;
    }
    if (param_2 == 0xa3) {
      piVar6 = (int *)GetWindowLongA(param_1,0);
      if (piVar6 == (int *)0x0) {
        return 0;
      }
      if (param_3 != 2) {
        LVar4 = DefWindowProcA(param_1,0xa3,param_3,(LPARAM)param_4);
        return LVar4;
      }
      if (((*(byte *)(piVar6 + 0x10) & 8) != 0) && (BVar7 = IsZoomed((HWND)piVar6[6]), BVar7 != 0))
      {
        return 0;
      }
      if (((*(byte *)(piVar6 + 0x10) & 4) != 0) && (BVar7 = IsIconic((HWND)piVar6[6]), BVar7 != 0))
      {
        return 0;
      }
      LVar4 = (**(code **)(*piVar6 + 0x14))(piVar6,0xa3,2,param_4);
      return LVar4;
    }
    if (param_2 == 0x112) {
      piVar6 = (int *)GetWindowLongA(param_1,0);
      if (piVar6 == (int *)0x0) {
        LVar4 = DefWindowProcA(param_1,0x112,param_3,(LPARAM)param_4);
        return LVar4;
      }
      uVar3 = param_3 & 0xfff0;
      if (uVar3 < 0xf011) {
        if (uVar3 == 0xf010) {
          if ((*(byte *)(piVar6 + 0x10) & 1) != 0) {
            return 0;
          }
          LVar4 = (**(code **)(*piVar6 + 0x14))(piVar6,0x112,param_3,param_4);
          return LVar4;
        }
        if (uVar3 == 0xf000) {
          if ((*(byte *)(piVar6 + 0x10) & 2) != 0) {
            return 0;
          }
          LVar4 = (**(code **)(*piVar6 + 0x14))(piVar6,0x112,param_3,param_4);
          return LVar4;
        }
      }
      else {
        if (uVar3 == 0xf020) {
          if ((*(byte *)(piVar6 + 0x10) & 4) != 0) {
            return 0;
          }
          LVar4 = (**(code **)(*piVar6 + 0x14))(piVar6,0x112,param_3,param_4);
          return LVar4;
        }
        if (uVar3 == 0xf030) {
          if ((*(byte *)(piVar6 + 0x10) & 8) != 0) {
            return 0;
          }
        }
        else if (uVar3 == 0xf120) {
          if (((*(byte *)(piVar6 + 0x10) & 4) != 0) &&
             (BVar7 = IsIconic((HWND)piVar6[6]), BVar7 != 0)) {
            return 0;
          }
          if (((*(byte *)(piVar6 + 0x10) & 8) != 0) &&
             (BVar7 = IsZoomed((HWND)piVar6[6]), BVar7 != 0)) {
            return 0;
          }
          LVar4 = (**(code **)(*piVar6 + 0x14))(piVar6,0x112,param_3,param_4);
          return LVar4;
        }
      }
      LVar4 = (**(code **)(*piVar6 + 0x14))(piVar6,0x112,param_3,param_4);
      return LVar4;
    }
switchD_00600382_caseD_4:
    piVar6 = (int *)GetWindowLongA(param_1,0);
    if (piVar6 != (int *)0x0) {
      LVar4 = (**(code **)(*piVar6 + 0x14))(piVar6,param_2,param_3,param_4);
      return LVar4;
    }
LAB_0060057a:
    LVar4 = DefWindowProcA(param_1,param_2,param_3,(LPARAM)param_4);
    return LVar4;
  }
  if (param_2 == 0x1c) {
    if (DAT_006be3b8 == (int *)0x0) {
      LVar4 = DefWindowProcA(param_1,0x1c,param_3,(LPARAM)param_4);
      return LVar4;
    }
    LVar4 = (**(code **)(*DAT_006be3b8 + 0x14))(DAT_006be3b8,0x1c,param_3,param_4);
    return LVar4;
  }
  switch(param_2) {
  case 1:
    piVar6 = *(int **)*param_4;
    FUN_00601420((uint)piVar6);
    SetWindowLongA(param_1,0,(LONG)piVar6);
    if (piVar6 == (int *)0x0) {
      LVar4 = DefWindowProcA(param_1,param_2,param_3,(LPARAM)param_4);
      return LVar4;
    }
    hdc = GetDC(param_1);
    PatBlt(hdc,0,0,piVar6[0xe],piVar6[0xf],0x42);
    ReleaseDC(param_1,hdc);
    piVar6[6] = (int)param_1;
    LVar4 = (**(code **)(*piVar6 + 0x14))(piVar6,param_2,param_3,param_4);
    return LVar4;
  case 2:
    piVar6 = (int *)GetWindowLongA(param_1,0);
    if (piVar6 != (int *)0x0) {
      LVar4 = (**(code **)(*piVar6 + 0x14))(piVar6,param_2,param_3,param_4);
      if (piVar6[0x21] != 0) {
        FUN_00600d30(piVar6,(int)piVar6);
      }
      if (((DAT_006be598 != 0) &&
          (pHVar1 = *(HWND *)(DAT_006be598 + 0x18), pHVar5 = GetCapture(), pHVar1 == pHVar5)) &&
         (*(int *)(DAT_006be598 + 0x18) == piVar6[6])) {
        ReleaseCapture();
      }
      FUN_00619730();
      return LVar4;
    }
    goto LAB_0060057a;
  case 3:
    piVar6 = (int *)GetWindowLongA(param_1,0);
    if (piVar6 != (int *)0x0) {
      piVar6[10] = (uint)param_4 & 0xffff;
      piVar6[0xb] = (uint)param_4 >> 0x10;
      LVar4 = (**(code **)(*piVar6 + 0x14))(piVar6,param_2,param_3,param_4);
      return LVar4;
    }
    break;
  default:
    goto switchD_00600382_caseD_4;
  case 5:
    piVar6 = (int *)GetWindowLongA(param_1,0);
    if (piVar6 != (int *)0x0) {
      piVar6[0xe] = (uint)param_4 & 0xffff;
      piVar6[0xf] = (uint)param_4 >> 0x10;
      LVar4 = (**(code **)(*piVar6 + 0x14))(piVar6,param_2,param_3,param_4);
      return LVar4;
    }
  }
  LVar4 = DefWindowProcA(param_1,param_2,param_3,(LPARAM)param_4);
  return LVar4;
}

