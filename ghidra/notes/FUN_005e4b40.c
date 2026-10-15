
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __thiscall FUN_005e4b40(void *param_1,int param_2)

{
  HWND hWnd;
  int iVar1;
  uint uVar2;
  void *pvVar3;
  undefined4 uVar4;
  undefined4 *puVar5;
  uint *puVar6;
  HDC hDC;
  char *pcVar7;
  uint uVar8;
  int iVar9;
  char *pcVar10;
  int *piVar11;
  uint uVar12;
  uint uVar13;
  float10 fVar14;
  longlong lVar15;
  LPCSTR *ppCVar16;
  undefined1 uVar17;
  undefined2 uStack_a8;
  ushort uStack_a6;
  int iStack_a4;
  void *pvStack_a0;
  undefined4 auStack_9c [3];
  HDC pHStack_90;
  undefined4 auStack_8c [3];
  undefined4 auStack_80 [3];
  undefined4 *puStack_74;
  int aiStack_70 [3];
  undefined1 auStack_64 [8];
  undefined4 auStack_5c [20];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_006550a6;
  pvStack_c = ExceptionList;
  if (param_2 == 0) {
    piVar11 = (int *)0x0;
  }
  else {
    piVar11 = (int *)(param_2 + -0xd4);
  }
  ExceptionList = &pvStack_c;
  *(int **)((int)param_1 + 0xbc) = piVar11;
  if (piVar11 != (int *)0x0) {
    iVar1 = (**(code **)(*piVar11 + 0x24))();
    uVar2 = FUN_005e4870(iVar1);
    pvVar3 = (void *)FUN_00618b70(0x28);
    uStack_4 = 0;
    pvStack_a0 = pvVar3;
    if (pvVar3 == (void *)0x0) {
      puVar5 = (undefined4 *)0x0;
    }
    else {
      uVar17 = 2;
      uVar4 = FUN_006037f0(6);
      puVar5 = FUN_005fbd20(pvVar3,uVar4,uVar2,uVar17);
    }
    uStack_4 = 0xffffffff;
    puVar6 = (uint *)FUN_005ff440((int)param_1);
    pvStack_a0 = (void *)FUN_00595c40();
    if (*(void **)((int)param_1 + 0x9c) != (void *)0x0) {
      FUN_005fcc30(*(void **)((int)param_1 + 0x9c),puVar6,0x1a,0x11,0,0,(undefined4 *)0x60,0x27);
    }
    if (puVar5 != (undefined4 *)0x0) {
      FUN_005fd0f0(puVar5,puVar6,0x1a,0x11);
      FUN_005fbfa0(puVar5);
      FUN_00618b60((undefined *)puVar5);
    }
    hWnd = *(HWND *)((int)param_1 + 0x18);
    hDC = GetDC(hWnd);
    pHStack_90 = hDC;
    FUN_00615db0(aiStack_70,1);
    uStack_4 = 1;
    if (*(int *)((int)param_1 + 0xa8) != 0) {
      pvVar3 = FUN_005b1610(*(void **)((int)param_1 + 0xbc),auStack_8c);
      uStack_4._0_1_ = 2;
      pcVar7 = (char *)FUN_00583c40((int)pvVar3);
      FUN_00601e20(*(void **)((int)param_1 + 0xa8),hWnd,hDC,pcVar7);
      uStack_4 = CONCAT31(uStack_4._1_3_,1);
      FUN_005f2ff0(auStack_8c);
    }
    if (*(int *)((int)param_1 + 0xac) != 0) {
      iVar1 = FUN_005b1480();
      if (iVar1 == 0) {
        FUN_005f2fc0(auStack_9c,&DAT_006bd568);
        uStack_4._0_1_ = 4;
        pcVar7 = (char *)FUN_00583c40((int)auStack_9c);
        pvVar3 = FUN_00616110(auStack_64,pcVar7);
        FUN_00615f00(pvVar3,0);
      }
      else {
        FUN_005f2fc0(auStack_9c,&DAT_006bd544);
        ppCVar16 = &lpString_006a7f40;
        uStack_4._0_1_ = 3;
        pcVar7 = (char *)FUN_00583c40((int)auStack_9c);
        pvVar3 = FUN_00616110(auStack_64,pcVar7);
        pvVar3 = FUN_00616110(pvVar3,(char *)ppCVar16);
        pvVar3 = FUN_00617480(pvVar3,iVar1);
        FUN_00615f00(pvVar3,0);
      }
      uStack_4 = CONCAT31(uStack_4._1_3_,1);
      FUN_005f2ff0(auStack_9c);
      FUN_00601e60(*(void **)((int)param_1 + 0xac),hWnd,hDC,aiStack_70);
    }
    iStack_a4 = 0;
    FUN_005f2f50(auStack_80);
    uStack_4._0_1_ = 5;
    if (*(int *)((int)param_1 + 0xb0) != 0) {
      uVar8 = FUN_00596af0(*(int *)((int)param_1 + 0xbc));
      uStack_a8 = 0;
      uStack_a6 = DAT_0065d428;
      uVar2 = *(uint *)(*(int *)((int)param_1 + 0xbc) + 0x174);
      if (DAT_006bc470 == (undefined4 *)0x0) {
        puStack_74 = (undefined4 *)FUN_00618b70(0x48);
        uStack_4._0_1_ = 6;
        if (puStack_74 == (undefined4 *)0x0) {
          DAT_006bc470 = (undefined4 *)0x0;
        }
        else {
          DAT_006bc470 = FUN_005a6290(puStack_74);
        }
        uStack_4._0_1_ = 5;
      }
      iVar1 = FUN_005a6460(DAT_006bc470,uVar2);
      if (pvStack_a0 == (void *)(*(int **)((int)param_1 + 0xbc))[0x5a]) {
        switch(uVar8 & 0xffff) {
        case 1:
          uStack_a8 = 0xdde1;
          break;
        case 2:
          break;
        case 3:
          uStack_a8 = 0xdde3;
          fVar14 = (float10)(**(code **)(**(int **)((int)param_1 + 0xbc) + 0x90))();
          if (fVar14 == (float10)_DAT_0066d6d4) {
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
        if (iVar1 != 0) {
          iVar9 = (**(code **)(*(int *)(iVar1 + 0x80) + 0xc))();
          if (iVar9 == 0) {
            if (iVar1 != 0) {
              iVar9 = (**(code **)(*(int *)(iVar1 + 0x98) + 0x10))();
              if (iVar9 != 0) {
                iStack_a4 = 1;
                if (*(int *)(iVar1 + 0x68) != 1) {
                  uStack_a8 = 0xdd8d;
                }
                piVar11 = (int *)(**(code **)(*(int *)(iVar1 + 0x98) + 0x10))();
                puVar5 = auStack_8c;
                pvVar3 = (void *)(**(code **)(*piVar11 + 0x10))();
                pvVar3 = FUN_005b1610(pvVar3,puVar5);
                uStack_4._0_1_ = 7;
                FUN_005f3090(auStack_80,(int)pvVar3);
                uStack_4._0_1_ = 5;
                FUN_005f2ff0(auStack_8c);
              }
            }
          }
          else {
            uStack_a8 = 0xdd8e;
            iStack_a4 = 0;
            fVar14 = (float10)(**(code **)(**(int **)((int)param_1 + 0xbc) + 0x90))();
            if (fVar14 == (float10)_DAT_0066d6d4) {
              uStack_a8 = 0xdd91;
            }
          }
        }
      }
      else {
        uStack_a8 = 0xddbf;
      }
      FUN_005f2fc0(auStack_9c,&uStack_a8);
      uStack_4 = CONCAT31(uStack_4._1_3_,8);
      pcVar7 = (char *)FUN_00583c40((int)auStack_9c);
      FUN_00616110(auStack_64,pcVar7);
      if (iStack_a4 != 0) {
        pcVar7 = (char *)FUN_00583c40((int)auStack_80);
        pvVar3 = FUN_00616110(auStack_64,(char *)&lpString_006a7f40);
        FUN_00616110(pvVar3,pcVar7);
      }
      FUN_00615f00(auStack_64,0);
      FUN_00601e60(*(void **)((int)param_1 + 0xb0),hWnd,pHStack_90,aiStack_70);
      uStack_4._0_1_ = 5;
      FUN_005f2ff0(auStack_9c);
      hDC = pHStack_90;
    }
    if (*(int *)((int)param_1 + 0xb4) != 0) {
      FUN_005f2fc0(auStack_8c,&DAT_006bd53c);
      iStack_a4 = (uint)DAT_0065d428 << 0x10;
      uStack_4._0_1_ = 9;
      if (pvStack_a0 == (void *)*(int *)(*(int *)((int)param_1 + 0xbc) + 0x168)) {
        iVar1 = FUN_00596b30(*(int *)((int)param_1 + 0xbc));
        iStack_a4 = CONCAT22(iStack_a4._2_2_,-0x2224 - (ushort)(iVar1 != 1));
        FUN_005f2fc0(auStack_9c,&iStack_a4);
        uStack_4._0_1_ = 10;
        pcVar7 = (char *)FUN_00583c40((int)auStack_9c);
        ppCVar16 = &lpString_006a7f40;
        pcVar10 = (char *)FUN_00583c40((int)auStack_8c);
        pvVar3 = FUN_00616110(auStack_64,pcVar10);
        pvVar3 = FUN_00616110(pvVar3,(char *)ppCVar16);
        pvVar3 = FUN_00616110(pvVar3,pcVar7);
        FUN_00615f00(pvVar3,0);
        FUN_00601e60(*(void **)((int)param_1 + 0xb4),hWnd,hDC,aiStack_70);
        uStack_4._0_1_ = 9;
        FUN_005f2ff0(auStack_9c);
      }
      uStack_4._0_1_ = 5;
      FUN_005f2ff0(auStack_8c);
    }
    ReleaseDC(hWnd,hDC);
    if (*(int *)((int)param_1 + 0xa4) != 0) {
      FUN_005b1650(*(int *)((int)param_1 + 0xbc));
      lVar15 = __ftol();
      FUN_0060e440(*(void **)((int)param_1 + 0xa4),(int)lVar15);
    }
    if (*(int *)((int)param_1 + 0xa0) != 0) {
      FUN_005b1640(*(void **)((int)param_1 + 0xbc));
      lVar15 = __ftol();
      FUN_0060e440(*(void **)((int)param_1 + 0xa0),(int)lVar15);
    }
    piVar11 = (int *)FUN_00604500((void *)((int)param_1 + 0x6c),0xdd01);
    if (piVar11 != (int *)0x0) {
      iVar1 = FUN_00596830(*(int *)((int)param_1 + 0xbc));
      if (iVar1 == 0) {
        (**(code **)(*piVar11 + 8))();
        (**(code **)(*piVar11 + 0x10))();
      }
      else {
        (**(code **)(*piVar11 + 0xc))();
        (**(code **)(*piVar11 + 4))(5);
      }
    }
    piVar11 = (int *)FUN_00604500((void *)((int)param_1 + 0x6c),0xdd02);
    if (piVar11 != (int *)0x0) {
      iVar1 = FUN_00596830(*(int *)((int)param_1 + 0xbc));
      if (iVar1 == 0) {
        (**(code **)(*piVar11 + 8))();
        (**(code **)(*piVar11 + 0x10))();
      }
      else {
        (**(code **)(*piVar11 + 0xc))();
        (**(code **)(*piVar11 + 4))(5);
      }
    }
    FUN_005b16e0(*(int *)((int)param_1 + 0xbc));
    lVar15 = __ftol();
    uVar2 = (uint)lVar15;
    FUN_005b16b0(*(int **)((int)param_1 + 0xbc));
    lVar15 = __ftol();
    uVar8 = (uint)lVar15;
    FUN_005b1740(*(void **)((int)param_1 + 0xbc));
    lVar15 = __ftol();
    uVar12 = (uint)lVar15;
    FUN_005b1680(*(int **)((int)param_1 + 0xbc));
    lVar15 = __ftol();
    uVar13 = (uint)lVar15;
    FUN_005b1710(*(int **)((int)param_1 + 0xbc));
    lVar15 = __ftol();
    FUN_005e45f0(param_1,(uint)lVar15,uVar13,uVar12,uVar8,uVar2);
    if (*(void **)((int)param_1 + 0xb8) != (void *)0x0) {
      FUN_00609920(*(void **)((int)param_1 + 0xb8));
      FUN_00595b30(*(void **)((int)param_1 + 0xb8));
    }
    InvalidateRect(*(HWND *)((int)param_1 + 0x18),(RECT *)0x0,0);
    uStack_4 = CONCAT31(uStack_4._1_3_,1);
    FUN_005f2ff0(auStack_80);
    uStack_4 = 0xffffffff;
    FUN_00615ea0((int)auStack_5c);
    FUN_00615440(auStack_5c);
  }
  ExceptionList = pvStack_c;
  return;
}

