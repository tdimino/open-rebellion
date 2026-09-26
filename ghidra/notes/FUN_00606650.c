
UINT FUN_00606650(int *param_1,uint param_2,HPALETTE param_3,HWND param_4)

{
  HWND hWnd;
  int *this;
  bool bVar1;
  HDC pHVar2;
  undefined3 extraout_var;
  int *piVar3;
  undefined4 *puVar4;
  UINT UVar5;
  HPALETTE hPal;
  UINT unaff_retaddr;
  int local_50 [2];
  int local_48;
  undefined1 auStack_44 [60];
  UINT UStack_8;
  UINT UStack_4;
  
  this = param_1;
  param_1 = (int *)0x0;
  hWnd = (HWND)this[6];
  if (param_2 < 0x10) {
    if (param_2 == 0xf) {
      pHVar2 = BeginPaint(hWnd,(LPPAINTSTRUCT)(auStack_44 + 4));
      SelectPalette(pHVar2,DAT_006be5a0,0);
      FUN_00607710(this,pHVar2);
      (**(code **)(*this + 0x44))(pHVar2);
      SelectPalette(pHVar2,param_3,0);
      EndPaint(hWnd,(PAINTSTRUCT *)auStack_44);
      return unaff_retaddr;
    }
    if (param_2 == 1) {
      (**(code **)(*this + 0x34))();
      return 0;
    }
  }
  else {
    if (0x100 < param_2) {
      if (param_2 < 0x201) {
        if (param_2 != 0x200) {
          if (param_2 == 0x111) {
            (**(code **)(*this + 0x48))(param_3,param_3,param_4);
            return UStack_8;
          }
          goto LAB_00606864;
        }
        if ((((uint)param_3 & 1) != 0) && (this[0x38] != 0)) {
          piVar3 = (int *)FUN_00607370(local_50,param_4);
          FUN_006071e0(this,*piVar3,piVar3[1]);
          return 0;
        }
      }
      else if (param_2 < 0x216) {
        if (param_2 == 0x215) {
          if (param_4 == hWnd) {
            return 0;
          }
        }
        else {
          if (param_2 == 0x201) {
            if (this[0x38] != 0) {
              return 0;
            }
            puVar4 = (undefined4 *)FUN_00607370(&local_48,param_4);
            FUN_00606ee0(this,*puVar4,puVar4[1]);
            return 0;
          }
          if (param_2 != 0x202) goto LAB_00606864;
        }
        if (this[0x38] != 0) {
          FUN_00607010((int)this);
          return 0;
        }
      }
      else {
        if (param_2 != 0x30f) {
          if (param_2 != 0x311) goto LAB_00606864;
          InvalidateRect(hWnd,(RECT *)0x0,0);
          if (hWnd == (HWND)param_3) {
            return 0;
          }
        }
        pHVar2 = GetDC(hWnd);
        hPal = SelectPalette(pHVar2,DAT_006be5a0,0);
        param_1 = (int *)RealizePalette(pHVar2);
        SelectPalette(pHVar2,hPal,0);
        if (param_1 != (int *)0x0) {
          InvalidateRect(hWnd,(RECT *)0x0,0);
        }
        ReleaseDC(hWnd,pHVar2);
      }
      return (UINT)param_1;
    }
    if (param_2 == 0x100) {
      (**(code **)(*this + 0x4c))(param_3,param_4);
      return UStack_4;
    }
    if (param_2 == 0x18) {
      bVar1 = FUN_00607390(this,1);
      if (CONCAT31(extraout_var,bVar1) != 0) {
        SetWindowPos((HWND)this[6],(HWND)0x0,0,0,0,0,3);
      }
      UVar5 = FUN_006007b0(this,0x18,(uint)param_3,(LPARAM)param_4);
      return UVar5;
    }
  }
LAB_00606864:
  UVar5 = FUN_006007b0(this,param_2,(uint)param_3,(LPARAM)param_4);
  return UVar5;
}

