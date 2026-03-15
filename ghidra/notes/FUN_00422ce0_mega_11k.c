
HGDIOBJ FUN_00422ce0(int *param_1,uint param_2,HWND param_3,uint *param_4)

{
  LONG LVar1;
  POINT pt;
  POINT pt_00;
  POINT pt_01;
  POINT pt_02;
  POINT pt_03;
  POINT pt_04;
  POINT pt_05;
  SHORT SVar2;
  ushort uVar3;
  short sVar4;
  undefined4 uVar5;
  int iVar6;
  HDC pHVar7;
  int *piVar8;
  HPALETTE pHVar9;
  HDC pHVar10;
  void *pvVar11;
  uint *puVar12;
  HCURSOR pHVar13;
  uint uVar14;
  HGDIOBJ h;
  char *pcVar15;
  HWND pHVar16;
  int *piVar17;
  undefined4 *puVar18;
  BOOL BVar19;
  HWND pHVar20;
  HWND pHVar21;
  undefined4 *puVar22;
  undefined4 uVar23;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  uint uVar24;
  undefined4 unaff_EBX;
  RECT *lprc;
  int *this;
  bool bVar25;
  bool bVar26;
  int iVar27;
  int iVar28;
  HDC *ppHVar29;
  int iVar30;
  tagRECT *ptVar31;
  HWND local_108;
  void *local_104;
  tagPOINT local_100;
  HDC local_f8 [2];
  void *local_f0;
  HMODULE local_ec;
  tagRECT local_e8;
  HGDIOBJ local_d8 [2];
  tagMSG local_d0;
  tagRECT local_b4;
  uint local_a4;
  tagRECT local_9c;
  undefined2 uStack_8c;
  undefined2 uStack_8a;
  undefined2 uStack_88;
  undefined2 uStack_86;
  undefined2 uStack_84;
  undefined2 uStack_82;
  undefined2 uStack_80;
  undefined2 uStack_7e;
  undefined1 auStack_7c [4];
  undefined1 auStack_78 [4];
  undefined1 auStack_74 [4];
  undefined1 auStack_70 [4];
  uint uStack_6c;
  undefined1 auStack_68 [4];
  undefined1 auStack_64 [4];
  undefined1 auStack_60 [4];
  undefined4 uStack_5c;
  undefined4 uStack_58;
  undefined4 uStack_54;
  undefined4 uStack_50;
  tagPAINTSTRUCT local_4c;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062be78;
  local_c = ExceptionList;
  pHVar16 = (HWND)param_1[6];
  local_ec = (HMODULE)0x0;
  local_d8[0] = (HGDIOBJ)0x0;
  local_108 = pHVar16;
  if (param_2 < 0x10) {
    if (param_2 == 0xf) {
      ExceptionList = &local_c;
      pHVar7 = BeginPaint(pHVar16,&local_4c);
      local_f8[0] = pHVar7;
      h = (HGDIOBJ)FUN_005fc130(param_1[0x117]);
      local_104 = SelectObject((HDC)param_1[0x118],h);
      local_ec = (HMODULE)SelectPalette((HDC)param_1[0x118],DAT_006be5a0,0);
      SelectPalette(pHVar7,DAT_006be5a0,0);
      (**(code **)(*param_1 + 0x28))(param_1[0x118]);
      FUN_00601ce0(param_1 + 0x38,(HDC)param_1[0x118]);
      FUN_00601ce0(param_1 + 0x5e,(HDC)param_1[0x118]);
      FUN_00601ce0(param_1 + 0x6b,(HDC)param_1[0x118]);
      FUN_00601ce0(param_1 + 0x78,(HDC)param_1[0x118]);
      FUN_00601ce0(param_1 + 0x85,(HDC)param_1[0x118]);
      if ((undefined4 *)param_1[0x4a] != (undefined4 *)0x0) {
        (*(code *)**(undefined4 **)param_1[0x4a])(param_1[0x117]);
      }
      if ((undefined4 *)param_1[0x4b] != (undefined4 *)0x0) {
        (*(code *)**(undefined4 **)param_1[0x4b])(param_1[0x117]);
      }
      piVar8 = param_1 + 0xb8;
      local_f0 = (void *)0xc;
      do {
        if (piVar8[4] != 0) {
          local_d0.wParam = piVar8[2];
          local_d0.lParam = piVar8[3];
          if ((void *)piVar8[-2] != (void *)0x0) {
            FUN_005fc140((void *)piVar8[-2],(HDC)param_1[0x118],'@',0xcc0020,*piVar8,piVar8[1],0,0,0
                         ,0);
          }
        }
        pHVar7 = local_f8[0];
        piVar8 = piVar8 + 7;
        local_f0 = (void *)((int)local_f0 + -1);
      } while (local_f0 != (void *)0x0);
      BitBlt(local_f8[0],local_4c.rcPaint.left,local_4c.rcPaint.top,
             local_4c.rcPaint.right - local_4c.rcPaint.left,
             local_4c.rcPaint.bottom - local_4c.rcPaint.top,(HDC)param_1[0x118],
             local_4c.rcPaint.left,local_4c.rcPaint.top,0xcc0020);
      iVar6 = param_1[0x97];
      if (iVar6 != 0) {
        piVar8 = param_1 + 0x98;
        FUN_00601b30(piVar8,*(int *)(iVar6 + 0x28),*(int *)(iVar6 + 0x2c));
        pcVar15 = (char *)FUN_00583c40(param_1[0x97] + 0x14);
        FUN_00601aa0(piVar8,pcVar15);
        FUN_00601b80(piVar8,pHVar7);
        FUN_00601ce0(piVar8,pHVar7);
      }
      SelectPalette((HDC)param_1[0x118],(HPALETTE)local_ec,0);
      SelectObject((HDC)param_1[0x118],local_104);
      EndPaint(local_108,&local_4c);
      ExceptionList = local_c;
      return local_d8[0];
    }
    ExceptionList = &local_c;
    if (param_2 == 1) {
      ExceptionList = &local_c;
      local_f0 = (void *)FUN_006037f0(7);
      local_ec = (HMODULE)FUN_006037f0(2);
      FUN_0060b940(0xbb9,3000,local_ec);
      uVar23 = *(undefined4 *)(param_1[8] + 0x18);
      uVar5 = FUN_00401a80();
      FUN_00401a60(uVar5);
      FUN_00610be0(uVar23,1,0);
      FUN_00427010(param_1,(uint *)0x0);
      param_1[9] = (param_1[0x27] != 1) + 7;
      iVar6 = FUN_005ff440((int)param_1);
      if (iVar6 != 0) {
        pHVar7 = GetDC(pHVar16);
        piVar8 = (int *)FUN_005ff440((int)param_1);
        pHVar9 = FUN_005fc3f0(piVar8);
        FUN_00600010(pHVar7,pHVar9);
        pHVar10 = CreateCompatibleDC(pHVar7);
        param_1[0x118] = (int)pHVar10;
        pvVar11 = (void *)FUN_00618b70(0x28);
        local_4 = 0;
        local_104 = pvVar11;
        if (pvVar11 == (void *)0x0) {
          pvVar11 = (void *)0x0;
        }
        else {
          local_f8[0] = (HDC)param_1[0xe];
          LVar1 = param_1[0xf];
          pHVar10 = pHVar7;
          puVar12 = (uint *)FUN_005ff440((int)param_1);
          pvVar11 = FUN_005fbea0(pvVar11,(LONG)local_f8[0],LVar1,puVar12,pHVar10);
          pHVar16 = local_108;
        }
        local_4 = 0xffffffff;
        param_1[0x117] = (int)pvVar11;
        ReleaseDC(pHVar16,pHVar7);
      }
      if (param_1[0x27] == 1) {
        local_104 = (void *)0x20000ff;
        FUN_00601b30(param_1 + 0x38,0x68,0x12);
        param_1[0x5e] = 0x3f;
        param_1[0x6b] = 0x3f;
        param_1[0x78] = 0x3f;
        param_1[0x5f] = 0x11;
        param_1[0x6c] = 0x11;
        param_1[0x79] = 0x11;
        FUN_00601b30(param_1 + 0x5e,0x109,0x10);
        FUN_00601b30(param_1 + 0x6b,0x168,0x10);
        FUN_00601b30(param_1 + 0x78,0x1c9,0x10);
        SetRect(&local_9c,0x14f,0x10,0x1a9,0x21);
        SetRect(&local_b4,0xf0,0x10,0x14a,0x21);
        SetRect((LPRECT)&local_d0,0x1ae,0x10,0x208,0x21);
        iVar30 = 0x21;
        iVar28 = 0xb9;
        iVar27 = 0x15;
        iVar6 = 0x67;
      }
      else {
        local_104 = (void *)0x200ff00;
        FUN_00601b30(param_1 + 0x38,500,0x12);
        param_1[0x5e] = 0x3f;
        param_1[0x6b] = 0x3f;
        param_1[0x78] = 0x3f;
        param_1[0x5f] = 0x11;
        param_1[0x6c] = 0x11;
        param_1[0x79] = 0x11;
        FUN_00601b30(param_1 + 0x5e,0xaf,0x13);
        FUN_00601b30(param_1 + 0x6b,0x111,0x13);
        FUN_00601b30(param_1 + 0x78,0x173,0x13);
        SetRect(&local_9c,0xf8,0x12,0x154,0x25);
        SetRect(&local_b4,0x95,0x12,0xf1,0x25);
        SetRect((LPRECT)&local_d0,0x159,0x12,0x1b5,0x25);
        iVar30 = 0x1f;
        iVar28 = 0x244;
        iVar27 = 0x13;
        iVar6 = 499;
      }
      this = param_1 + 0x5e;
      piVar17 = param_1 + 0x6b;
      piVar8 = param_1 + 0x78;
      SetRect(&local_e8,iVar6,iVar27,iVar28,iVar30);
      FUN_00601c90(param_1 + 0x38,(int)local_104,(HWND)0x0);
      param_1[0x38] = 0x3c;
      FUN_00403e90(param_1 + 0x38,1);
      param_1[0x43] = 1;
      FUN_00601c60(param_1 + 0x38,10);
      uStack_88 = 0x1513;
      uStack_86 = DAT_0065d424;
      FUN_00600b30(param_1);
      FUN_00601c60(this,8);
      param_1[0x69] = 3;
      FUN_00403e90(this,0x26);
      FUN_00601c90(this,(int)local_104,(HWND)0x0);
      uStack_82 = DAT_0065d424;
      uStack_84 = 0x1510;
      FUN_00600b30(param_1);
      FUN_00601c60(piVar17,8);
      param_1[0x76] = 3;
      FUN_00403e90(piVar17,0x26);
      pvVar11 = local_104;
      FUN_00601c90(piVar17,(int)local_104,(HWND)0x0);
      uStack_8c = 0x1511;
      uStack_8a = DAT_0065d424;
      FUN_00600b30(param_1);
      FUN_00601c60(piVar8,8);
      param_1[0x83] = 3;
      FUN_00403e90(piVar8,0x26);
      FUN_00601c90(piVar8,(int)pvVar11,(HWND)0x0);
      uStack_7e = DAT_0065d424;
      uStack_80 = 0x1512;
      FUN_00600b30(param_1);
      pvVar11 = local_f0;
      FUN_00427270(param_1,local_f0);
      FUN_0042d570((int)param_1);
      FUN_004286b0(param_1,local_ec);
      FUN_00426ee0(param_1,pvVar11);
      FUN_0042adb0((int)param_1);
      pHVar13 = DAT_006be5b4;
      if (DAT_006be5b4 == (HCURSOR)0x0) {
        pHVar13 = LoadCursorA((HINSTANCE)0x0,(LPCSTR)0x7f00);
      }
      param_1[0x119] = (int)pHVar13;
      pHVar13 = LoadCursorA((HINSTANCE)param_1[7],(LPCSTR)0x3ea);
      param_1[0x11a] = (int)pHVar13;
      uVar14 = FUN_00401ae0();
      if ((uVar14 & 0x100) != 0) {
        piVar8 = param_1 + 0x85;
        *piVar8 = 500;
        param_1[0x86] = 10;
        FUN_00601b30(piVar8,0x46,0);
        FUN_00601c60(piVar8,4);
        param_1[0x90] = 3;
        FUN_00403e90(piVar8,0x25);
        FUN_00601c90(piVar8,(int)local_104,(HWND)0x0);
      }
      FUN_004173c0();
      if (param_1[0x2c] == 0) {
        FUN_00422cb0((int)param_1);
        param_1[0x2c] = 0;
      }
      param_1[0x37] = 1;
      FUN_0041dc50();
      ExceptionList = local_c;
      return local_d8[0];
    }
    goto switchD_00424a56_caseD_406;
  }
  if (param_2 < 0x101) {
    if (param_2 == 0x100) {
      bVar25 = false;
      ExceptionList = &local_c;
      if (((HWND)0x6f < param_3) && (ExceptionList = &local_c, param_3 < (HWND)0x77)) {
        ExceptionList = &local_c;
        for (iVar6 = FUN_005f5060((int)(param_1 + 0x1b)); iVar6 != 0; iVar6 = *(int *)(iVar6 + 0x10)
            ) {
          if ((*(byte *)(iVar6 + 0xb8) & 1) != 0) {
            bVar25 = true;
            break;
          }
        }
      }
      if (bVar25) {
        ExceptionList = local_c;
        return local_d8[0];
      }
      SVar2 = GetKeyState(0x11);
      if (SVar2 < 0) {
        ExceptionList = local_c;
        return local_d8[0];
      }
      switch(param_3) {
      case (HWND)0x9:
        ExceptionList = local_c;
        return local_d8[0];
      default:
        param_2 = 0x100;
        break;
      case (HWND)0x70:
        PostMessageA((HWND)param_1[6],0x111,0x133,0);
        ExceptionList = local_c;
        return local_d8[0];
      case (HWND)0x71:
switchD_00423614_caseD_71:
        FUN_0042a000((int)param_1);
        ExceptionList = local_c;
        return local_d8[0];
      case (HWND)0x72:
switchD_00423614_caseD_72:
        FUN_0042a0c0((int)param_1);
        ExceptionList = local_c;
        return local_d8[0];
      case (HWND)0x73:
switchD_00423614_caseD_73:
        FUN_0042a4d0((int)param_1);
        ExceptionList = local_c;
        return local_d8[0];
      case (HWND)0x74:
switchD_00423614_caseD_74:
        FUN_0042a180((int)param_1);
        ExceptionList = local_c;
        return local_d8[0];
      case (HWND)0x75:
switchD_00423614_caseD_75:
        FUN_0042a240(param_1,1,0x79);
        ExceptionList = local_c;
        return local_d8[0];
      case (HWND)0x76:
        FUN_004ece30(&local_ec);
        local_4 = 0x1a;
        FUN_0042dbe0(&local_f0);
        local_4._0_1_ = 0x1b;
        FUN_00429f30(param_1,&local_ec,&local_f0);
        local_4 = CONCAT31(local_4._1_3_,0x1a);
        FUN_00619730();
        local_4 = 0xffffffff;
        FUN_00619730();
        ExceptionList = local_c;
        return local_d8[0];
      }
    }
    else {
      ExceptionList = &local_c;
      if (param_2 != 0x18) goto switchD_00424a56_caseD_406;
      iVar6 = param_1[0x4c];
      if (param_3 == (HWND)0x0) {
        ExceptionList = &local_c;
        if (iVar6 != 0) {
          ExceptionList = &local_c;
          *(undefined4 *)(iVar6 + 0x38) = 1;
        }
        if ((int *)param_1[0x4a] != (int *)0x0) {
          (**(code **)(*(int *)param_1[0x4a] + 0x18))();
        }
        if ((int *)param_1[0x4b] != (int *)0x0) {
          (**(code **)(*(int *)param_1[0x4b] + 0x18))();
        }
      }
      else {
        ExceptionList = &local_c;
        if (iVar6 != 0) {
          ExceptionList = &local_c;
          *(undefined4 *)(iVar6 + 0x38) = 0;
          (**(code **)(*(int *)param_1[0x4c] + 4))();
        }
        if ((int *)param_1[0x4a] != (int *)0x0) {
          (**(code **)(*(int *)param_1[0x4a] + 0x1c))();
        }
        if ((int *)param_1[0x4b] != (int *)0x0) {
          (**(code **)(*(int *)param_1[0x4b] + 0x1c))();
        }
        if ((*(byte *)(param_1 + 0x30) & 8) != 0) {
          param_1[0x30] = 1;
          param_2 = 0x18;
          goto LAB_00425969;
        }
      }
      param_2 = 0x18;
    }
    goto LAB_00425969;
  }
  if (param_2 < 0x201) {
    if (param_2 == 0x200) {
      local_100.y = (uint)param_4 >> 0x10;
      local_100.x = (uint)param_4 & 0xffff;
      if ((param_1[0x30] != 1) && (((uint)param_3 & 4) == 0)) {
        ExceptionList = &local_c;
        SetCursor((HCURSOR)param_1[0x11b]);
        ExceptionList = local_c;
        return local_d8[0];
      }
      ExceptionList = &local_c;
      piVar8 = (int *)(**(code **)(param_1[0x93] + 8))();
      if (piVar8 != (int *)0x0) {
        do {
          local_e8.left = piVar8[0x10];
          local_e8.top = piVar8[0x11];
          local_e8.right = piVar8[0x12];
          local_e8.bottom = piVar8[0x13];
          pt_03.y = local_100.y;
          pt_03.x = local_100.x;
          BVar19 = PtInRect(&local_e8,pt_03);
          if ((*(byte *)(piVar8 + 0xf) & 1) == 0) {
            if (BVar19 != 0) goto LAB_0042416b;
          }
          else if (BVar19 == 0) {
            local_e8.right = local_e8.right + 0x3c;
            InvalidateRect(pHVar16,&local_e8,0);
            piVar8[0xf] = piVar8[0xf] & 0xfffffffe;
            param_1[0x97] = 0;
          }
          else {
LAB_0042416b:
            if ((int *)param_1[0x97] != piVar8) {
              local_e8.right = local_e8.right + 0x3c;
              InvalidateRect(pHVar16,&local_e8,0);
              piVar8[0xf] = piVar8[0xf] | 1;
              param_1[0x97] = (int)piVar8;
              break;
            }
          }
          piVar8 = (int *)(**(code **)(*piVar8 + 0xc))();
          if (piVar8 == (int *)0x0) {
            SetCursor((HCURSOR)param_1[0x119]);
            ExceptionList = local_c;
            return local_d8[0];
          }
        } while( true );
      }
      SetCursor((HCURSOR)param_1[0x119]);
      ExceptionList = local_c;
      return local_d8[0];
    }
    ExceptionList = &local_c;
    if (param_2 == 0x111) {
      ExceptionList = &local_c;
      uVar14 = FUN_004fcee0();
      bVar25 = uVar14 < 2;
      switch((uint)param_3 & 0xffff) {
      case 0x12d:
        if (bVar25) {
          ExceptionList = local_c;
          return local_d8[0];
        }
        goto switchD_00423614_caseD_71;
      case 0x12e:
        if (bVar25) {
          ExceptionList = local_c;
          return local_d8[0];
        }
        goto switchD_00423614_caseD_72;
      case 0x12f:
        if (bVar25) {
          ExceptionList = local_c;
          return local_d8[0];
        }
        goto switchD_00423614_caseD_74;
      case 0x130:
        if (bVar25) {
          ExceptionList = local_c;
          return local_d8[0];
        }
        goto switchD_00423614_caseD_73;
      case 0x131:
        FUN_004ece30(&local_ec);
        local_4 = 3;
        FUN_0042dbe0(&local_f0);
        local_4._0_1_ = 4;
        if (!bVar25) {
          FUN_00429f30(param_1,&local_ec,&local_f0);
        }
        local_4 = CONCAT31(local_4._1_3_,3);
        FUN_00619730();
        local_4 = 0xffffffff;
        FUN_00619730();
        ExceptionList = local_c;
        return local_d8[0];
      case 0x132:
        goto switchD_004236fd_caseD_132;
      case 0x133:
        if (bVar25) {
          ExceptionList = local_c;
          return local_d8[0];
        }
        param_1[0x28] = 1;
        ExceptionList = local_c;
        return local_d8[0];
      case 0x134:
        if (param_1[0x32] == 0x92) {
          ExceptionList = local_c;
          return local_d8[0];
        }
        if (param_1[0x32] == 0x93) {
          ExceptionList = local_c;
          return local_d8[0];
        }
        if (bVar25) {
          ExceptionList = local_c;
          return local_d8[0];
        }
        iVar6 = param_1[0x49];
        if (iVar6 == 0) {
          ExceptionList = local_c;
          return local_d8[0];
        }
        iVar27 = param_1[0x92];
        local_d0.hwnd = *(HWND *)(iVar27 + 0x28);
        local_d0.message = *(UINT *)(iVar27 + 0x2c);
        local_d0.wParam = *(int *)(iVar27 + 0x38) + *(int *)(iVar27 + 0x28);
        local_d0.lParam = *(int *)(iVar27 + 0x3c) + *(int *)(iVar27 + 0x2c);
        local_e8.left = *(LONG *)(iVar6 + 0x28);
        local_e8.top = *(LONG *)(iVar6 + 0x2c);
        local_e8.right = *(int *)(iVar6 + 0x38) + *(int *)(iVar6 + 0x28);
        local_e8.bottom = *(int *)(iVar6 + 0x3c) + *(int *)(iVar6 + 0x2c);
        FUN_00428940(local_108,&local_e8.left,(int *)&local_d0);
        (**(code **)(*(int *)param_1[0x49] + 0x2c))(5);
        BringWindowToTop(*(HWND *)(param_1[0x49] + 0x18));
        (**(code **)(*(int *)param_1[0x92] + 8))();
        ExceptionList = local_c;
        return local_d8[0];
      default:
        local_f8[0] = (HDC)(param_1 + 0x1b);
        piVar8 = (int *)FUN_00604500(local_f8[0],(uint)param_3 & 0xffff);
        if (piVar8 == (int *)0x0) {
          ExceptionList = local_c;
          return local_d8[0];
        }
        if ((uint)param_3 >> 0x10 == 0x29a) {
          FUN_004f4340(&local_9c.left);
          local_4 = 5;
          (**(code **)(*piVar8 + 0x58))(&local_9c,0);
          pHVar16 = local_108;
          local_100.y = (uint)param_4 >> 0x10;
          local_100.x = (uint)param_4 & 0xffff;
          local_f0 = (void *)local_100.y;
          ScreenToClient(local_108,&local_100);
          pt.y = local_100.y;
          pt.x = local_100.x;
          pHVar16 = ChildWindowFromPointEx(pHVar16,pt,1);
          piVar17 = (int *)FUN_00604540(local_f8[0],(int)pHVar16);
          if ((pHVar16 != (HWND)0x0) && (piVar17 != (int *)0x0)) {
            local_100.y = (LONG)local_f0;
            local_100.x = (uint)param_4 & 0xffff;
            ScreenToClient(pHVar16,&local_100);
            (**(code **)(*piVar17 + 0x68))(&local_108,local_100.x,local_100.y);
            local_4c.rgbReserved[0x1c] = '\x06';
            (**(code **)(*piVar17 + 0x70))(&stack0xfffffef0,unaff_EBX,local_108);
            local_4._0_1_ = 7;
            if ((((uint)local_108 & 0xffffff) != 0) || (((uint)local_104 & 0xffffff) != 0)) {
              uVar3 = GetAsyncKeyState(0x11);
              pHVar7 = (HDC)(((uVar3 & 0x8000) != 0) + 0x201);
              local_f8[0] = pHVar7;
              switch(piVar8[9] & 0x3f) {
              case 1:
                FUN_004f4340(&local_e8.left);
                local_4._0_1_ = 8;
                FUN_004f4340(&local_b4.left);
                local_4._0_1_ = 9;
                pvVar11 = (void *)thunk_FUN_005f5060((int)&local_9c);
                while (pvVar11 != (void *)0x0) {
                  puVar12 = (uint *)FUN_00403040(pvVar11,auStack_60);
                  local_4._0_1_ = 10;
                  puVar22 = (undefined4 *)FUN_004f5910(&local_9c,puVar12);
                  local_4._0_1_ = 9;
                  FUN_00619730();
                  if ((*(byte *)(puVar22 + 7) & 4) != 0) {
                    pvVar11 = (void *)FUN_00618b70(0x20);
                    local_4._0_1_ = 0xb;
                    local_f0 = pvVar11;
                    if (pvVar11 == (void *)0x0) {
                      pvVar11 = (void *)0x0;
                    }
                    else {
                      puVar18 = (undefined4 *)FUN_00403040(puVar22,auStack_68);
                      local_ec = (HMODULE)((uint)local_ec | 1);
                      local_4 = CONCAT31(local_4._1_3_,0xc);
                      pvVar11 = FUN_004f5b10(pvVar11,puVar18,4);
                    }
                    local_4 = 0xb;
                    if (((uint)local_ec & 1) != 0) {
                      local_ec = (HMODULE)((uint)local_ec & 0xfffffffe);
                      FUN_00619730();
                    }
                    local_4._0_1_ = 9;
                    FUN_004f57b0(&local_e8,pvVar11);
                  }
                  uVar14 = puVar22[7];
                  puVar22[7] = uVar14 & 0xfffffffb;
                  if ((uVar14 & 0xfffffffb) == 0) {
                    if (puVar22 != (undefined4 *)0x0) {
                      (**(code **)*puVar22)(1);
                    }
                  }
                  else {
                    FUN_004f57b0(&local_b4,puVar22);
                  }
                  pvVar11 = (void *)thunk_FUN_005f5060((int)&local_9c);
                  pHVar7 = local_f8[0];
                }
                iVar6 = thunk_FUN_005f5060((int)&local_b4);
                if ((iVar6 != 0) && (iVar6 = FUN_004ece60((uint *)&local_104), iVar6 != 0)) {
                  FUN_0041cdf0((uint)pHVar7,(int)&local_b4,(uint *)&local_104,0);
                }
                iVar6 = thunk_FUN_005f5060((int)&local_e8);
                if ((iVar6 != 0) && (iVar6 = FUN_004ece60((uint *)&local_104), iVar6 != 0)) {
                  FUN_0041cdf0(0x214,(int)&local_e8,(uint *)&local_108,0);
                }
                local_4._0_1_ = 8;
                FUN_004f4380(&local_b4.left);
                local_4._0_1_ = 7;
                FUN_004f4380(&local_e8.left);
                break;
              case 4:
              case 10:
                iVar6 = FUN_004ece60((uint *)&local_104);
                if (iVar6 != 0) {
                  FUN_0041cdf0((uint)pHVar7,(int)&local_9c,(uint *)&local_104,0);
                }
                break;
              case 9:
                iVar6 = FUN_004ece60((uint *)&local_104);
                if (iVar6 != 0) {
                  FUN_0041cdf0(0x214,(int)&local_9c,(uint *)&local_104,0);
                }
              }
            }
            local_4._0_1_ = 6;
            FUN_00619730();
            local_4 = CONCAT31(local_4._1_3_,5);
            FUN_00619730();
          }
          thunk_FUN_005f50a0(&local_9c);
          local_4 = 0xffffffff;
          FUN_004f4380(&local_9c.left);
          ExceptionList = local_c;
          return local_d8[0];
        }
        if ((uint)param_3 >> 0x10 != 0x466) {
          ExceptionList = local_c;
          return local_d8[0];
        }
        FUN_00428b40(param_1,piVar8);
        if (param_1[0x2e] != piVar8[6]) {
          ExceptionList = local_c;
          return local_d8[0];
        }
        SendMessageA(local_108,0x467,0,0);
        ExceptionList = local_c;
        return local_d8[0];
      case 0x136:
        FUN_0042a240(param_1,1,0x7a);
        ExceptionList = local_c;
        return local_d8[0];
      case 0x137:
        FUN_0042a240(param_1,1,0x7b);
        ExceptionList = local_c;
        return local_d8[0];
      case 0x138:
        FUN_0042a240(param_1,1,0x7c);
        ExceptionList = local_c;
        return local_d8[0];
      case 0x139:
        FUN_0042a240(param_1,1,0x7d);
        ExceptionList = local_c;
        return local_d8[0];
      case 0x13a:
        FUN_0042a240(param_1,1,0x7e);
        ExceptionList = local_c;
        return local_d8[0];
      case 0x13b:
        FUN_0042a240(param_1,1,0x7f);
        ExceptionList = local_c;
        return local_d8[0];
      case 0x13c:
        FUN_0042a240(param_1,1,0x80);
        ExceptionList = local_c;
        return local_d8[0];
      case 0x13d:
        FUN_0042a240(param_1,1,0x82);
        ExceptionList = local_c;
        return local_d8[0];
      case 0x13e:
        FUN_0042a240(param_1,1,0x81);
        ExceptionList = local_c;
        return local_d8[0];
      case 0x140:
        iVar6 = param_1[0x46];
        if (iVar6 != 0) {
          local_d0.hwnd = *(HWND *)(iVar6 + 0x28);
          local_d0.message = *(UINT *)(iVar6 + 0x2c);
          local_d0.wParam = *(int *)(iVar6 + 0x38) + *(int *)(iVar6 + 0x28);
          local_d0.lParam = *(int *)(iVar6 + 0x3c) + *(int *)(iVar6 + 0x2c);
          iVar6 = param_1[0x52];
          local_e8.left = *(int *)(iVar6 + 0x28);
          local_e8.top = *(int *)(iVar6 + 0x2c);
          local_e8.right = *(int *)(iVar6 + 0x38) + *(int *)(iVar6 + 0x28);
          local_e8.bottom = *(int *)(iVar6 + 0x3c) + *(int *)(iVar6 + 0x2c);
          FUN_00428940(local_108,&local_e8.left,(int *)&local_d0);
          FUN_00610b20(&local_b4,0,0,0,0,0,0);
        }
        FUN_00425d00(param_1,(uint)param_3 >> 0x10,(int)param_4);
        FUN_00426d00(param_1,(uint)param_3 >> 0x10);
        for (piVar8 = (int *)FUN_005f5060((int)(param_1 + 0x1b)); piVar8 != (int *)0x0;
            piVar8 = (int *)piVar8[4]) {
          iVar6 = FUN_004291d0(param_1,piVar8[9]);
          if (iVar6 == 0) {
            (**(code **)(*piVar8 + 4))(5);
          }
        }
        ExceptionList = local_c;
        return local_d8[0];
      case 0x154:
        FUN_004019c0();
        ExceptionList = local_c;
        return local_d8[0];
      case 0x15e:
        if (param_1[0x2b] != 0) {
          ExceptionList = local_c;
          return local_d8[0];
        }
        if (param_1[0x30] == 2) {
          if ((undefined4 *)param_1[0x31] != (undefined4 *)0x0) {
            (*(code *)**(undefined4 **)param_1[0x31])(1);
            param_1[0x31] = 0;
          }
          param_1[0x30] = 1;
          pHVar16 = GetCapture();
          if (pHVar16 == local_108) {
            ReleaseCapture();
          }
        }
        FUN_00401040(0xb);
        ExceptionList = local_c;
        return local_d8[0];
      }
    }
    goto switchD_00424a56_caseD_406;
  }
  if (param_2 < 0x403) {
    if (param_2 == 0x402) {
      if ((int *)param_1[0x126] == (int *)0x0) {
        return (HGDIOBJ)0x0;
      }
      uVar23 = *(undefined4 *)(param_1[8] + 0x18);
      ExceptionList = &local_c;
      FUN_0041d330((int *)param_1[0x126]);
      param_1[0x126] = 0;
      uVar5 = FUN_00401a80();
      FUN_00401a60(uVar5);
      FUN_00610be0(uVar23,1,0);
      FUN_004173c0();
      param_1[0x2d] = 1;
      FUN_00422cb0((int)param_1);
      ExceptionList = local_c;
      return local_d8[0];
    }
    if (param_2 == 0x202) {
      ExceptionList = &local_c;
      uVar14 = FUN_004fcee0();
      if (uVar14 < 2) {
        ExceptionList = local_c;
        return local_d8[0];
      }
      local_100.y = (uint)param_4 >> 0x10;
      local_100.x = (uint)param_4 & 0xffff;
      if (param_1[0x30] == 1) {
        if ((param_1[0x46] != 0) &&
           (BVar19 = IsWindowVisible(*(HWND *)(param_1[0x46] + 0x18)), BVar19 != 0)) {
          FUN_00610b20(&local_d0,0,0,0,0,0,0);
          local_d0.time = local_d0.time & 0xfffffff9 | 1;
          local_4 = 0x12;
          local_d0.message = 0x259;
          local_d0.wParam = 7;
          FUN_00610c30((int)&local_d0);
          (**(code **)(*(int *)param_1[0x46] + 8))();
          for (piVar8 = (int *)FUN_005f5060((int)(param_1 + 0x1b)); piVar8 != (int *)0x0;
              piVar8 = (int *)piVar8[4]) {
            iVar6 = FUN_004291d0(param_1,piVar8[9]);
            if (iVar6 == 0) {
              (**(code **)(*piVar8 + 4))(5);
            }
          }
          local_4 = 0xffffffff;
        }
        if (param_1[0x97] != 0) {
          if ((param_1[0x49] != 0) &&
             (BVar19 = IsWindowVisible(*(HWND *)(param_1[0x49] + 0x18)), BVar19 != 0)) {
            PostMessageA(pHVar16,0x405,*(WPARAM *)(param_1[0x49] + 0x24),0);
          }
          puVar12 = (uint *)FUN_0042d170((void *)param_1[0x97],auStack_7c);
          local_4 = 0x13;
          FUN_00429ce0(param_1,puVar12);
          local_4 = 0xffffffff;
          FUN_00619730();
          ExceptionList = local_c;
          return local_d8[0];
        }
        iVar6 = FUN_004420b0((void *)param_1[0x4b],local_100.x,local_100.y);
        if (iVar6 == 0) {
          ExceptionList = local_c;
          return local_d8[0];
        }
        goto switchD_00423614_caseD_75;
      }
      if (param_1[0x30] != 2) {
        ExceptionList = local_c;
        return local_d8[0];
      }
      pHVar16 = ChildWindowFromPointEx
                          (local_108,(POINT)(CONCAT44(local_100.y,param_4) & 0xffffffff0000ffff),1);
      piVar8 = (int *)FUN_00604540(param_1 + 0x1b,(int)pHVar16);
      if (piVar8 == (int *)0x0) {
        pHVar21 = GetParent(pHVar16);
        piVar8 = (int *)FUN_00604540(param_1 + 0x1b,(int)pHVar21);
        if ((piVar8 == (int *)0x0) &&
           (local_f8[0] = (HDC)((uint)param_3 & 4), local_f8[0] == (HDC)0x0)) goto LAB_0042499b;
      }
      MapWindowPoints(local_108,pHVar16,&local_100,1);
      local_f8[0] = (HDC)((uint)param_3 & 4);
      if (local_f8[0] != (HDC)0x0) {
        pt_01.y = local_100.y;
        pt_01.x = local_100.x;
        pHVar21 = ChildWindowFromPointEx(pHVar16,pt_01,1);
        pHVar20 = pHVar16;
        if (pHVar21 != pHVar16) {
          do {
            pHVar16 = pHVar21;
            MapWindowPoints(pHVar20,pHVar16,&local_100,1);
            pt_02.y = local_100.y;
            pt_02.x = local_100.x;
            pHVar21 = ChildWindowFromPointEx(pHVar16,pt_02,1);
            pHVar20 = pHVar16;
          } while (pHVar21 != pHVar16);
        }
        param_1[0x30] = 1;
        uVar14 = local_100.y << 0x10 | local_100.x & 0xffffU;
        SendMessageA(pHVar16,0x201,0,uVar14);
        SendMessageA(pHVar16,0x202,0,uVar14);
        param_1[0x30] = 2;
        FUN_00601080(local_108);
        goto LAB_0042499b;
      }
      param_1[0x11b] = param_1[0x119];
      FUN_004ece30(&local_104);
      local_4 = 0x14;
      if ((int *)param_1[0x31] == (int *)0x0) {
        puVar22 = (undefined4 *)(**(code **)(*piVar8 + 0x70))(auStack_78,local_100.x,local_100.y);
        local_4._0_1_ = 0x15;
        FUN_004f26d0(&local_104,puVar22);
      }
      else {
        iVar6 = (**(code **)(*(int *)param_1[0x31] + 0xc))();
        if (((iVar6 == 0x201) ||
            (iVar6 = (**(code **)(*(int *)param_1[0x31] + 0xc))(), iVar6 == 0x214)) ||
           (iVar6 = (**(code **)(*(int *)param_1[0x31] + 0xc))(), iVar6 == 0x202)) {
          puVar22 = (undefined4 *)(**(code **)(*piVar8 + 0x70))(auStack_64,local_100.x,local_100.y);
          local_4._0_1_ = 0x16;
          FUN_004f26d0(&local_104,puVar22);
        }
        else {
          puVar22 = (undefined4 *)(**(code **)(*piVar8 + 0x68))(auStack_74,local_100.x,local_100.y);
          local_4._0_1_ = 0x17;
          FUN_004f26d0(&local_104,puVar22);
        }
      }
      local_4._0_1_ = 0x14;
      FUN_00619730();
      piVar8 = (int *)param_1[0x31];
      if (piVar8 == (int *)0x0) {
LAB_004248e2:
        FUN_0041ce40((uint *)&local_104);
      }
      else if (((uint)local_104 & 0xffffff) == 0) {
        if (piVar8 == (int *)0x0) goto LAB_004248e2;
      }
      else {
        uVar23 = (**(code **)(*piVar8 + 0xc))();
        switch(uVar23) {
        case 0x201:
        case 0x202:
        case 0x214:
          iVar6 = FUN_004ece60((uint *)&local_104);
          if (iVar6 != 0) {
            (**(code **)(*(int *)param_1[0x31] + 0x2c))(&local_104);
            FUN_0041ce20((int *)param_1[0x31],0);
LAB_004248c0:
            param_1[0x31] = 0;
          }
          break;
        case 0x240:
          iVar6 = FUN_004ece60((uint *)&local_104);
          if (iVar6 != 0) {
            pvVar11 = (void *)(**(code **)(*(int *)param_1[0x31] + 0x28))();
            pvVar11 = FUN_004f5940(pvVar11,(uint *)&local_104);
            if ((pvVar11 != (void *)0x0) &&
               (piVar8 = FUN_004f2d10(param_1[0x27],(uint *)&local_104), piVar8 != (int *)0x0)) {
              FUN_004ece30(&local_ec);
              local_4._0_1_ = 0x18;
              local_f0 = (void *)0x0;
              do {
                if (local_f0 != (void *)0x0) break;
                puVar12 = FUN_004025b0(piVar8,&uStack_6c);
                local_4._0_1_ = 0x19;
                FUN_004f26d0(&local_ec,puVar12);
                local_4._0_1_ = 0x18;
                FUN_00619730();
                uStack_5c = 0x90;
                uStack_58 = 0x98;
                if (((uint)local_ec >> 0x18 < 0x90) || (0x97 < (uint)local_ec >> 0x18)) {
                  bVar25 = false;
                }
                else {
                  bVar25 = true;
                }
                FUN_00619730();
                if (bVar25) {
                  local_f0 = (void *)0x1;
                }
                piVar8 = (int *)piVar8[7];
              } while (piVar8 != (int *)0x0);
              if (local_f0 != (void *)0x0) {
                FUN_004f26d0(&local_104,&local_ec);
              }
              local_4._0_1_ = 0x14;
              FUN_00619730();
            }
            (**(code **)(*(int *)param_1[0x31] + 0x2c))(&local_104);
            FUN_0042a320(param_1,(int *)param_1[0x31]);
            goto LAB_004248c0;
          }
        }
        if ((undefined4 *)param_1[0x31] != (undefined4 *)0x0) {
          (*(code *)**(undefined4 **)param_1[0x31])(1);
        }
        param_1[0x31] = 0;
      }
      local_4 = 0xffffffff;
      FUN_00619730();
LAB_0042499b:
      if (local_f8[0] != (HDC)0x0) {
        ExceptionList = local_c;
        return local_d8[0];
      }
      if ((undefined4 *)param_1[0x31] != (undefined4 *)0x0) {
        (*(code *)**(undefined4 **)param_1[0x31])(1);
        param_1[0x31] = 0;
      }
      param_1[0x30] = 1;
      pHVar16 = GetCapture();
      if (pHVar16 != local_108) {
        ExceptionList = local_c;
        return local_d8[0];
      }
      ReleaseCapture();
      ExceptionList = local_c;
      return local_d8[0];
    }
    if (param_2 == 0x203) {
      ExceptionList = &local_c;
      uVar14 = FUN_004fcee0();
      pHVar16 = local_108;
      if (uVar14 < 2) {
        ExceptionList = local_c;
        return local_d8[0];
      }
      local_100.x = (uint)param_4 & 0xffff;
      local_100.y = (uint)param_4 >> 0x10;
      if (param_1[0x30] == 2) {
        pHVar20 = ChildWindowFromPointEx
                            (local_108,(POINT)(CONCAT44(local_100.y,param_4) & 0xffffffff0000ffff),1
                            );
        pHVar21 = pHVar16;
        if (pHVar20 != pHVar16) {
          do {
            pHVar16 = pHVar20;
            MapWindowPoints(pHVar21,pHVar16,&local_100,1);
            pt_00.y = local_100.y;
            pt_00.x = local_100.x;
            pHVar20 = ChildWindowFromPointEx(pHVar16,pt_00,1);
            pHVar21 = pHVar16;
          } while (pHVar20 != pHVar16);
        }
        param_1[0x30] = 1;
        SendMessageA(pHVar16,0x203,0,local_100.y << 0x10 | local_100.x & 0xffffU);
        param_1[0x30] = 2;
        FUN_00601080(local_108);
        ExceptionList = local_c;
        return local_d8[0];
      }
      bVar25 = false;
      iVar6 = 0;
      lprc = (RECT *)(param_1 + 0xb8);
      do {
        if (bVar25) {
          ExceptionList = local_c;
          return local_d8[0];
        }
        pt_05.y = local_100.y;
        pt_05.x = local_100.x;
        BVar19 = PtInRect(lprc,pt_05);
        if (BVar19 != 0) {
          FUN_00429020(param_1,iVar6 + 1);
          bVar25 = true;
        }
        iVar6 = iVar6 + 1;
        lprc = (RECT *)&lprc[1].bottom;
      } while (iVar6 < 0xc);
      ExceptionList = local_c;
      return local_d8[0];
    }
    ExceptionList = &local_c;
    if (param_2 == 0x205) {
      ExceptionList = &local_c;
      uVar14 = FUN_004fcee0();
      if (uVar14 < 2) {
        ExceptionList = local_c;
        return local_d8[0];
      }
      local_100.x = (LONG)(short)param_4;
      local_100.y = (LONG)param_4._2_2_;
      if (param_1[0x27] == 1) {
        iVar30 = 0x21;
        iVar28 = 0xb9;
        iVar27 = 0x15;
        iVar6 = 0x67;
      }
      else {
        iVar30 = 0x1f;
        iVar28 = 0x244;
        iVar27 = 0x13;
        iVar6 = 499;
      }
      SetRect((LPRECT)&local_d0,iVar6,iVar27,iVar28,iVar30);
      iVar6 = FUN_004420b0((void *)param_1[0x4a],local_100.x,local_100.y);
      if (iVar6 != 0) {
        FUN_0042d050(param_1,local_100.x,local_100.y,iVar6);
        ExceptionList = local_c;
        return local_d8[0];
      }
      iVar6 = FUN_004420b0((void *)param_1[0x4b],local_100.x,local_100.y);
      if (iVar6 != 0) {
        FUN_0042d050(param_1,local_100.x,local_100.y,iVar6);
        ExceptionList = local_c;
        return local_d8[0];
      }
      pt_04.y = local_100.y;
      pt_04.x = local_100.x;
      BVar19 = PtInRect((RECT *)&local_d0,pt_04);
      if (BVar19 == 0) {
        ExceptionList = local_c;
        return local_d8[0];
      }
      FUN_0042d190(param_1,local_100.x,local_100.y);
      ExceptionList = local_c;
      return local_d8[0];
    }
    goto switchD_00424a56_caseD_406;
  }
  ExceptionList = &local_c;
  switch(param_2) {
  case 0x405:
    bVar25 = true;
    ExceptionList = &local_c;
    FUN_005f54a0(param_1 + 0x11e,(uint)param_3);
    FUN_00610b20(&local_b4,0,0,0,0,0,0);
    local_a4 = local_a4 & 0xfffffff9 | 1;
    local_4 = 0xd;
    local_b4.right = 7;
    local_b4.top = 0;
    switch((uint)param_3 & 0x3f) {
    case 1:
      local_b4.top = 0x25d;
      break;
    case 3:
      iVar6 = param_1[0x49];
      if (iVar6 != 0) {
        iVar27 = param_1[0x92];
        local_e8.left = *(int *)(iVar27 + 0x28);
        local_e8.top = *(int *)(iVar27 + 0x2c);
        local_e8.right = *(int *)(iVar27 + 0x38) + *(int *)(iVar27 + 0x28);
        local_e8.bottom = *(int *)(iVar27 + 0x3c) + *(int *)(iVar27 + 0x2c);
        local_d0.hwnd = *(HWND *)(iVar6 + 0x28);
        local_d0.message = *(UINT *)(iVar6 + 0x2c);
        local_d0.wParam = *(int *)(iVar6 + 0x38) + *(int *)(iVar6 + 0x28);
        local_d0.lParam = *(int *)(iVar6 + 0x3c) + *(int *)(iVar6 + 0x2c);
        FUN_00428940(pHVar16,&local_e8.left,(int *)&local_d0);
        (**(code **)(*(int *)param_1[0x49] + 8))();
        (**(code **)(*(int *)param_1[0x92] + 4))(5);
      }
      bVar25 = false;
      break;
    case 4:
    case 9:
    case 10:
    case 0xb:
      local_b4.top = 0x25f;
    }
    if (local_b4.top != 0) {
      FUN_00610c30((int)&local_b4);
    }
    if (bVar25) {
      FUN_00600f90(param_1,(uint)param_3);
      PostMessageA(pHVar16,0x467,0,0);
    }
    break;
  case 0x409:
    ExceptionList = &local_c;
    local_d8[0] = (HGDIOBJ)FUN_00442430();
    break;
  case 0x467:
    ExceptionList = &local_c;
    pHVar16 = (HWND)FUN_00601320();
    PostMessageA((HWND)param_1[6],0x409,0,0);
    if (pHVar16 != (HWND)0x0) {
      param_3 = pHVar16;
    }
    iVar6 = FUN_00604540(param_1 + 0x1b,(int)param_3);
    if ((HWND)param_1[0x2e] == param_3) {
      if (param_3 == (HWND)0x0) {
        SetFocus((HWND)param_1[6]);
      }
    }
    else if ((iVar6 == 0) ||
            (((*(byte *)(param_1 + 0x30) & 4) == 0 &&
             (iVar6 = FUN_004291d0(param_1,*(int *)(iVar6 + 0x24)), iVar6 == 0)))) {
      BVar19 = IsWindow((HWND)param_1[0x2e]);
      if (BVar19 != 0) {
        PostMessageA((HWND)param_1[0x2e],0x467,(WPARAM)param_1[0x2e],(LPARAM)param_3);
      }
      if (param_3 == (HWND)0x0) {
        pHVar16 = GetWindow((HWND)param_1[6],5);
        bVar25 = false;
        param_1[0x2e] = (int)pHVar16;
        while ((pHVar16 != (HWND)0x0 && (!bVar25))) {
          iVar6 = FUN_00604540(param_1 + 0x1b,param_1[0x2e]);
          if ((iVar6 != 0) &&
             ((iVar27 = FUN_005f5500(param_1 + 0x11e,*(uint *)(iVar6 + 0x24)), iVar27 != 0 &&
              (iVar6 = FUN_004291d0(param_1,*(int *)(iVar6 + 0x24)), iVar6 == 0)))) {
            bVar25 = true;
          }
          if (!bVar25) {
            pHVar16 = GetWindow((HWND)param_1[0x2e],2);
            param_1[0x2e] = (int)pHVar16;
          }
          pHVar16 = (HWND)param_1[0x2e];
        }
      }
      else {
        iVar6 = param_1[0x2e];
        param_1[0x2e] = (int)param_3;
        param_1[0x2f] = iVar6;
      }
      pHVar16 = (HWND)param_1[0x2e];
      if (pHVar16 == (HWND)0x0) {
        SetFocus((HWND)param_1[6]);
      }
      else {
        PostMessageA(pHVar16,0x467,0,(LPARAM)pHVar16);
        ShowWindow((HWND)param_1[0x2e],5);
        BringWindowToTop((HWND)param_1[0x2e]);
        SetFocus((HWND)param_1[0x2e]);
      }
    }
    break;
  case 0x468:
  case 0x482:
    ExceptionList = &local_c;
    uVar14 = FUN_004fcee0();
    if (uVar14 < 2) {
      ExceptionList = local_c;
      return local_d8[0];
    }
    PostMessageA((HWND)param_1[6],0x409,0,0);
    if ((int *)param_1[0x46] != (int *)0x0) {
      (**(code **)(*(int *)param_1[0x46] + 8))();
    }
    bVar25 = FUN_00601340();
    iVar6 = CONCAT31(extraout_var,bVar25);
    while (iVar6 == 0) {
      param_1[0x29] = 1;
      iVar6 = PeekMessageA(&local_d0,(HWND)0x0,0,0,0);
      while (iVar6 != 0) {
        if (local_d0.message == 0x12) {
          ExceptionList = local_c;
          return (HGDIOBJ)0x0;
        }
        if ((local_d0.hwnd == (HWND)param_1[6]) && (local_d0.message == 2)) {
          ExceptionList = local_c;
          return (HGDIOBJ)0x0;
        }
        GetMessageA(&local_d0,(HWND)0x0,0,0);
        TranslateMessage(&local_d0);
        DispatchMessageA(&local_d0);
        iVar6 = PeekMessageA(&local_d0,(HWND)0x0,0,0,0);
      }
      local_f8[0] = (HDC)0x1;
      if (DAT_006b14cc == (int *)0x0) {
        ExceptionList = local_c;
        return (HGDIOBJ)0x0;
      }
      (**(code **)(*DAT_006b14cc + 0x2c))();
      FUN_00401bd0((uint *)local_f8);
      bVar25 = FUN_00601340();
      iVar6 = CONCAT31(extraout_var_00,bVar25);
    }
    param_1[0x29] = 0;
    FUN_00601080((HWND)param_1[6]);
    if (param_2 != 0x468) {
      if (param_3 == (HWND)0x0) {
        FUN_00401040((int)param_4);
        ExceptionList = local_c;
        return local_d8[0];
      }
      if (param_3 != (HWND)0x1) {
        ExceptionList = local_c;
        return local_d8[0];
      }
      FUN_00610c00(0);
      FUN_004019a0(param_4[0x17],(int)(param_4 + 0x14),0);
      ExceptionList = local_c;
      return local_d8[0];
    }
    piVar8 = (int *)0x0;
    switch((uint)param_3 & 0x3f) {
    case 6:
      param_1[0x28] = 0;
      local_f8[0] = (HDC)FUN_00618b70(0x140);
      local_4 = 0xe;
      if (local_f8[0] == (HDC)0x0) {
LAB_00424dca:
        local_4 = 0xffffffff;
        piVar8 = (int *)0x0;
      }
      else {
        piVar8 = (int *)FUN_0044f670(local_f8[0],param_1[7],(int)param_1,param_3,param_4);
        local_4 = 0xffffffff;
      }
      break;
    case 0xd:
      if (param_4 == (uint *)0x0) {
        uVar24 = 0x79;
        uVar14 = 1;
      }
      else {
        uVar24 = (uint)param_4 >> 0x10;
        uVar14 = (uint)param_4 & 0xffff;
      }
      FUN_0042a240(param_1,uVar14,uVar24);
      break;
    case 0x10:
      local_f8[0] = (HDC)FUN_00618b70(0x124);
      local_4 = 0xf;
      if (local_f8[0] == (HDC)0x0) goto LAB_00424dca;
      piVar8 = (int *)FUN_0044f060(local_f8[0],param_1[7],(int)param_1,param_3,param_4);
      local_4 = 0xffffffff;
      break;
    case 0x1a:
      piVar17 = FUN_004f2d10(param_1[0x27],param_4);
      if ((piVar17 != (int *)0x0) || (iVar6 = FUN_004ece60(param_4), iVar6 == 0)) {
        local_f8[0] = (HDC)FUN_00618b70(0x1d8);
        local_4 = 0x11;
        if (local_f8[0] == (HDC)0x0) {
          piVar8 = (int *)0x0;
        }
        else {
          piVar8 = (int *)FUN_00442d70(local_f8[0],param_1[7],(int)param_1,param_3,param_4);
        }
        local_4 = 0xffffffff;
      }
      if (param_4 != (uint *)0x0) {
        FUN_00619730();
        FUN_00618b60((undefined *)param_4);
      }
      break;
    case 0x1c:
      local_f8[0] = (HDC)FUN_00618b70(0x188);
      local_4 = 0x10;
      if (local_f8[0] == (HDC)0x0) goto LAB_00424dca;
      piVar8 = (int *)FUN_0044c410(local_f8[0],param_1[7],(int)param_1,param_3,param_4);
      local_4 = 0xffffffff;
    }
    ReleaseCapture();
    if (piVar8 != (int *)0x0) {
      piVar8[0x14] = piVar8[0x14] & 0xefffffff;
      FUN_005ffce0(piVar8,0);
      (**(code **)(*piVar8 + 4))(5);
      FUN_005f4f10(param_1 + 0x1b,(int)piVar8);
      SetFocus((HWND)piVar8[6]);
    }
    break;
  case 0x483:
    bVar25 = DAT_006be4e4 == 0;
    ExceptionList = &local_c;
    FUN_005205a0(local_d8);
    local_4 = 0x1c;
    FUN_0041d160(local_d8,0);
    FUN_004f4340(local_f8);
    local_4._0_1_ = 0x1d;
    FUN_004ece30(&local_108);
    local_4._0_1_ = 0x1e;
    PostMessageA((HWND)param_1[6],0x409,0,0);
    switch((uint)param_3 & 0xffff) {
    case 0xbb9:
      if (bVar25) {
        for (piVar8 = (int *)FUN_005f5060((int)(param_1 + 0x1b)); piVar8 != (int *)0x0;
            piVar8 = (int *)piVar8[4]) {
          iVar6 = FUN_004291d0(param_1,piVar8[9]);
          if (iVar6 == 0) {
            (**(code **)(*piVar8 + 0x30))();
          }
        }
      }
      break;
    case 0xbba:
      sVar4 = (short)param_1[0x45];
      if (sVar4 == 1) {
        FUN_0041cdf0(0x22,(int)local_f8,(uint *)&local_108,0);
      }
      else if (sVar4 == 2) {
        FUN_0041cdf0(0x23,(int)local_f8,(uint *)&local_108,0);
      }
      else if (sVar4 == 3) {
        FUN_0041cdf0(0x24,(int)local_f8,(uint *)&local_108,0);
      }
      break;
    case 0xbbb:
      sVar4 = (short)param_1[0x45];
      if (sVar4 == 2) {
        FUN_0041cdf0(0x21,(int)local_f8,(uint *)&local_108,0);
      }
      else if (sVar4 == 3) {
        FUN_0041cdf0(0x22,(int)local_f8,(uint *)&local_108,0);
      }
      else if (sVar4 == 4) {
        FUN_0041cdf0(0x23,(int)local_f8,(uint *)&local_108,0);
      }
      break;
    case 0xbbc:
      if ((short)param_1[0x45] != 0) {
        FUN_0041cdf0(0x20,(int)local_f8,(uint *)&local_108,0);
      }
      break;
    case 0xbbd:
      if ((bVar25) && (iVar6 = FUN_00401080(), iVar6 != 1)) {
        FUN_0042a240(param_1,0x68,0x79);
      }
      break;
    case 0xbbe:
      iVar6 = FUN_00604500(param_1 + 0x1b,0xd);
      if (iVar6 == 0) {
        iVar6 = FUN_00604500(param_1 + 0x1b,0x19);
        if (iVar6 != 0) {
          PostMessageA(*(HWND *)(iVar6 + 0x18),0x483,0xbbe,0);
        }
      }
      else {
        PostMessageA(*(HWND *)(iVar6 + 0x18),0x483,0xbbe,0);
      }
      break;
    case 0xbbf:
      if ((bVar25) &&
         (piVar8 = (int *)FUN_00604540(param_1 + 0x1b,param_1[0x2e]), piVar8 != (int *)0x0)) {
        (**(code **)(*piVar8 + 0x58))(local_f8,0);
        sVar4 = FUN_005f50e0((int)local_f8);
        if (sVar4 == 1) {
          FUN_005205a0(&local_e8.left);
          ptVar31 = &local_e8;
          ppHVar29 = local_f8;
          local_4 = CONCAT31(local_4._1_3_,0x1f);
          iVar6 = FUN_00401060();
          FUN_0041dcc0(iVar6,(uint)ppHVar29,ptVar31);
          iVar6 = FUN_00520720(&local_e8,0x103);
          if ((iVar6 != 0) && (*(int *)(iVar6 + 0x1c) != 0)) {
            FUN_0041cdf0(0x103,(int)local_f8,(uint *)&local_108,0);
          }
          local_4._0_1_ = 0x1e;
          FUN_005205e0(&local_e8.left);
        }
      }
      break;
    case 0xbc0:
      if ((bVar25) &&
         (piVar8 = (int *)FUN_00604540(param_1 + 0x1b,param_1[0x2e]), piVar8 != (int *)0x0)) {
        (**(code **)(*piVar8 + 0x58))(local_f8,0);
        pvVar11 = (void *)thunk_FUN_005f5060((int)local_f8);
        if (pvVar11 != (void *)0x0) {
          puVar12 = (uint *)FUN_00403040(pvVar11,auStack_70);
          uStack_54 = 0x30;
          uStack_50 = 0x40;
          local_4._0_1_ = 0x20;
          if ((*puVar12 >> 0x18 < 0x30) || (0x3f < *puVar12 >> 0x18)) {
            bVar25 = false;
          }
          else {
            bVar25 = true;
          }
          FUN_00619730();
          local_4._0_1_ = 0x1e;
          FUN_00619730();
          if (bVar25) {
            FUN_005205a0(&local_e8.left);
            ptVar31 = &local_e8;
            ppHVar29 = local_f8;
            local_4 = CONCAT31(local_4._1_3_,0x21);
            iVar6 = FUN_00401060();
            FUN_0041dcc0(iVar6,(uint)ppHVar29,ptVar31);
            iVar6 = FUN_00520720(&local_e8,0x240);
            if ((iVar6 != 0) && (*(int *)(iVar6 + 0x1c) != 0)) {
              FUN_0041cdf0(0x240,(int)local_f8,(uint *)&local_108,0);
            }
            local_4._0_1_ = 0x1e;
            FUN_005205e0(&local_e8.left);
          }
        }
      }
      break;
    case 0xbc1:
      if (((bVar25) && (param_1[0x30] == 1)) &&
         ((iVar6 = FUN_00520720(local_d8,0x110), iVar6 != 0 && (*(int *)(iVar6 + 0x1c) != 0)))) {
        FUN_0041cdf0(0x110,(int)local_f8,(uint *)&local_108,0);
      }
      break;
    case 0xbc2:
      if ((((bVar25) && (param_1[0x30] == 1)) && (iVar6 = FUN_00520720(local_d8,0x111), iVar6 != 0))
         && (*(int *)(iVar6 + 0x1c) != 0)) {
        FUN_0041cdf0(0x111,(int)local_f8,(uint *)&local_108,0);
      }
      break;
    case 0xbc3:
      if (((bVar25) && (param_1[0x30] == 1)) &&
         ((iVar6 = FUN_00520720(local_d8,0x112), iVar6 != 0 && (*(int *)(iVar6 + 0x1c) != 0)))) {
        FUN_0041cdf0(0x112,(int)local_f8,(uint *)&local_108,0);
      }
      break;
    case 0xbc4:
      if (((bVar25) && (param_1[0x30] == 1)) &&
         ((iVar6 = FUN_00520720(local_d8,0x113), iVar6 != 0 && (*(int *)(iVar6 + 0x1c) != 0)))) {
        FUN_0041cdf0(0x113,(int)local_f8,(uint *)&local_108,0);
      }
      break;
    case 0xbc5:
      if ((((bVar25) && (param_1[0x30] == 1)) && (iVar6 = FUN_00520720(local_d8,0x115), iVar6 != 0))
         && (*(int *)(iVar6 + 0x1c) != 0)) {
        FUN_0041cdf0(0x115,(int)local_f8,(uint *)&local_108,0);
      }
      break;
    case 0xbc6:
      if (((bVar25) && (param_1[0x30] == 1)) &&
         ((iVar6 = FUN_00520720(local_d8,0x116), iVar6 != 0 && (*(int *)(iVar6 + 0x1c) != 0)))) {
        FUN_0041cdf0(0x116,(int)local_f8,(uint *)&local_108,0);
      }
      break;
    case 0xbc8:
      if (((bVar25) && (param_1[0x30] == 1)) &&
         ((iVar6 = FUN_00520720(local_d8,0x11b), iVar6 != 0 && (*(int *)(iVar6 + 0x1c) != 0)))) {
        FUN_0041cdf0(0x11b,(int)local_f8,(uint *)&local_108,0);
      }
      break;
    case 0xbc9:
      if ((((bVar25) && (param_1[0x30] == 1)) && (iVar6 = FUN_00520720(local_d8,0x11e), iVar6 != 0))
         && (*(int *)(iVar6 + 0x1c) != 0)) {
        FUN_0041cdf0(0x11e,(int)local_f8,(uint *)&local_108,0);
      }
      break;
    case 0xbca:
      FUN_00425d00(param_1,0x11,0);
      break;
    case 0xbcb:
      FUN_00425d00(param_1,0x12,0);
      break;
    case 0xbcc:
      FUN_00425d00(param_1,0x21,0);
      break;
    case 0xbcd:
      FUN_00425d00(param_1,0x22,0);
      break;
    case 0xbce:
      FUN_00425d00(param_1,0x43,0);
      break;
    case 0xbcf:
      FUN_00425d00(param_1,0x44,0);
      break;
    case 0xbd0:
      FUN_00425d00(param_1,0x65,0);
      break;
    case 0xbd1:
      FUN_00425d00(param_1,0x66,0);
      break;
    case 0xbd2:
      FUN_00425d00(param_1,0x67,0);
      break;
    case 0xbd3:
    case 0xbd4:
      BVar19 = IsWindow((HWND)param_1[0x2e]);
      if ((((BVar19 != 0) && (BVar19 = IsWindowVisible((HWND)param_1[0x2e]), BVar19 != 0)) &&
          ((bVar25 &&
           ((iVar6 = FUN_00604540(param_1 + 0x1b,param_1[0x2e]), iVar6 != 0 &&
            (pvVar11 = (void *)FUN_005f5500(param_1 + 0x11e,*(uint *)(iVar6 + 0x24)), iVar6 != 0))))
          )) && (pvVar11 != (void *)0x0)) {
        bVar25 = false;
        bVar26 = (short)param_3 == 0xbd4;
        local_f0 = (void *)0x0;
        do {
          if (bVar26) {
            pvVar11 = (void *)FUN_005f5c60((int)pvVar11);
          }
          else {
            pvVar11 = *(void **)((int)pvVar11 + 0x10);
          }
          if (pvVar11 == (void *)0x0) {
            if (!bVar25) {
              if (bVar26) {
                pvVar11 = (void *)thunk_FUN_005f5080((int)(param_1 + 0x11e));
              }
              else {
                pvVar11 = (void *)thunk_FUN_005f5060((int)(param_1 + 0x11e));
              }
              bVar25 = true;
            }
            if ((pvVar11 == (void *)0x0) && (bVar25)) break;
          }
          if ((((byte)*(int *)((int)pvVar11 + 0x18) & 0x3f) != 1) &&
             (iVar6 = FUN_004291d0(param_1,*(int *)((int)pvVar11 + 0x18)), iVar6 == 0)) {
            local_f0 = pvVar11;
          }
        } while (local_f0 == (void *)0x0);
        if (local_f0 != (void *)0x0) {
          iVar6 = FUN_00604500(param_1 + 0x1b,*(uint *)((int)local_f0 + 0x18));
          SendMessageA((HWND)param_1[6],0x467,*(WPARAM *)(iVar6 + 0x18),0);
        }
      }
      break;
    case 0xbd5:
      FUN_00401040(0xe);
      break;
    case 0xbd7:
      if ((((bVar25) && (param_1[0x30] == 1)) && (iVar6 = FUN_00520720(local_d8,0x114), iVar6 != 0))
         && (*(int *)(iVar6 + 0x1c) != 0)) {
        FUN_0041cdf0(0x114,(int)local_f8,(uint *)&local_108,0);
      }
    }
    local_4._0_1_ = 0x1d;
    FUN_00619730();
    local_4 = CONCAT31(local_4._1_3_,0x1c);
    FUN_004f4380(local_f8);
    local_4 = 0xffffffff;
    FUN_005205e0(local_d8);
  default:
switchD_00424a56_caseD_406:
LAB_00425969:
    local_d8[0] = FUN_005ff530(param_1,param_2,param_3,(LPARAM)param_4);
  }
  ExceptionList = local_c;
  return local_d8[0];
switchD_004236fd_caseD_132:
  if (bVar25) {
    ExceptionList = local_c;
    return local_d8[0];
  }
  iVar6 = param_1[0x52];
  local_d0.hwnd = *(HWND *)(iVar6 + 0x28);
  local_d0.message = *(UINT *)(iVar6 + 0x2c);
  local_d0.wParam = *(int *)(iVar6 + 0x38) + *(int *)(iVar6 + 0x28);
  local_d0.lParam = *(int *)(iVar6 + 0x3c) + *(int *)(iVar6 + 0x2c);
  pHVar16 = local_108;
  if ((param_1[0x49] != 0) &&
     (BVar19 = IsWindowVisible(*(HWND *)(param_1[0x49] + 0x18)), pHVar16 = local_108, BVar19 != 0))
  {
    iVar6 = param_1[0x92];
    local_b4.left = *(LONG *)(iVar6 + 0x28);
    local_b4.top = *(LONG *)(iVar6 + 0x2c);
    local_b4.right = *(int *)(iVar6 + 0x38) + *(int *)(iVar6 + 0x28);
    iVar27 = param_1[0x49];
    local_b4.bottom = *(int *)(iVar6 + 0x3c) + *(int *)(iVar6 + 0x2c);
    local_e8.left = *(int *)(iVar27 + 0x28);
    local_e8.top = *(int *)(iVar27 + 0x2c);
    local_e8.right = *(int *)(iVar27 + 0x38) + *(int *)(iVar27 + 0x28);
    local_e8.bottom = *(int *)(iVar27 + 0x3c);
    FUN_00428940(local_108,&local_b4.left,&local_e8.left);
    (**(code **)(*(int *)param_1[0x49] + 8))();
    (**(code **)(*(int *)param_1[0x92] + 4))(5);
  }
  for (piVar8 = (int *)FUN_005f5060((int)(param_1 + 0x1b)); piVar8 != (int *)0x0;
      piVar8 = (int *)piVar8[4]) {
    (**(code **)(*piVar8 + 8))();
  }
  iVar6 = param_1[0x46];
  if (iVar6 != 0) {
    local_e8.left = *(LONG *)(iVar6 + 0x28);
    local_e8.top = *(LONG *)(iVar6 + 0x2c);
    local_e8.right = *(int *)(iVar6 + 0x38) + *(int *)(iVar6 + 0x28);
    local_e8.bottom = *(int *)(iVar6 + 0x3c) + *(int *)(iVar6 + 0x2c);
    goto LAB_00423a44;
  }
  if (param_1[0x27] == 2) {
    pHVar7 = (HDC)FUN_00618b70(0x170);
    local_4 = 1;
    local_f8[0] = pHVar7;
    if (pHVar7 == (HDC)0x0) goto LAB_004239f2;
    iVar6 = param_1[7];
    uVar23 = FUN_005ff440((int)param_1);
    pvVar11 = FUN_00451130(pHVar7,iVar6,0x1a4,0xe9,0xa0,0xa0,param_1[0x27],(int)param_1,0x140,uVar23
                          );
  }
  else {
    pHVar7 = (HDC)FUN_00618b70(0x170);
    local_4 = 2;
    local_f8[0] = pHVar7;
    if (pHVar7 == (HDC)0x0) {
LAB_004239f2:
      pvVar11 = (void *)0x0;
    }
    else {
      iVar6 = param_1[7];
      uVar23 = FUN_005ff440((int)param_1);
      pvVar11 = FUN_00451130(pHVar7,iVar6,0x172,0xe9,0xa0,0xa0,param_1[0x27],(int)param_1,0x140,
                             uVar23);
    }
  }
  local_4 = 0xffffffff;
  param_1[0x46] = (int)pvVar11;
  FUN_005ffce0(pvVar11,0);
  iVar6 = param_1[0x46];
  local_e8.left = *(LONG *)(iVar6 + 0x28);
  local_e8.top = *(LONG *)(iVar6 + 0x2c);
  local_e8.right = *(int *)(iVar6 + 0x38) + *(int *)(iVar6 + 0x28);
  local_e8.bottom = *(int *)(iVar6 + 0x3c) + *(int *)(iVar6 + 0x2c);
LAB_00423a44:
  FUN_00428940(pHVar16,&local_e8.left,(int *)&local_d0);
  (**(code **)(*(int *)param_1[0x46] + 0x2c))(5);
  ExceptionList = local_c;
  return local_d8[0];
}

