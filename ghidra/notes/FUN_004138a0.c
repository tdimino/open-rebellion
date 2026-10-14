
LRESULT FUN_004138a0(int param_1,uint param_2,HWND param_3,undefined4 *param_4)

{
  HWND pHVar1;
  int *piVar2;
  HWND wParam;
  undefined4 *puVar3;
  LRESULT LVar4;
  undefined4 uVar5;
  BOOL BVar6;
  int iVar7;
  HDC pHVar8;
  void *pvStack_c;
  undefined1 *puStack_8;
  int iStack_4;
  
  puVar3 = param_4;
  wParam = param_3;
  iStack_4 = 0xffffffff;
  puStack_8 = &LAB_0062aa23;
  pvStack_c = ExceptionList;
  pHVar1 = *(HWND *)(param_1 + 0x18);
  if (param_2 < 8) {
    if (param_2 == 7) {
      if (*(int *)(param_1 + 0xc0) == 0) {
        return 0;
      }
      if ((*(uint *)(param_1 + 0xb4) & 0x10000000) != 0) {
        return 0;
      }
      ExceptionList = &pvStack_c;
      SetFocus(*(HWND *)(*(int *)(param_1 + 0xc0) + 0x18));
      ExceptionList = pvStack_c;
      return 0;
    }
    if (param_2 == 1) {
      ExceptionList = &pvStack_c;
      Ordinal_17();
      FUN_006008e0(500);
      FUN_00614200(pHVar1,1);
      if ((*(byte *)(param_1 + 0x128) & 1) != 0) {
        FUN_00614360(0);
      }
      param_3 = (HWND)CONCAT22(DAT_0065d420,0x1000);
      FUN_005f2fc0(&param_3);
      iStack_4 = 0;
      uVar5 = FUN_00583c40();
      FUN_005fff30(uVar5);
      param_4 = (undefined4 *)FUN_00618b70(0x14);
      iStack_4._0_1_ = 1;
      if (param_4 == (undefined4 *)0x0) {
        iVar7 = 0;
      }
      else {
        iVar7 = FUN_005f68d0(param_1);
      }
      iStack_4 = (uint)iStack_4._1_3_ << 8;
      if (iVar7 == 0) {
        FUN_0060a9e0(s_Start_Of_Intro_Music_Failed__006a8328,s_Application_SoundManager__006a8348);
      }
      DAT_006b1468 = 1;
      iStack_4 = 0xffffffff;
      FUN_005f2ff0();
      ExceptionList = pvStack_c;
      return 0;
    }
    if (param_2 == 2) {
LAB_00413a60:
      ExceptionList = &pvStack_c;
      iVar7 = FUN_00421380();
      if (iVar7 != 0) {
        ExceptionList = pvStack_c;
        return 0;
      }
      FUN_00414330(0xd);
      ExceptionList = pvStack_c;
      return 0;
    }
  }
  else if (param_2 < 0x11) {
    if (param_2 == 0x10) goto LAB_00413a60;
    if (param_2 == 8) {
      if (*(int *)(param_1 + 0xc0) == 0) {
        return 0;
      }
      if ((*(uint *)(param_1 + 0xb4) & 0x10000000) != 0) {
        return 0;
      }
      ExceptionList = &pvStack_c;
      BVar6 = IsChild(pHVar1,param_3);
      if (BVar6 == 0) {
        ExceptionList = pvStack_c;
        return 0;
      }
      pHVar1 = *(HWND *)(*(int *)(param_1 + 0xc0) + 0x18);
      if (wParam == pHVar1) {
        ExceptionList = pvStack_c;
        return 0;
      }
      SetFocus(pHVar1);
      ExceptionList = pvStack_c;
      return 0;
    }
  }
  else if (param_2 < 0x105) {
    if (param_2 == 0x104) {
      return 0;
    }
    if (param_2 == 0x1c) {
      if (param_3 == (HWND)0x0) {
        ExceptionList = &pvStack_c;
        *(uint *)(param_1 + 0xb4) = *(uint *)(param_1 + 0xb4) | 0x10000000;
        if ((*(byte *)(param_1 + 0x128) & 1) == 0) {
          SetFocus(pHVar1);
          FUN_00615110();
          ShowWindow(*(HWND *)(param_1 + 0x18),6);
        }
        if (*(int *)(param_1 + 0xc0) != 0) {
          SendMessageA(*(HWND *)(*(int *)(param_1 + 0xc0) + 0x18),0x479,0,0);
        }
      }
      else {
        ExceptionList = &pvStack_c;
        *(uint *)(param_1 + 0xb4) = *(uint *)(param_1 + 0xb4) & 0xefffffff;
        if ((*(byte *)(param_1 + 0x128) & 1) == 0) {
          if (*(int **)(param_1 + 0xc0) != (int *)0x0) {
            (**(code **)(**(int **)(param_1 + 0xc0) + 8))();
          }
          FUN_00614310(0);
          FUN_006143a0(0x280,0x1e0,8,0);
          FUN_00614360(0);
          FUN_00615150();
        }
        iVar7 = FUN_006001f0();
        if (iVar7 != 0) {
          InvalidateRect(*(HWND *)(param_1 + 0x18),(RECT *)0x0,0);
        }
        FUN_00601080(0);
        if (*(int *)(param_1 + 0xc4) == 0) {
          if (*(int *)(param_1 + 0xc0) != 0) {
            SetFocus(*(HWND *)(*(int *)(param_1 + 0xc0) + 0x18));
          }
          if (*(int *)(param_1 + 0xc0) != 0) {
            SendMessageA(*(HWND *)(*(int *)(param_1 + 0xc0) + 0x18),0x47a,0,0);
            if (*(int **)(param_1 + 0xc0) != (int *)0x0) {
              (**(code **)(**(int **)(param_1 + 0xc0) + 4))(5);
            }
          }
        }
        else {
          SetFocus(*(HWND *)(*(int *)(param_1 + 0xc4) + 0x18));
        }
        thunk_FUN_00416f70();
        if (DAT_006b1468 != 0) {
          iVar7 = FUN_00597480();
          if (iVar7 == 0) {
            uVar5 = 1;
          }
          else {
            iVar7 = FUN_00401440();
            FUN_005ff020(iVar7,s_textcomm_dll_006a7edc,1);
            FUN_005ff020(iVar7,s_common_dll_006a7c98,8);
            if (iVar7 != 0) {
              FUN_00618b60(iVar7);
            }
            uVar5 = 5;
          }
          FUN_00414330(uVar5);
          DAT_006b1468 = 0;
          LVar4 = DefWindowProcA(pHVar1,0x1c,(WPARAM)wParam,(LPARAM)param_4);
          ExceptionList = pvStack_c;
          return LVar4;
        }
      }
      LVar4 = DefWindowProcA(pHVar1,0x1c,(WPARAM)wParam,(LPARAM)param_4);
      ExceptionList = pvStack_c;
      return LVar4;
    }
  }
  else if (param_2 < 0x310) {
    if (param_2 == 0x30f) {
      ExceptionList = &pvStack_c;
      pHVar8 = GetDC(pHVar1);
      SetSystemPaletteUse(pHVar8,2);
      SetSystemPaletteUse(pHVar8,1);
      piVar2 = *(int **)(param_1 + 0xc0);
      if (piVar2 != (int *)0x0) {
        (**(code **)(*piVar2 + 0x14))(piVar2,0x30f,param_3,param_4);
      }
      InvalidateRect(*(HWND *)(param_1 + 0x18),(RECT *)0x0,0);
      FUN_00600010(pHVar8,DAT_006be5a0);
      ReleaseDC(*(HWND *)(param_1 + 0x18),pHVar8);
      ExceptionList = pvStack_c;
      return 1;
    }
    if (param_2 == 0x111) {
      uVar5 = 0x111;
      if ((3000 < ((uint)param_3 & 0xffff)) && (((uint)param_3 & 0xffff) < 0xbd8)) {
        uVar5 = 0x483;
      }
      piVar2 = *(int **)(param_1 + 0xc0);
      if (piVar2 == (int *)0x0) {
        return 0;
      }
      ExceptionList = &pvStack_c;
      LVar4 = (**(code **)(*piVar2 + 0x14))(piVar2,uVar5,param_3,param_4);
      ExceptionList = pvStack_c;
      return LVar4;
    }
    if (param_2 == 0x112) {
      if (((param_3 == (HWND)0xf100) && (*(int *)(param_1 + 0xc0) != 0)) &&
         (*(int *)(param_1 + 0xc0) == *(int *)(param_1 + 0xbc))) {
        ExceptionList = &pvStack_c;
        SendMessageA(*(HWND *)(*(int *)(param_1 + 0xbc) + 0x18),0x100,0x79,0x79);
        ExceptionList = pvStack_c;
        return 0;
      }
      param_2 = 0x112;
      goto LAB_004138f1;
    }
  }
  else if (param_2 < 0x406) {
    if (param_2 == 0x405) {
      if (param_3 == (HWND)0xd) {
        ExceptionList = &pvStack_c;
        thunk_FUN_00416d90();
        ExceptionList = pvStack_c;
        return 0;
      }
      if (param_3 != (HWND)0xf) {
        return 0;
      }
      ExceptionList = &pvStack_c;
      if (*(int **)(param_1 + 0xc4) != (int *)0x0) {
        ExceptionList = &pvStack_c;
        (**(code **)(**(int **)(param_1 + 0xc4) + 8))();
        DestroyWindow(*(HWND *)(*(int *)(param_1 + 0xc4) + 0x18));
        if (*(undefined4 **)(param_1 + 0xc4) != (undefined4 *)0x0) {
          (**(code **)**(undefined4 **)(param_1 + 0xc4))(1);
        }
        *(undefined4 *)(param_1 + 0xc4) = 0;
      }
      puVar3 = param_4;
      switch(param_4) {
      case (undefined4 *)0x65:
        FUN_00414330(2);
        ExceptionList = pvStack_c;
        return 0;
      case (undefined4 *)0x66:
      case (undefined4 *)0x6a:
      case (undefined4 *)0x6b:
      case (undefined4 *)0x6c:
      case (undefined4 *)0x6d:
        iVar7 = ShowCursor(1);
        if (iVar7 < 0) {
          do {
            iVar7 = ShowCursor(1);
          } while (iVar7 < 0);
          FUN_00414330(3);
          ExceptionList = pvStack_c;
          return 0;
        }
        goto LAB_0041413d;
      case (undefined4 *)0x67:
        FUN_00414330(7);
        ExceptionList = pvStack_c;
        return 0;
      case (undefined4 *)0x68:
      case (undefined4 *)0x69:
        goto switchD_00413e03_caseD_68;
      }
      if (*(int *)(param_1 + 0xc0) == 0) {
        ExceptionList = pvStack_c;
        return 0;
      }
      pHVar8 = GetDC(*(HWND *)(param_1 + 0x18));
      if (*(int *)(param_1 + 0xc0) == *(int *)(param_1 + 0xb8)) {
        FUN_005ff440();
        uVar5 = FUN_005fc3f0();
      }
      else {
        if (*(int *)(param_1 + 0xc0) != *(int *)(param_1 + 0xbc)) goto LAB_00413eaf;
        FUN_005ff440();
        uVar5 = FUN_005fc3f0();
      }
      FUN_00600010(pHVar8,uVar5);
LAB_00413eaf:
      ReleaseCapture();
      if (*(int **)(param_1 + 0xc0) != (int *)0x0) {
        (**(code **)(**(int **)(param_1 + 0xc0) + 4))(5);
      }
      ReleaseDC(*(HWND *)(param_1 + 0x18),pHVar8);
      if (*(int *)(param_1 + 0xc0) == 0) {
        ExceptionList = pvStack_c;
        return 0;
      }
      PostMessageA(*(HWND *)(*(int *)(param_1 + 0xc0) + 0x18),0x402,0,(LPARAM)puVar3);
      ExceptionList = pvStack_c;
      return 0;
    }
    if (param_2 == 0x311) {
      ExceptionList = &pvStack_c;
      FUN_00421360(param_3);
      iVar7 = FUN_006001f0();
      if (iVar7 == 0) {
        ExceptionList = pvStack_c;
        return 0;
      }
      InvalidateRect(*(HWND *)(param_1 + 0x18),(RECT *)0x0,0);
      ExceptionList = pvStack_c;
      return 0;
    }
  }
  else {
    switch(param_2) {
    case 0x40c:
      if ((*(int *)(param_1 + 0xdc) != 4) && (*(int *)(param_1 + 0xdc) != 5)) {
        return 0;
      }
      if (*(int *)(param_1 + 0xc0) == 0) {
        return 0;
      }
      ExceptionList = &pvStack_c;
      SendMessageA(*(HWND *)(*(int *)(param_1 + 0xc0) + 0x18),param_2,(WPARAM)param_3,
                   (LPARAM)param_4);
      ExceptionList = pvStack_c;
      return 0;
    case 0x465:
      if (param_3 != (HWND)0xf) {
        return 0;
      }
      ExceptionList = &pvStack_c;
      if (*(int **)(param_1 + 0xc4) != (int *)0x0) {
        ExceptionList = &pvStack_c;
        (**(code **)(**(int **)(param_1 + 0xc4) + 8))();
        DestroyWindow(*(HWND *)(*(int *)(param_1 + 0xc4) + 0x18));
        if (*(undefined4 **)(param_1 + 0xc4) != (undefined4 *)0x0) {
          (**(code **)**(undefined4 **)(param_1 + 0xc4))(1);
        }
        *(undefined4 *)(param_1 + 0xc4) = 0;
      }
      puVar3 = param_4;
      switch(param_4) {
      case (undefined4 *)0x65:
      case (undefined4 *)0x66:
      case (undefined4 *)0x6a:
      case (undefined4 *)0x6b:
      case (undefined4 *)0x6c:
      case (undefined4 *)0x6d:
        iVar7 = ShowCursor(1);
        if (iVar7 < 0) {
          do {
            iVar7 = ShowCursor(1);
          } while (iVar7 < 0);
          FUN_00414330(3);
          ExceptionList = pvStack_c;
          return 0;
        }
        goto LAB_0041413d;
      case (undefined4 *)0x67:
      case (undefined4 *)0x68:
      case (undefined4 *)0x69:
switchD_00413e03_caseD_68:
        FUN_00414330(8);
        ExceptionList = pvStack_c;
        return 0;
      }
      if (*(int *)(param_1 + 0xc0) == 0) {
        ExceptionList = pvStack_c;
        return 0;
      }
      pHVar8 = GetDC(*(HWND *)(param_1 + 0x18));
      if (*(int *)(param_1 + 0xc0) == *(int *)(param_1 + 0xb8)) {
        FUN_005ff440();
        uVar5 = FUN_005fc3f0();
      }
      else {
        if (*(int *)(param_1 + 0xc0) != *(int *)(param_1 + 0xbc)) goto LAB_00414011;
        FUN_005ff440();
        uVar5 = FUN_005fc3f0();
      }
      FUN_00600010(pHVar8,uVar5);
LAB_00414011:
      ReleaseCapture();
      if (*(int **)(param_1 + 0xc0) != (int *)0x0) {
        (**(code **)(**(int **)(param_1 + 0xc0) + 4))(5);
      }
      ReleaseDC(*(HWND *)(param_1 + 0x18),pHVar8);
      if (*(int *)(param_1 + 0xc0) == 0) {
        ExceptionList = pvStack_c;
        return 0;
      }
      PostMessageA(*(HWND *)(*(int *)(param_1 + 0xc0) + 0x18),0x402,0,(LPARAM)puVar3);
      ExceptionList = pvStack_c;
      return 0;
    case 0x469:
      if (param_4 == (undefined4 *)0x0) {
        return 0;
      }
      ExceptionList = &pvStack_c;
      DestroyWindow((HWND)param_4[6]);
      (**(code **)*puVar3)(1);
      ExceptionList = pvStack_c;
      return 0;
    case 0x46a:
      switch(param_3) {
      case (HWND)0x46c:
        if (*(int *)(param_1 + 0xe4) != 2) {
          if (*(int *)(param_1 + 0xe4) != 1) {
            return 0;
          }
          ExceptionList = &pvStack_c;
          uVar5 = FUN_0041cdb0();
          FUN_0041d310(uVar5);
          ExceptionList = pvStack_c;
          return 0;
        }
        if (*(int *)(param_1 + 0xdc) != 8) {
          return 0;
        }
        ExceptionList = &pvStack_c;
        uVar5 = FUN_0041cdb0();
        FUN_0041d310(uVar5);
        if (*(int *)(*(int *)(param_1 + 0xb8) + 0xac) != 0) {
          ExceptionList = pvStack_c;
          return 0;
        }
        iVar7 = FUN_0041cf80();
        if (((iVar7 != 0) || (iVar7 = FUN_0041dc10(), iVar7 == 0)) &&
           (iVar7 = FUN_0041dc30(), iVar7 == 0)) {
          ExceptionList = pvStack_c;
          return 0;
        }
        FUN_00401200(0,0x1206,500,0x50);
        ExceptionList = pvStack_c;
        return 0;
      case (HWND)0x46d:
      case (HWND)0x47c:
      case (HWND)0x47d:
        ExceptionList = &pvStack_c;
        FUN_005974c0();
        break;
      default:
        return 0;
      case (HWND)0x470:
        ExceptionList = &pvStack_c;
        FUN_00414330(0xe);
        ExceptionList = pvStack_c;
        return 0;
      }
LAB_0041413d:
      FUN_00414330(3);
      ExceptionList = pvStack_c;
      return 0;
    case 0x46b:
      if (param_3 == (HWND)0x47c) {
        ExceptionList = &pvStack_c;
        FUN_0041dbf0();
        ExceptionList = pvStack_c;
        return 0;
      }
      if (param_3 != (HWND)0x47d) {
        return 0;
      }
      ExceptionList = &pvStack_c;
      FUN_0041dbf0();
      ExceptionList = pvStack_c;
      return 0;
    case 0x479:
    case 0x47a:
      if (*(int *)(param_1 + 0xc0) == 0) {
        return 0;
      }
      ExceptionList = &pvStack_c;
      SendMessageA(*(HWND *)(*(int *)(param_1 + 0xc0) + 0x18),param_2,0,0);
      ExceptionList = pvStack_c;
      return 0;
    }
  }
LAB_004138f1:
  ExceptionList = &pvStack_c;
  LVar4 = FUN_006007b0(param_1,param_2,param_3,param_4);
  ExceptionList = pvStack_c;
  return LVar4;
}

