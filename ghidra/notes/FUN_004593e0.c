
UINT FUN_004593e0(int *param_1,uint param_2,HPALETTE param_3,HWND param_4)

{
  int *this;
  HDC hdc;
  HGDIOBJ h;
  int *piVar1;
  char *pcVar2;
  int iVar3;
  HWND pHVar4;
  undefined4 *puVar5;
  uint uVar6;
  DWORD lParam;
  UINT UVar7;
  HWND pHVar8;
  void *this_00;
  int iVar9;
  int X;
  tagPOINT local_c8;
  HWND local_c0;
  HWND local_bc;
  UINT local_b8;
  tagPOINT tStack_b4;
  undefined1 auStack_ac [4];
  undefined1 local_a8 [4];
  undefined1 auStack_a4 [4];
  HDC pHStack_a0;
  undefined1 auStack_9c [4];
  undefined1 auStack_98 [4];
  HPALETTE pHStack_94;
  undefined1 auStack_90 [60];
  int iStack_54;
  int iStack_50;
  tagPAINTSTRUCT local_4c;
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063074d;
  local_c = ExceptionList;
  iVar3 = param_1[0x65];
  pHVar4 = (HWND)param_1[6];
  local_bc = *(HWND *)(iVar3 + 0x18);
  local_b8 = 0;
  local_c0 = pHVar4;
  if (param_2 < 0x10) {
    if (param_2 == 0xf) {
      ExceptionList = &local_c;
      pHStack_a0 = BeginPaint(pHVar4,&local_4c);
      pHStack_94 = SelectPalette(pHStack_a0,DAT_006be5a0,0);
      param_1[0x50] = 0;
      FUN_005fcc30(*(void **)(param_1[0x65] + 0x45c),(uint *)param_1[0x29],0,0,param_1[10],
                   param_1[0xb],(undefined4 *)param_1[0xe],param_1[0xf]);
      FUN_005fe050((void *)param_1[0x29],0,0,(short)param_1[0xe],(short)param_1[0xf]);
      FUN_005fd0f0((void *)param_1[0x2a],(uint *)param_1[0x29],0,0);
      (**(code **)(param_1[0x59] + 8))();
      h = (HGDIOBJ)FUN_005fc130(param_1[0x29]);
      local_bc = (HWND)SelectObject((HDC)param_1[0x58],h);
      FUN_00601880(auStack_90,(char *)0x0,0,0,100,0,1,1);
      local_4 = 0;
      FUN_00601c60(auStack_90,10);
      FUN_00403e90(auStack_90,1);
      piVar1 = (int *)(**(code **)(param_1[0x59] + 8))();
      while (piVar1 != (int *)0x0) {
        this = (int *)piVar1[9];
        if (this != (int *)0x0) {
          local_c8.x = piVar1[10];
          local_c8.y = piVar1[0xb];
          FUN_005fd0f0(this,(uint *)param_1[0x29],local_c8.x,local_c8.y);
          pcVar2 = (char *)FUN_00583c40((int)(piVar1 + 5));
          FUN_00601aa0(auStack_90,pcVar2);
          FUN_00601c90(auStack_90,piVar1[0x18],(HWND)0x0);
          iVar3 = FUN_005fc0e0(this);
          local_c8.x = (local_c8.x + iVar3 / 2) - 0x32;
          local_c8.y = local_c8.y + piVar1[0xd];
          FUN_00601b30(auStack_90,local_c8.x,local_c8.y);
          FUN_00601ce0(auStack_90,(HDC)param_1[0x58]);
        }
        piVar1 = (int *)(**(code **)(*piVar1 + 0xc))();
        pHVar4 = local_c0;
      }
      SelectObject((HDC)param_1[0x58],local_bc);
      FUN_004ece30(&tStack_b4.x);
      local_4 = CONCAT31(local_4._1_3_,1);
      piVar1 = (int *)(**(code **)(param_1[0x5d] + 8))();
      hdc = pHStack_a0;
      while (pHStack_a0 = hdc, piVar1 != (int *)0x0) {
        if ((piVar1[0xf] & 2U) != 0) {
          this_00 = (void *)piVar1[8];
          if (((piVar1[0xf] & 1U) != 0) && ((void *)piVar1[9] != (void *)0x0)) {
            this_00 = (void *)piVar1[9];
          }
          if (this_00 != (void *)0x0) {
            iStack_54 = piVar1[0x12];
            iStack_50 = piVar1[0x13];
            FUN_005fd0f0(this_00,(uint *)param_1[0x29],piVar1[0x10],piVar1[0x11]);
            pHVar4 = local_c0;
          }
        }
        piVar1 = (int *)(**(code **)(*piVar1 + 0xc))();
        hdc = pHStack_a0;
      }
      FUN_005fc140((void *)param_1[0x29],hdc,'@',0xcc0020,0,0,0,0,0,0);
      FUN_00601ce0((void *)param_1[0x49],hdc);
      SelectPalette(hdc,pHStack_94,0);
      EndPaint(pHVar4,&local_4c);
      for (pHVar4 = GetWindow(pHVar4,5); pHVar4 != (HWND)0x0; pHVar4 = GetWindow(pHVar4,2)) {
        UpdateWindow(pHVar4);
      }
      local_4 = local_4 & 0xffffff00;
      FUN_00619730();
      local_4 = 0xffffffff;
      FUN_00601990((int)auStack_90);
      ExceptionList = local_c;
      return local_b8;
    }
    if (param_2 == 2) {
      if ((HDC)param_1[0x58] == (HDC)0x0) {
        return 0;
      }
      ExceptionList = &local_c;
      DeleteDC((HDC)param_1[0x58]);
      ExceptionList = local_c;
      return local_b8;
    }
switchD_00459879_default:
    ExceptionList = &local_c;
    UVar7 = FUN_004ac3a0(param_1,param_2,param_3,param_4);
    ExceptionList = local_c;
    return UVar7;
  }
  if (param_2 < 0x112) {
    if (param_2 == 0x111) {
      if (((uint)param_3 & 0xffff) == 0x138) {
        ExceptionList = &local_c;
        (**(code **)(*param_1 + 0x30))();
        ExceptionList = local_c;
        return local_b8;
      }
      if (((uint)param_3 & 0xffff) != 0x139) {
        ExceptionList = &local_c;
        UVar7 = FUN_004ac3a0(param_1,0x111,param_3,param_4);
        ExceptionList = local_c;
        return UVar7;
      }
      X = param_1[10];
      iVar9 = param_1[0xb];
      if (*(int *)(iVar3 + 0x9c) == 1) {
        X = (-(uint)(X != 0x3c) & 0xffffff10) + 300;
      }
      else if (*(int *)(iVar3 + 0x9c) == 2) {
        X = (-(uint)(X != 0x78) & 0xffffff0b) + 0x16d;
      }
      ExceptionList = &local_c;
      iVar3 = FUN_005f5060(iVar3 + 0x6c);
      do {
        if (iVar3 == 0) {
LAB_00459814:
          (**(code **)(*param_1 + 8))();
          pHVar4 = local_c0;
          SetWindowPos(local_c0,(HWND)0x0,X,iVar9,param_1[0xc],param_1[0xd],0x305);
          (**(code **)(*param_1 + 0x2c))(5);
          UpdateWindow(pHVar4);
          ExceptionList = local_c;
          return local_b8;
        }
        if ((((byte)*(int *)(iVar3 + 0x24) & 0x3f) == 1) && (*(int *)(iVar3 + 0x24) != param_1[9]))
        {
          X = *(int *)(iVar3 + 0x28);
          iVar9 = *(int *)(iVar3 + 0x2c);
          SendMessageA(local_bc,0x405,*(WPARAM *)(iVar3 + 0x24),0);
          goto LAB_00459814;
        }
        iVar3 = *(int *)(iVar3 + 0x10);
      } while( true );
    }
    if (param_2 == 0x100) {
      ExceptionList = &local_c;
      (**(code **)(*(int *)param_1[8] + 0x14))((int *)param_1[8],0x100,param_3,param_4);
      ExceptionList = local_c;
      return local_b8;
    }
    goto switchD_00459879_default;
  }
  if (param_2 < 0x216) {
    if (param_2 != 0x215) {
      switch(param_2) {
      case 0x200:
        local_c8.y = (uint)param_4 >> 0x10;
        local_c8.x = (uint)param_4 & 0xffff;
        if (param_1[0x50] != 0) {
          return 0;
        }
        ExceptionList = &local_c;
        pHVar8 = GetCapture();
        if ((pHVar8 == pHVar4) && (((uint)param_3 & 1) != 0)) {
          if (param_1[0x52] != 0) goto LAB_00459a59;
          if (param_1[0x68] != 0) {
            param_1[0x52] = 1;
            UpdateWindow(local_c0);
            if ((param_1[0x53] == -1000) || (param_1[0x54] == -1000)) {
              param_1[0x53] = local_c8.x;
              param_1[0x54] = local_c8.y;
            }
            if ((((uint)param_3 & 8) != 0) &&
               (piVar1 = FUN_0045cc10(param_1 + 0x5d,param_1[0x53],param_1[0x54]),
               piVar1 != (int *)0x0)) {
              puVar5 = (undefined4 *)FUN_004f26d0(auStack_a4,piVar1 + 0xf);
              local_4 = 2;
              uVar6 = FUN_005f4960(puVar5);
              local_4 = 0xffffffff;
              FUN_00619730();
              if ((uVar6 & 1) == 0) {
                FUN_0045b1b0(param_1,piVar1);
                param_1[0x68] = (int)piVar1;
              }
            }
            puVar5 = (undefined4 *)FUN_004f26d0(auStack_ac,(undefined4 *)(param_1[0x68] + 0x3c));
            local_4 = 3;
            uVar6 = FUN_005f4960(puVar5);
            local_4 = 0xffffffff;
            FUN_00619730();
            if ((uVar6 & 1) != 0) {
              FUN_0060dc80((undefined4 *)param_1[0x66]);
              FUN_0060dbb0((void *)param_1[0x66],(uint *)param_1[0x67]);
              FUN_0060dcb0((void *)param_1[0x66],param_1[0x53],param_1[0x54]);
              tStack_b4.x = param_1[0x53];
              tStack_b4.y = param_1[0x54];
              MapWindowPoints(local_c0,local_bc,&tStack_b4,1);
              FUN_0060dce0((undefined4 *)param_1[0x66]);
              ExceptionList = local_c;
              return local_b8;
            }
            goto LAB_00459ce3;
          }
        }
        if (param_1[0x52] == 0) {
          ExceptionList = local_c;
          return local_b8;
        }
LAB_00459a59:
        MapWindowPoints(local_c0,local_bc,&local_c8,1);
        FUN_0060df00(&local_c8);
        ExceptionList = local_c;
        return local_b8;
      case 0x201:
      case 0x204:
        if (param_1[0x50] != 0) {
          return 0;
        }
        local_c8.x = (uint)param_4 & 0xffff;
        local_c8.y = (uint)param_4 >> 0x10;
        ExceptionList = &local_c;
        pHVar4 = GetCapture();
        if ((pHVar4 != local_c0) && (param_2 == 0x201)) {
          FUN_00601080(local_c0);
          param_1[0x53] = local_c8.x;
          param_1[0x54] = local_c8.y;
        }
        piVar1 = FUN_0045cc10(param_1 + 0x5d,local_c8.x,local_c8.y);
        param_1[0x68] = (int)piVar1;
        if (piVar1 == (int *)0x0) {
          ExceptionList = local_c;
          return local_b8;
        }
        if (param_1[0x52] == 0) {
          puVar5 = (undefined4 *)FUN_004f26d0(auStack_9c,piVar1 + 0xf);
          local_4 = 4;
          uVar6 = FUN_005f4960(puVar5);
          local_4 = 0xffffffff;
          FUN_00619730();
          if (param_1[0x68] == 0) {
            if (((uint)param_3 & 8) == 0) {
              FUN_0045afc0(param_1,(void *)0x0);
            }
          }
          else if (((uVar6 & 1) == 0) && (((uint)param_3 & 8) == 0)) {
            FUN_0045afc0(param_1,(void *)0x0);
            FUN_0045b1b0(param_1,(void *)param_1[0x68]);
          }
        }
        if (param_2 == 0x201) {
          ExceptionList = local_c;
          return local_b8;
        }
        UVar7 = FUN_004ac3a0(param_1,param_2,param_3,param_4);
        ExceptionList = local_c;
        return UVar7;
      case 0x202:
        local_c8.x = (uint)param_4 & 0xffff;
        local_c8.y = (uint)param_4 >> 0x10;
        if (param_1[0x52] == 0) {
          ExceptionList = &local_c;
          pHVar8 = GetCapture();
          if (((pHVar8 == pHVar4) && (param_1[0x68] != 0)) && (((uint)param_3 & 8) != 0)) {
            puVar5 = (undefined4 *)FUN_004f26d0(auStack_98,(undefined4 *)(param_1[0x68] + 0x3c));
            local_4 = 5;
            uVar6 = FUN_005f4960(puVar5);
            local_4 = 0xffffffff;
            FUN_00619730();
            if ((uVar6 & 1) == 0) {
              FUN_0045b1b0(param_1,(void *)param_1[0x68]);
            }
            else {
              FUN_0045afc0(param_1,(void *)param_1[0x68]);
              param_1[0x68] = 0;
            }
          }
        }
        else {
          iVar3 = param_1[0x53];
          if (iVar3 < local_c8.x) {
            iVar3 = local_c8.x - iVar3;
          }
          else {
            iVar3 = iVar3 - local_c8.x;
          }
          iVar9 = param_1[0x54];
          if (iVar9 < local_c8.y) {
            iVar9 = local_c8.y - iVar9;
          }
          else {
            iVar9 = iVar9 - local_c8.y;
          }
          if ((4 < iVar3) || (ExceptionList = &local_c, 4 < iVar9)) {
            uVar6 = param_1[9];
            ExceptionList = &local_c;
            lParam = GetMessagePos();
            PostMessageA(*(HWND *)(param_1[0x65] + 0x18),0x111,uVar6 & 0xffff | 0x29a0000,lParam);
          }
        }
        pHVar4 = GetCapture();
        if (pHVar4 != local_c0) {
          ExceptionList = local_c;
          return local_b8;
        }
        ReleaseCapture();
        if (param_1[0x52] == 0) {
          ExceptionList = local_c;
          return local_b8;
        }
        FUN_0060de50();
LAB_00459ce3:
        param_1[0x52] = 0;
        ExceptionList = local_c;
        return local_b8;
      case 0x203:
        local_c8.x = (uint)param_4 & 0xffff;
        local_c8.y = (uint)param_4 >> 0x10;
        if (param_1[0x50] != 0) {
          return 0;
        }
        ExceptionList = &local_c;
        piVar1 = FUN_0045cc10(param_1 + 0x5d,local_c8.x,local_c8.y);
        if (piVar1 == (int *)0x0) {
          ExceptionList = local_c;
          return local_b8;
        }
        puVar5 = (undefined4 *)FUN_004f26d0(local_a8,piVar1 + 0xf);
        local_4 = 6;
        uVar6 = FUN_005f4960(puVar5);
        local_4 = 0xffffffff;
        FUN_00619730();
        if ((uVar6 & 2) != 0) {
          FUN_0045afc0(param_1,(void *)0x0);
          FUN_0045b1b0(param_1,piVar1);
          FUN_0045aac0(param_1,piVar1,local_c8.x,local_c8.y);
        }
        pHVar4 = GetCapture();
        if (pHVar4 != local_c0) {
          ExceptionList = local_c;
          return local_b8;
        }
        break;
      default:
        goto switchD_00459879_default;
      }
      goto LAB_00459ded;
    }
  }
  else if (param_2 != 0x40d) {
    if (param_2 == 0x40e) {
      return 0;
    }
    goto switchD_00459879_default;
  }
  ExceptionList = &local_c;
  if (param_1[0x52] != 0) {
    ExceptionList = &local_c;
    FUN_0060de50();
    param_1[0x52] = 0;
  }
  pHVar8 = GetCapture();
  if (pHVar8 != pHVar4) {
    ExceptionList = local_c;
    return local_b8;
  }
LAB_00459ded:
  ReleaseCapture();
  ExceptionList = local_c;
  return local_b8;
}

