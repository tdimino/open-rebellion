
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __thiscall FUN_005e7b10(int param_1,int param_2)

{
  HWND hWnd;
  undefined2 uVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  int iVar4;
  HDC hDC;
  int iVar5;
  undefined4 uVar6;
  undefined4 uVar7;
  int *piVar8;
  float10 fVar9;
  undefined1 *puVar10;
  undefined4 uVar11;
  undefined2 uStack_a8;
  ushort uStack_a6;
  int iStack_a4;
  int iStack_a0;
  HDC pHStack_90;
  undefined1 auStack_8c [24];
  int iStack_74;
  undefined1 auStack_70 [100];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00655266;
  pvStack_c = ExceptionList;
  if (param_2 == 0) {
    piVar8 = (int *)0x0;
  }
  else {
    piVar8 = (int *)(param_2 + -0xd4);
  }
  ExceptionList = &pvStack_c;
  *(int **)(param_1 + 0xc0) = piVar8;
  if (piVar8 != (int *)0x0) {
    uVar2 = (**(code **)(*piVar8 + 0x24))();
    uVar2 = FUN_005e4870(uVar2);
    iStack_a0 = FUN_00618b70(0x28);
    uStack_4 = 0;
    if (iStack_a0 == 0) {
      iVar4 = 0;
    }
    else {
      uVar11 = 2;
      uVar3 = FUN_006037f0(6,uVar2,2);
      iVar4 = FUN_005fbd20(uVar3,uVar2,uVar11);
    }
    uStack_4 = 0xffffffff;
    uVar2 = FUN_005ff440();
    iStack_a0 = FUN_00595c40();
    if (*(int *)(param_1 + 0x9c) != 0) {
      FUN_005fcc30(uVar2,0x1a,0x11,0,0,0x60,0x27);
    }
    if (iVar4 != 0) {
      FUN_005fd0f0(uVar2,0x1a,0x11);
      FUN_005fbfa0();
      FUN_00618b60(iVar4);
    }
    hWnd = *(HWND *)(param_1 + 0x18);
    hDC = GetDC(hWnd);
    pHStack_90 = hDC;
    FUN_00615db0(1);
    uStack_4 = 1;
    if (*(int *)(param_1 + 0xa8) != 0) {
      FUN_005b1610(auStack_8c);
      uStack_4._0_1_ = 2;
      uVar2 = FUN_00583c40();
      FUN_00601e20(hWnd,hDC,uVar2);
      uStack_4 = CONCAT31(uStack_4._1_3_,1);
      FUN_005f2ff0();
    }
    if (*(int *)(param_1 + 0xac) != 0) {
      iVar4 = FUN_005b1480();
      if (iVar4 == 0) {
        FUN_005f2fc0(&DAT_006bd778);
        uStack_4._0_1_ = 4;
        uVar2 = FUN_00583c40();
        FUN_00616110(uVar2);
        FUN_00615f00(0);
      }
      else {
        FUN_005f2fc0(&DAT_006bd754);
        puVar10 = &DAT_006a7f40;
        uStack_4._0_1_ = 3;
        uVar2 = FUN_00583c40(&DAT_006a7f40,iVar4);
        FUN_00616110(uVar2);
        FUN_00616110(puVar10);
        FUN_00617480(iVar4);
        FUN_00615f00(0);
      }
      uStack_4 = CONCAT31(uStack_4._1_3_,1);
      FUN_005f2ff0();
      FUN_00601e60(hWnd,hDC,auStack_70);
    }
    iStack_a4 = 0;
    FUN_005f2f50();
    uStack_4._0_1_ = 5;
    if (*(int *)(param_1 + 0xb0) != 0) {
      uVar1 = FUN_00596af0(*(undefined4 *)(param_1 + 0xc0));
      uStack_a8 = 0;
      uStack_a6 = DAT_0065d428;
      uVar2 = *(undefined4 *)(*(int *)(param_1 + 0xc0) + 0x174);
      if (DAT_006bc470 == 0) {
        iStack_74 = FUN_00618b70(0x48);
        uStack_4._0_1_ = 6;
        if (iStack_74 == 0) {
          DAT_006bc470 = 0;
        }
        else {
          DAT_006bc470 = FUN_005a6290();
        }
        uStack_4._0_1_ = 5;
      }
      iVar4 = FUN_005a6460(uVar2);
      if (iStack_a0 == (*(int **)(param_1 + 0xc0))[0x5a]) {
        switch(uVar1) {
        case 1:
          uStack_a8 = 0xdde1;
          break;
        case 2:
          break;
        case 3:
          uStack_a8 = 0xdde3;
          fVar9 = (float10)(**(code **)(**(int **)(param_1 + 0xc0) + 0x90))();
          if (fVar9 == (float10)_DAT_0066d744) {
            uStack_a8 = 0xdd91;
          }
          break;
        case 4:
          uStack_a8 = 0xddeb;
          break;
        case 5:
          uStack_a8 = 0xdde5;
          break;
        case 6:
          uStack_a8 = 0xdde6;
          break;
        case 7:
          uStack_a8 = 0xdde7;
          break;
        case 8:
          uStack_a8 = 0xdde8;
          break;
        case 9:
          uStack_a8 = 0xdde9;
          break;
        case 10:
          uStack_a8 = 0xddea;
          break;
        default:
          uStack_a8 = 0xdd8f;
        }
        if (iVar4 != 0) {
          iVar5 = (**(code **)(*(int *)(iVar4 + 0x80) + 0xc))();
          if (iVar5 == 0) {
            if (iVar4 != 0) {
              iVar5 = (**(code **)(*(int *)(iVar4 + 0x98) + 0x10))();
              if (iVar5 != 0) {
                iStack_a4 = 1;
                if (*(int *)(iVar4 + 0x68) != 1) {
                  uStack_a8 = 0xdd8d;
                }
                piVar8 = (int *)(**(code **)(*(int *)(iVar4 + 0x98) + 0x10))();
                puVar10 = auStack_8c;
                (**(code **)(*piVar8 + 0x10))(puVar10);
                uVar2 = FUN_005b1610(puVar10);
                uStack_4._0_1_ = 7;
                FUN_005f3090(uVar2);
                uStack_4._0_1_ = 5;
                FUN_005f2ff0();
              }
            }
          }
          else {
            uStack_a8 = 0xdd8e;
            iStack_a4 = 0;
            fVar9 = (float10)(**(code **)(**(int **)(param_1 + 0xc0) + 0x90))();
            if (fVar9 == (float10)_DAT_0066d744) {
              uStack_a8 = 0xdd91;
            }
          }
        }
      }
      else {
        uStack_a8 = 0xddbf;
      }
      FUN_005f2fc0(&uStack_a8);
      uStack_4 = CONCAT31(uStack_4._1_3_,8);
      uVar2 = FUN_00583c40();
      FUN_00616110(uVar2);
      if (iStack_a4 != 0) {
        uVar2 = FUN_00583c40();
        FUN_00616110(&DAT_006a7f40);
        FUN_00616110(uVar2);
      }
      FUN_00615f00(0);
      FUN_00601e60(hWnd,pHStack_90,auStack_70);
      uStack_4._0_1_ = 5;
      FUN_005f2ff0();
      hDC = pHStack_90;
    }
    if (*(int *)(param_1 + 0xb4) != 0) {
      FUN_005f2fc0(&DAT_006bd74c);
      iStack_a4 = (uint)DAT_0065d428 << 0x10;
      uStack_4._0_1_ = 9;
      if (iStack_a0 == *(int *)(*(int *)(param_1 + 0xc0) + 0x168)) {
        iVar4 = FUN_00596b30(*(int *)(param_1 + 0xc0));
        iStack_a4 = CONCAT22(iStack_a4._2_2_,-0x2224 - (ushort)(iVar4 != 1));
        FUN_005f2fc0(&iStack_a4);
        uStack_4._0_1_ = 10;
        uVar2 = FUN_00583c40();
        puVar10 = &DAT_006a7f40;
        uVar3 = FUN_00583c40(&DAT_006a7f40,uVar2);
        FUN_00616110(uVar3);
        FUN_00616110(puVar10);
        FUN_00616110(uVar2);
        FUN_00615f00(0);
        FUN_00601e60(hWnd,hDC,auStack_70);
        uStack_4._0_1_ = 9;
        FUN_005f2ff0();
      }
      uStack_4._0_1_ = 5;
      FUN_005f2ff0();
    }
    ReleaseDC(hWnd,hDC);
    if (*(int *)(param_1 + 0xa4) != 0) {
      FUN_005b1650();
      uVar2 = __ftol();
      FUN_0060e440(uVar2);
    }
    if (*(int *)(param_1 + 0xa0) != 0) {
      FUN_005b1640();
      uVar2 = __ftol();
      FUN_0060e440(uVar2);
    }
    FUN_005b16e0();
    uVar2 = __ftol();
    FUN_005b16b0(uVar2);
    uVar3 = __ftol();
    FUN_005b1740(uVar3);
    uVar11 = __ftol();
    FUN_005b1680(uVar11);
    uVar6 = __ftol();
    FUN_005b1710(uVar6);
    uVar7 = __ftol();
    FUN_005e7540(uVar7,uVar6,uVar11,uVar3,uVar2);
    if (*(int *)(param_1 + 0xb8) != 0) {
      FUN_00609920();
      FUN_00595b30(*(undefined4 *)(param_1 + 0xb8));
    }
    if (*(int *)(param_1 + 0xbc) != 0) {
      iVar4 = FUN_00596830(*(undefined4 *)(param_1 + 0xc0));
      if (iVar4 == 0) {
        (**(code **)(**(int **)(param_1 + 0xbc) + 0x10))();
        (**(code **)(**(int **)(param_1 + 0xbc) + 8))();
      }
      else {
        (**(code **)(**(int **)(param_1 + 0xbc) + 4))(5);
        (**(code **)(**(int **)(param_1 + 0xbc) + 0xc))();
      }
    }
    piVar8 = (int *)FUN_00604500(0xdd07);
    if (piVar8 != (int *)0x0) {
      iVar4 = FUN_00596830(*(undefined4 *)(param_1 + 0xc0));
      if (iVar4 == 0) {
        (**(code **)(*piVar8 + 0x10))();
        (**(code **)(*piVar8 + 8))();
      }
      else {
        (**(code **)(*piVar8 + 4))(5);
        (**(code **)(*piVar8 + 0xc))();
      }
    }
    InvalidateRect(*(HWND *)(param_1 + 0x18),(RECT *)0x0,0);
    uStack_4 = CONCAT31(uStack_4._1_3_,1);
    FUN_005f2ff0();
    uStack_4 = 0xffffffff;
    FUN_00615ea0();
    FUN_00615440();
  }
  ExceptionList = pvStack_c;
  return;
}

