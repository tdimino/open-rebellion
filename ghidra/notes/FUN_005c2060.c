
LRESULT FUN_005c2060(int *param_1,uint param_2,WPARAM param_3,uint param_4)

{
  int iVar1;
  undefined4 *puVar2;
  int *piVar3;
  HINSTANCE pHVar4;
  HCURSOR pHVar5;
  BOOL BVar6;
  LRESULT LVar7;
  uint uVar8;
  HCURSOR pHVar9;
  uint uVar10;
  LRESULT unaff_ESI;
  int *piVar11;
  undefined4 unaff_retaddr;
  LPCSTR pCVar12;
  undefined1 auStack_50 [16];
  undefined1 auStack_40 [52];
  int *piStack_c;
  int *piStack_8;
  int *piStack_4;
  
  piVar3 = param_1;
  if (param_2 < 7) {
    if (param_2 != 6) {
      if (param_2 == 1) {
        pCVar12 = (LPCSTR)0x2b;
        pHVar4 = (HINSTANCE)FUN_005fefd0(6);
        pHVar5 = LoadCursorA(pHVar4,pCVar12);
        piVar3 = param_1;
        pCVar12 = (LPCSTR)0x2a;
        param_1[0x194] = (int)pHVar5;
        pHVar4 = (HINSTANCE)FUN_005fefd0(6);
        pHVar5 = LoadCursorA(pHVar4,pCVar12);
        piVar3[0x193] = (int)pHVar5;
        return 0;
      }
LAB_005c22b5:
      LVar7 = DefWindowProcA((HWND)param_1[6],param_2,param_3,param_4);
      return LVar7;
    }
    puVar2 = (undefined4 *)param_1[0x4c];
    if (puVar2 != (undefined4 *)0x0) {
      DAT_006bcbb4 = (**(code **)*puVar2)(puVar2,&DAT_0066e418,&param_1);
      if (DAT_006bcbb4 != 0) {
        FUN_005d93f0(0,DAT_006bcbb4);
      }
      if (DAT_006bcbb4 == 0) {
        DAT_006bcbb4 = (**(code **)(*piStack_8 + 0x30))(piStack_8,unaff_retaddr);
        if (DAT_006bcbb4 != 0) {
          FUN_005d93f0(0,DAT_006bcbb4);
        }
        (**(code **)(*(int *)auStack_40._48_4_ + 8))(auStack_40._48_4_);
      }
      DAT_006bcbb4 = (**(code **)(*(int *)piVar3[0x55] + 0x7c))((int *)piVar3[0x55],piVar3[0x59]);
      if (DAT_006bcbb4 != 0) {
        FUN_005d93f0(0,DAT_006bcbb4);
        return 0;
      }
    }
  }
  else if (param_2 < 0x19) {
    if (param_2 == 0x18) {
      LVar7 = FUN_006007b0(param_1,0x18,param_3,param_4);
      return LVar7;
    }
    if (param_2 == 0xf) {
      if (param_1[0x4c] == 0) {
        LVar7 = DefWindowProcA((HWND)param_1[6],0xf,param_3,param_4);
        return LVar7;
      }
      BVar6 = GetUpdateRect((HWND)param_1[6],(LPRECT)auStack_50,0);
      if (BVar6 != 0) {
        BeginPaint((HWND)piVar3[6],(LPPAINTSTRUCT)auStack_40);
        DAT_006bcbb4 = (*(code *)**(undefined4 **)piVar3[0x4c])
                                 ((undefined4 *)piVar3[0x4c],&DAT_0066e418,&param_2);
        if (DAT_006bcbb4 != 0) {
          FUN_005d93f0(0,DAT_006bcbb4);
        }
        if (DAT_006bcbb4 == 0) {
          DAT_006bcbb4 = (**(code **)(*piStack_4 + 0x2c))(piStack_4,auStack_50._4_4_);
          if (DAT_006bcbb4 != 0) {
            FUN_005d93f0(0,DAT_006bcbb4);
          }
          (**(code **)(*piStack_c + 8))(piStack_c);
        }
        EndPaint((HWND)piVar3[6],(PAINTSTRUCT *)(auStack_50 + 4));
        return unaff_ESI;
      }
    }
    else if (param_2 != 0x10) goto LAB_005c22b5;
  }
  else {
    if (param_2 != 0x200) {
      if (param_2 == 0x201) {
        pHVar5 = (HCURSOR)param_1[0x194];
        if (param_1[0x192] == 1) {
          pHVar5 = (HCURSOR)param_1[0x193];
        }
        pHVar9 = GetCursor();
        if (pHVar5 != pHVar9) {
          SetCursor(pHVar5);
        }
        uVar8 = param_4 >> 0x10;
        piVar11 = (int *)(param_4 & 0xffff);
        uVar10 = FUN_005c2d20(piVar3,piVar11,uVar8);
        if (uVar10 == 0) {
          param_1 = (int *)((int)piVar11 + -1);
          uVar10 = FUN_005c2d20(piVar3,param_1,uVar8 - 1);
          if (uVar10 == 0) {
            uVar10 = FUN_005c2d20(piVar3,(int *)((int)piVar11 + 1),uVar8 - 1);
            if (uVar10 == 0) {
              uVar10 = FUN_005c2d20(piVar3,(int *)((int)piVar11 + 1),uVar8 + 1);
              if ((uVar10 == 0) && (uVar10 = FUN_005c2d20(piVar3,param_1,uVar8 + 1), uVar10 == 0)) {
                if ((param_3 & 8) != 0) {
                  return 0;
                }
                FUN_00595b70(0,0);
                return 0;
              }
            }
          }
        }
        if ((param_3 & 8) != 0) {
          FUN_00595b70(uVar10,1);
          return 0;
        }
        FUN_005c0e60();
        FUN_005c0e70();
        FUN_00595b70(0,0);
        FUN_00595b70(uVar10,0);
        return 0;
      }
      if (param_2 == 0x204) {
        uVar10 = param_4 >> 0x10;
        piVar11 = (int *)(param_4 & 0xffff);
        uVar8 = FUN_005c2d20(param_1,piVar11,uVar10);
        if (uVar8 == 0) {
          uVar8 = FUN_005c2d20(piVar3,(int *)((int)piVar11 + -1),uVar10 - 1);
          if ((uVar8 == 0) &&
             (uVar8 = FUN_005c2d20(piVar3,(int *)((int)piVar11 + 1),uVar10 - 1), uVar8 == 0)) {
            uVar8 = FUN_005c2d20(piVar3,(int *)((int)piVar11 + 1),uVar10 + 1);
            if ((uVar8 == 0) &&
               (uVar8 = FUN_005c2d20(piVar3,(int *)((int)piVar11 + -1),uVar10 + 1), uVar8 == 0)) {
              iVar1 = uVar10 - 2;
              param_1 = (int *)((int)piVar11 + -2);
              uVar8 = FUN_005c2d20(piVar3,param_1,iVar1);
              if ((((uVar8 == 0) && (uVar8 = FUN_005c2d20(piVar3,piVar11,iVar1), uVar8 == 0)) &&
                  (uVar8 = FUN_005c2d20(piVar3,(int *)((int)piVar11 + 2),iVar1), uVar8 == 0)) &&
                 (uVar8 = FUN_005c2d20(piVar3,(int *)((int)piVar11 + 2),uVar10), uVar8 == 0)) {
                iVar1 = uVar10 + 2;
                uVar8 = FUN_005c2d20(piVar3,(int *)((int)piVar11 + 2),iVar1);
                if (((uVar8 == 0) &&
                    (uVar8 = FUN_005c2d20(piVar3,piVar11,iVar1), piVar11 = param_1, uVar8 == 0)) &&
                   ((uVar8 = FUN_005c2d20(piVar3,param_1,iVar1), uVar8 == 0 &&
                    (uVar8 = FUN_005c2d20(piVar3,piVar11,uVar10), uVar8 == 0)))) {
                  return 0;
                }
              }
            }
          }
        }
        if ((param_3 & 8) != 0) {
          FUN_00595b70(uVar8,1);
          return 0;
        }
        FUN_00595b70(uVar8,0);
        return 0;
      }
      goto LAB_005c22b5;
    }
    pHVar5 = (HCURSOR)param_1[0x194];
    if (param_1[0x192] == 1) {
      pHVar5 = (HCURSOR)param_1[0x193];
    }
    pHVar9 = GetCursor();
    if (pHVar5 != pHVar9) {
      SetCursor(pHVar5);
    }
  }
  return 0;
}

