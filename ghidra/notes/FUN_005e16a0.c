
void __thiscall FUN_005e16a0(int param_1,int param_2)

{
  HWND hWnd;
  bool bVar1;
  void *pvVar2;
  short sVar3;
  HDC hDC;
  undefined4 uVar4;
  int iVar5;
  undefined4 uVar6;
  int *piVar7;
  undefined4 uVar8;
  undefined4 unaff_EBX;
  short unaff_SI;
  int iVar9;
  undefined1 *puVar10;
  undefined *puVar11;
  undefined1 *puVar12;
  undefined4 uStack_10c;
  undefined4 uStack_104;
  undefined1 auStack_100 [4];
  undefined4 uStack_fc;
  undefined4 uStack_f8;
  HDC pHStack_f4;
  HWND pHStack_f0;
  undefined1 auStack_d4 [92];
  int aiStack_78 [2];
  undefined4 auStack_70 [11];
  int iStack_44;
  undefined1 auStack_40 [52];
  void *pvStack_c;
  undefined1 *puStack_8;
  int iStack_4;
  
  pvVar2 = ExceptionList;
  iStack_4 = 0xffffffff;
  puStack_8 = &LAB_00654e60;
  pvStack_c = ExceptionList;
  bVar1 = false;
  ExceptionList = &pvStack_c;
  *(int *)(param_1 + 0x9c) = param_2;
  if (param_2 == 0) {
    ExceptionList = pvVar2;
    return;
  }
  hWnd = *(HWND *)(param_1 + 0x18);
  uStack_10c._0_2_ = 0;
  uStack_10c._2_2_ = DAT_0065d428;
  pHStack_f0 = hWnd;
  hDC = GetDC(hWnd);
  pHStack_f4 = hDC;
  if (*(int *)(param_1 + 0xa4) != 0) {
    uVar4 = FUN_005c7150();
    switch(uVar4) {
    case 0:
      uStack_10c._0_2_ = 0xdd88;
      break;
    case 1:
      uStack_10c._0_2_ = 0xdd89;
      break;
    case 2:
      uStack_10c._0_2_ = 0xdd8a;
      break;
    case 3:
      uStack_10c._0_2_ = 0xdd8b;
    }
    FUN_00601de0(hWnd,hDC,&uStack_10c);
  }
  FUN_005f2f50();
  iStack_4 = 0;
  if (*(int *)(param_1 + 0xa8) != 0) {
    switch(*(undefined4 *)(*(int *)(param_1 + 0x9c) + 0x68)) {
    case 1:
      uStack_10c._0_2_ = 0xdde1;
      break;
    case 2:
      uStack_10c._0_2_ = 0xdde2;
      break;
    case 3:
      uStack_10c._0_2_ = 0xdde3;
      break;
    case 4:
      uStack_10c._0_2_ = 0xddeb;
      break;
    case 5:
      uStack_10c._0_2_ = 0xdde5;
      break;
    case 6:
      uStack_10c._0_2_ = 0xdde6;
      break;
    case 7:
      uStack_10c._0_2_ = 0xdde7;
      break;
    case 8:
      uStack_10c._0_2_ = 0xdde8;
      break;
    case 9:
      uStack_10c._0_2_ = 0xdde9;
      break;
    case 10:
      uStack_10c._0_2_ = 0xddea;
      break;
    default:
      uStack_10c._0_2_ = 0xdd8f;
    }
    iVar5 = (**(code **)(*(int *)(*(int *)(param_1 + 0x9c) + 0x80) + 0xc))();
    if (iVar5 == 0) {
      iVar5 = (**(code **)(*(int *)(*(int *)(param_1 + 0x9c) + 0x98) + 0x10))();
      if (iVar5 != 0) {
        bVar1 = true;
        if (*(int *)(*(int *)(param_1 + 0x9c) + 0x68) != 1) {
          uStack_10c._0_2_ = 0xdd8d;
        }
        piVar7 = (int *)(**(code **)(*(int *)(*(int *)(param_1 + 0x9c) + 0x98) + 0x10))();
        puVar12 = auStack_100;
        (**(code **)(*piVar7 + 0x10))(puVar12);
        uVar4 = FUN_005b1610(puVar12);
        iStack_4._0_1_ = 1;
        FUN_005f3090(uVar4);
        iStack_4 = (uint)iStack_4._1_3_ << 8;
        FUN_005f2ff0();
      }
    }
    else {
      bVar1 = false;
      uStack_10c._0_2_ = 0xdd8e;
    }
    FUN_005f2fc0(&uStack_10c);
    iStack_4._0_1_ = 2;
    FUN_00615db0(1);
    iStack_4 = CONCAT31(iStack_4._1_3_,3);
    uVar4 = FUN_00583c40();
    FUN_00616110(uVar4);
    if (bVar1) {
      uVar4 = FUN_00583c40();
      FUN_00616110(&DAT_006a7f40);
      FUN_00616110(uVar4);
    }
    FUN_00615f00(0);
    FUN_00601e60(hWnd,hDC,auStack_d4);
    iStack_4._0_1_ = 2;
    FUN_00615ea0();
    FUN_00615440();
    iStack_4 = (uint)iStack_4._1_3_ << 8;
    FUN_005f2ff0();
  }
  if (*(int *)(param_1 + 0xac) != 0) {
    FUN_005f2fc0(&DAT_006bd32c);
    iStack_4._0_1_ = 4;
    uStack_104._0_2_ = 0;
    uStack_104._2_2_ = DAT_0065d428;
    iVar5 = FUN_00596b60(*(undefined4 *)(param_1 + 0x9c));
    uStack_104 = CONCAT22(uStack_104._2_2_,-0x2224 - (ushort)(iVar5 != 1));
    FUN_005f2fc0(&uStack_104);
    iStack_4._0_1_ = 5;
    FUN_00615db0(1);
    iStack_4._0_1_ = 6;
    uVar4 = FUN_00583c40();
    puVar12 = &DAT_006a7f40;
    uVar6 = FUN_00583c40(&DAT_006a7f40,uVar4);
    FUN_00616110(uVar6);
    FUN_00616110(puVar12);
    FUN_00616110(uVar4);
    FUN_00615f00(0);
    FUN_00601e60(hWnd,hDC,auStack_d4);
    iStack_4._0_1_ = 5;
    FUN_00615ea0();
    FUN_00615440();
    iStack_4._0_1_ = 4;
    FUN_005f2ff0();
    iStack_4 = (uint)iStack_4._1_3_ << 8;
    FUN_005f2ff0();
  }
  ReleaseDC(hWnd,hDC);
  piVar7 = (int *)FUN_00604500(0xdd01);
  if (piVar7 != (int *)0x0) {
    iVar5 = FUN_00596850(*(undefined4 *)(param_1 + 0x9c));
    if (iVar5 == 0) {
      (**(code **)(*piVar7 + 8))();
      (**(code **)(*piVar7 + 0x10))();
    }
    else {
      (**(code **)(*piVar7 + 0xc))();
      (**(code **)(*piVar7 + 4))(5);
    }
  }
  piVar7 = (int *)FUN_00604500(0xdd02);
  if (piVar7 != (int *)0x0) {
    iVar5 = FUN_00596850(*(undefined4 *)(param_1 + 0x9c));
    if (iVar5 == 0) {
      (**(code **)(*piVar7 + 8))();
      (**(code **)(*piVar7 + 0x10))();
    }
    else {
      (**(code **)(*piVar7 + 0xc))();
      (**(code **)(*piVar7 + 4))(5);
    }
  }
  uVar4 = FUN_006037f0(6);
  uStack_104 = FUN_005ff440();
  iVar9 = 0;
  uStack_10c = (uint)uStack_10c._2_2_ << 0x10;
  iVar5 = FUN_00595c40();
  if (iVar5 == 0) {
    iVar5 = (**(code **)(**(int **)(param_1 + 0x9c) + 0x3c))(0x1d);
    if (iVar5 == 0) {
      iVar5 = (**(code **)(**(int **)(param_1 + 0x9c) + 0x3c))(0x1e);
      if (iVar5 == 0) {
        iVar5 = (**(code **)(**(int **)(param_1 + 0x9c) + 0x3c))(0x1f);
        if (iVar5 == 0) {
          iVar5 = (**(code **)(**(int **)(param_1 + 0x9c) + 0x3c))(0x20);
          if (iVar5 == 0) goto LAB_005e1dcc;
          unaff_EBX = (**(code **)(**(int **)(param_1 + 0x9c) + 0x3c))(0x20);
          unaff_SI = -0x2238;
          iVar5 = FUN_00618b70(0x28);
          pvStack_c._0_1_ = 10;
          sVar3 = -0x2238;
          if (iVar5 != 0) {
            iVar9 = FUN_005fbd20(uVar4,0x7f1,2);
            pvStack_c = (void *)((uint)pvStack_c._1_3_ << 8);
            goto LAB_005e1dcc;
          }
        }
        else {
          unaff_EBX = (**(code **)(**(int **)(param_1 + 0x9c) + 0x3c))(0x1f);
          unaff_SI = -0x2239;
          iVar5 = FUN_00618b70(0x28);
          pvStack_c._0_1_ = 9;
          sVar3 = -0x2239;
          if (iVar5 != 0) {
            iVar9 = FUN_005fbd20(uVar4,0x7f0,2);
            pvStack_c = (void *)((uint)pvStack_c._1_3_ << 8);
            goto LAB_005e1dcc;
          }
        }
      }
      else {
        unaff_EBX = (**(code **)(**(int **)(param_1 + 0x9c) + 0x3c))(0x1e);
        unaff_SI = -0x223a;
        iVar5 = FUN_00618b70(0x28);
        pvStack_c._0_1_ = 8;
        sVar3 = -0x223a;
        if (iVar5 != 0) {
          iVar9 = FUN_005fbd20(uVar4,0x7ef,2);
          pvStack_c = (void *)((uint)pvStack_c._1_3_ << 8);
          goto LAB_005e1dcc;
        }
      }
    }
    else {
      unaff_EBX = (**(code **)(**(int **)(param_1 + 0x9c) + 0x3c))(0x1d);
      unaff_SI = -0x223b;
      iVar5 = FUN_00618b70(0x28);
      pvStack_c._0_1_ = 7;
      sVar3 = -0x223b;
      if (iVar5 != 0) {
        iVar9 = FUN_005fbd20(uVar4,0x7ee,2);
        pvStack_c = (void *)((uint)pvStack_c._1_3_ << 8);
        goto LAB_005e1dcc;
      }
    }
  }
  else {
    iVar5 = (**(code **)(**(int **)(param_1 + 0x9c) + 0x3c))(0x21);
    if (iVar5 == 0) {
      iVar5 = (**(code **)(**(int **)(param_1 + 0x9c) + 0x3c))(0x22);
      if (iVar5 == 0) {
        iVar5 = (**(code **)(**(int **)(param_1 + 0x9c) + 0x3c))(0x23);
        if (iVar5 == 0) {
          iVar5 = (**(code **)(**(int **)(param_1 + 0x9c) + 0x3c))(0x24);
          if (iVar5 == 0) goto LAB_005e1dcc;
          unaff_EBX = (**(code **)(**(int **)(param_1 + 0x9c) + 0x3c))(0x24);
          unaff_SI = -0x2234;
          iVar5 = FUN_00618b70(0x28);
          pvStack_c._0_1_ = 0xe;
          sVar3 = -0x2234;
          if (iVar5 != 0) {
            iVar9 = FUN_005fbd20(uVar4,0x7f5,2);
            pvStack_c = (void *)((uint)pvStack_c._1_3_ << 8);
            goto LAB_005e1dcc;
          }
        }
        else {
          unaff_EBX = (**(code **)(**(int **)(param_1 + 0x9c) + 0x3c))(0x23);
          unaff_SI = -0x2235;
          iVar5 = FUN_00618b70(0x28);
          pvStack_c._0_1_ = 0xd;
          sVar3 = -0x2235;
          if (iVar5 != 0) {
            iVar9 = FUN_005fbd20(uVar4,0x7f4,2);
            pvStack_c = (void *)((uint)pvStack_c._1_3_ << 8);
            goto LAB_005e1dcc;
          }
        }
      }
      else {
        unaff_EBX = (**(code **)(**(int **)(param_1 + 0x9c) + 0x3c))(0x22);
        unaff_SI = -0x2236;
        iVar5 = FUN_00618b70(0x28);
        pvStack_c._0_1_ = 0xc;
        sVar3 = -0x2236;
        if (iVar5 != 0) {
          iVar9 = FUN_005fbd20(uVar4,0x7f3,2);
          pvStack_c = (void *)((uint)pvStack_c._1_3_ << 8);
          goto LAB_005e1dcc;
        }
      }
    }
    else {
      unaff_EBX = (**(code **)(**(int **)(param_1 + 0x9c) + 0x3c))(0x21);
      unaff_SI = -0x2237;
      iVar5 = FUN_00618b70(0x28);
      pvStack_c._0_1_ = 0xb;
      sVar3 = -0x2237;
      if (iVar5 != 0) {
        iVar9 = FUN_005fbd20(uVar4,0x7f2,2);
        pvStack_c = (void *)((uint)pvStack_c._1_3_ << 8);
        goto LAB_005e1dcc;
      }
    }
  }
  unaff_SI = sVar3;
  pvStack_c = (void *)((uint)pvStack_c._1_3_ << 8);
  iVar9 = 0;
LAB_005e1dcc:
  if (iVar9 != 0) {
    FUN_005fcc30(uStack_10c,0x17,0x3b,0,0,0x66,0x6b);
    FUN_005fbfa0();
    FUN_00618b60(iVar9);
  }
  FUN_00615db0(1);
  pvStack_c._0_1_ = 0xf;
  if ((*(int *)(param_1 + 0xb0) != 0) && (unaff_SI != 0)) {
    FUN_00601b30(0x17,0x92);
    FUN_005f2fc0(&stack0xfffffeec);
    pvStack_c = (void *)CONCAT31(pvStack_c._1_3_,0x10);
    uVar4 = FUN_005c6570();
    puVar11 = &DAT_006aa5cc;
    uVar6 = FUN_005c6530(&DAT_006aa5cc,uVar4);
    puVar10 = &DAT_006a7f40;
    puVar12 = &DAT_006a7f40;
    uVar8 = FUN_00583c40(&DAT_006a7f40,unaff_EBX,&DAT_006a7f40,uVar6);
    FUN_00616110(uVar8);
    FUN_00616110(puVar12);
    FUN_00617480(unaff_EBX);
    FUN_00616110(puVar10);
    FUN_00617480(uVar6);
    FUN_00616110(puVar11);
    FUN_00617480(uVar4);
    FUN_00615f00(0);
    FUN_00601e60(uStack_f8,uStack_fc,aiStack_78);
    FUN_00616900(0);
    iVar5 = *(int *)(aiStack_78[0] + 4);
    if (*(int *)((int)&iStack_44 + iVar5) < 0) {
      FUN_00615ee0(auStack_40 + iVar5);
    }
    *(undefined4 *)((int)auStack_70 + iVar5) = 0;
    if (*(int *)((int)&iStack_44 + iVar5) < 0) {
      FUN_00615ef0(auStack_40 + iVar5);
    }
    pvStack_c._0_1_ = 0xf;
    FUN_005f2ff0();
  }
  InvalidateRect(*(HWND *)(param_1 + 0x18),(RECT *)0x0,0);
  pvStack_c = (void *)((uint)pvStack_c._1_3_ << 8);
  FUN_00615ea0();
  FUN_00615440();
  iStack_4 = 0xffffffff;
  FUN_005f2ff0();
  ExceptionList = pvStack_c;
  return;
}

