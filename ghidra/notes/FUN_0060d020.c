
LRESULT FUN_0060d020(int *param_1,uint param_2,uint param_3,HWND param_4)

{
  int iVar1;
  int iVar2;
  int iVar3;
  POINT Point;
  POINT Point_00;
  HWND pHVar4;
  void *this;
  void *this_00;
  uint *puVar5;
  HDC pHVar6;
  int *piVar7;
  HWND pHVar8;
  BOOL BVar9;
  LRESULT LVar10;
  short sVar11;
  short sVar12;
  undefined4 uVar13;
  int iVar14;
  undefined4 uVar15;
  tagPOINT local_54;
  tagPAINTSTRUCT local_4c;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00656b8b;
  local_c = ExceptionList;
  if (param_2 < 0x10) {
    if (param_2 == 0xf) {
      pHVar8 = (HWND)param_1[6];
      ExceptionList = &local_c;
      pHVar6 = BeginPaint(pHVar8,&local_4c);
      (**(code **)(*param_1 + 0x28))(pHVar6);
      EndPaint(pHVar8,&local_4c);
      ExceptionList = local_c;
      return 0;
    }
    if (param_2 == 1) {
      pHVar8 = (HWND)param_1[6];
      iVar1 = param_1[7];
      ExceptionList = &local_c;
      SetCapture(pHVar8);
      GetCapture();
      FUN_0060cd90(param_1);
      this = (void *)FUN_00618b70(0x118);
      this_00 = (void *)0x0;
      uStack_4 = 0;
      if (this != (void *)0x0) {
        iVar2 = param_1[0xe];
        iVar3 = param_1[0xf];
        uVar15 = *(undefined4 *)(param_1[0x28] + 0xa4);
        iVar14 = iVar2 + -4;
        uVar13 = 0xbba;
        puVar5 = (uint *)FUN_005ff440((int)param_1);
        this_00 = FUN_00607ea0(this,iVar1,2,2,iVar2 + -4,iVar3 + -4,param_1,puVar5,uVar13,iVar14,
                               uVar15);
      }
      uStack_4 = 0xffffffff;
      param_1[0x27] = (int)this_00;
      FUN_00609340(this_00,param_1[0x2c]);
      FUN_005ffce0((void *)param_1[0x27],0);
      iVar14 = param_1[0x27];
      FUN_0060a490((void *)(iVar14 + 0x94),iVar1,iVar14,*(undefined2 *)(param_1[0x28] + 0xbc),
                   *(undefined2 *)(param_1[0x28] + 0xbe),param_1[0xf],0,param_1[0xf],1,
                   *(uint **)(iVar14 + 0xa8));
      *(undefined4 *)(param_1[0x27] + 0xe8) = *(undefined4 *)(param_1[0x28] + 0x9c);
      *(undefined4 *)(param_1[0x27] + 0xec) = *(undefined4 *)(param_1[0x28] + 0xa0);
      *(undefined4 *)(param_1[0x27] + 0xe4) = *(undefined4 *)(param_1[0x28] + 0x98);
      FUN_00609940((void *)param_1[0x27],*(undefined4 *)(param_1[0x28] + 0x94));
      FUN_00609950((void *)param_1[0x27],*(undefined4 *)(param_1[0x28] + 0x94));
      *(undefined4 *)(param_1[0x27] + 0xf4) = 0x70000;
      *(undefined4 *)(param_1[0x27] + 0xe0) = 0xffffff;
      *(undefined1 *)(param_1[0x27] + 0xd4) = *(undefined1 *)(param_1[0x28] + 0xac);
      FUN_00608280((void *)param_1[0x27],param_1[0x28] + 0xd0);
      FUN_0060a280((void *)param_1[0x27]);
      if (*(int *)(param_1[0x28] + 0xe0) != 0) {
        FUN_00609ed0((void *)param_1[0x27],*(int *)(*(int *)(param_1[0x28] + 0xe0) + 0xc));
      }
      if (DAT_006be3b8 == 0) {
        pHVar4 = (HWND)0x0;
      }
      else {
        pHVar4 = *(HWND *)(DAT_006be3b8 + 0x18);
      }
      SetParent(pHVar8,pHVar4);
      GetCapture();
      ExceptionList = local_c;
      return 0;
    }
    if (param_2 == 2) {
      pHVar8 = (HWND)param_1[6];
      ExceptionList = &local_c;
      pHVar4 = GetCapture();
      if (pHVar4 == pHVar8) {
        ReleaseCapture();
      }
      pHVar8 = (HWND)param_1[6];
      param_2 = 2;
      goto LAB_0060d52f;
    }
  }
  else {
    sVar12 = (short)param_4;
    sVar11 = (short)((uint)param_4 >> 0x10);
    if (param_2 < 0x201) {
      if (param_2 == 0x200) {
switchD_0060d35a_caseD_202:
        local_54.x = (LONG)sVar12;
        local_54.y = (LONG)sVar11;
        ExceptionList = &local_c;
        ClientToScreen((HWND)param_1[6],&local_54);
        Point_00.y = local_54.y;
        Point_00.x = local_54.x;
        pHVar8 = WindowFromPoint(Point_00);
        if (param_2 == 0x202) {
          GetCapture();
        }
        if (pHVar8 == (HWND)0x0) {
          ExceptionList = local_c;
          return 0;
        }
        if (pHVar8 == (HWND)param_1[6]) {
          ExceptionList = local_c;
          return 0;
        }
        BVar9 = IsChild((HWND)param_1[6],pHVar8);
        if (BVar9 == 0) {
          ExceptionList = local_c;
          return 0;
        }
        ScreenToClient(pHVar8,&local_54);
        SendMessageA(pHVar8,param_2,param_3,local_54.y << 0x10 | local_54.x & 0xffffU);
        ExceptionList = local_c;
        return 0;
      }
      if (param_2 == 0x111) {
        if ((short)param_3 != 0xbba) {
          return 0;
        }
        if (param_3 >> 0x10 != 0x29b) {
          return 0;
        }
        ExceptionList = &local_c;
        piVar7 = (int *)FUN_00609eb0(param_1[0x27]);
        if (piVar7 == (int *)0x0) {
          ExceptionList = local_c;
          return 0;
        }
        do {
          if ((*(byte *)(piVar7 + 0xf) & 1) != 0) break;
          piVar7 = (int *)(**(code **)(*piVar7 + 0xc))();
        } while (piVar7 != (int *)0x0);
        if (piVar7 == (int *)0x0) {
          ExceptionList = local_c;
          return 0;
        }
        FUN_0060c970((void *)param_1[0x28],piVar7[3]);
        ExceptionList = local_c;
        return 0;
      }
    }
    else {
      switch(param_2) {
      case 0x201:
      case 0x204:
        pHVar8 = (HWND)param_1[6];
        local_54.y = (LONG)sVar11;
        ExceptionList = &local_c;
        local_54.x = (int)sVar12;
        ClientToScreen(pHVar8,&local_54);
        Point.y = local_54.y;
        Point.x = local_54.x;
        pHVar4 = WindowFromPoint(Point);
        if ((((sVar12 < 0) || (param_1[0xe] < (int)sVar12)) || ((int)param_4 < 0)) ||
           (param_1[0xf] < (int)sVar11)) {
          SendMessageA(*(HWND *)(param_1[0x28] + 0x18),0x111,param_1[9],0);
        }
        if (pHVar4 == (HWND)0x0) {
          ExceptionList = local_c;
          return 0;
        }
        if (pHVar4 == pHVar8) {
          ExceptionList = local_c;
          return 0;
        }
        BVar9 = IsChild(pHVar8,pHVar4);
        if (BVar9 == 0) {
          ExceptionList = local_c;
          return 0;
        }
        ScreenToClient(pHVar4,&local_54);
        SendMessageA(pHVar4,param_2,param_3,local_54.y << 0x10 | local_54.x & 0xffffU);
        ExceptionList = local_c;
        return 0;
      case 0x202:
      case 0x203:
      case 0x205:
        goto switchD_0060d35a_caseD_202;
      case 0x215:
        pHVar8 = (HWND)param_1[6];
        ExceptionList = &local_c;
        GetCapture();
        if (param_4 == pHVar8) {
          ExceptionList = local_c;
          return 0;
        }
        BVar9 = IsChild(pHVar8,param_4);
        if (BVar9 != 0) {
          ExceptionList = local_c;
          return 0;
        }
        if (param_4 == (HWND)0x0) {
          ExceptionList = local_c;
          return 0;
        }
        SendMessageA(*(HWND *)(param_1[0x28] + 0x18),0x405,param_1[9] & 0xffff,(LPARAM)param_4);
        ExceptionList = local_c;
        return 0;
      }
    }
  }
  pHVar8 = (HWND)param_1[6];
  ExceptionList = &local_c;
LAB_0060d52f:
  LVar10 = DefWindowProcA(pHVar8,param_2,param_3,(LPARAM)param_4);
  ExceptionList = local_c;
  return LVar10;
}

