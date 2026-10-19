
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __thiscall FUN_005c6610(int param_1,undefined4 param_2)

{
  undefined4 uVar1;
  undefined4 uVar2;
  float fVar3;
  float fVar4;
  bool bVar5;
  float fVar6;
  void *pvVar7;
  undefined4 *puVar8;
  int iVar9;
  int *piVar10;
  int *piVar11;
  int *piVar12;
  float *pfVar13;
  int iVar14;
  uint uVar15;
  undefined4 uVar16;
  int iVar17;
  uint uVar18;
  float unaff_ESI;
  float10 fVar19;
  float10 fVar20;
  float fStack_54;
  float fStack_50;
  float fStack_4c;
  float fStack_48;
  float fStack_44;
  float fStack_40;
  float fStack_3c;
  float fStack_38;
  float fStack_34;
  undefined4 auStack_2c [3];
  undefined4 uStack_20;
  undefined4 uStack_1c;
  float fStack_18;
  undefined4 auStack_10 [2];
  float fStack_8;
  
  if (*(int *)(param_1 + 100) == 3) {
    iVar9 = *(int *)(param_1 + 0x50);
    puVar8 = &uStack_20;
    pvVar7 = (void *)FUN_005aaf90();
    puVar8 = (undefined4 *)FUN_005a7490(pvVar7,puVar8,iVar9);
    iVar9 = *(int *)(param_1 + 0x54);
    *(undefined4 *)(iVar9 + 0x1c) = *puVar8;
    *(undefined4 *)(iVar9 + 0x20) = puVar8[1];
    *(undefined4 *)(iVar9 + 0x24) = puVar8[2];
    *(undefined4 *)(iVar9 + 0x28) = puVar8[3];
    iVar9 = *(int *)(param_1 + 0x54);
    *(float *)(iVar9 + 0x1c) = *(float *)(iVar9 + 0x1c) * _DAT_0066cb10;
    *(float *)(iVar9 + 0x20) = *(float *)(iVar9 + 0x20) * _DAT_0066cb10;
    *(float *)(iVar9 + 0x24) = *(float *)(iVar9 + 0x24) * _DAT_0066cb10;
    iVar9 = *(int *)(param_1 + 0x54);
    *(undefined4 *)(iVar9 + 0x2c) = *(undefined4 *)(iVar9 + 0x1c);
    *(undefined4 *)(iVar9 + 0x30) = *(undefined4 *)(iVar9 + 0x20);
    *(undefined4 *)(iVar9 + 0x34) = *(undefined4 *)(iVar9 + 0x24);
    *(undefined4 *)(iVar9 + 0x38) = *(undefined4 *)(iVar9 + 0x28);
    FUN_005a33b0(&fStack_40,(undefined4 *)(*(int *)(param_1 + 0x54) + 0x3c));
    iVar9 = FUN_005aaf90();
    fStack_34 = -1.0;
    fStack_38 = *(float *)(iVar9 + 0x8cc) * *(float *)(*(int *)(param_1 + 0x54) + 0x24) *
                _DAT_0066cb14;
    iVar9 = (**(code **)(*(int *)(*(int *)(param_1 + 0x54) + 0xc) + 0xc))
                      (&fStack_40,param_2,0x3f800000);
    if (iVar9 != 0) {
      return;
    }
    piVar10 = (int *)(**(code **)(**(int **)(param_1 + 0x54) + 0x10))();
    if (piVar10 == (int *)0x0) {
      return;
    }
    do {
      piVar11 = (int *)(**(code **)(*piVar10 + 0x10))();
      piVar12 = piVar11;
      FUN_005aaf90();
      iVar9 = FUN_005ad870(piVar12);
      if ((iVar9 != 0) && (iVar9 = FUN_005b1b70(piVar11), iVar9 == 1)) {
        FUN_005a33b0(&stack0xffffffa4,piVar11 + 0xc);
        iVar9 = *(int *)(param_1 + 0x50);
        puVar8 = &uStack_1c;
        pvVar7 = (void *)FUN_005aaf90();
        iVar9 = FUN_005a7490(pvVar7,puVar8,iVar9);
        unaff_ESI = -((unaff_ESI - fStack_54) * *(float *)(iVar9 + 8));
        if (unaff_ESI <= _DAT_0066cb0c) {
          iVar9 = *(int *)(param_1 + 0x50);
          pvVar7 = (void *)FUN_005aaf90();
          fVar19 = FUN_005a8ad0(pvVar7,iVar9);
          if ((((fVar19 <= (float10)_DAT_0066cb18) && (iVar9 = piVar11[0x5c], iVar9 != 8)) &&
              (iVar9 != 7)) && (iVar9 != 6)) {
            FUN_005a33b0(&fStack_3c,(undefined4 *)(*(int *)(param_1 + 0x54) + 0x1c));
            FUN_005a33b0(piVar11 + 8,&fStack_3c);
            FUN_005a33b0(auStack_2c,(undefined4 *)(*(int *)(param_1 + 0x54) + 0x1c));
            FUN_005a33b0(piVar11 + 4,auStack_2c);
            if (piVar11[0x49] == 0) {
              piVar11[0x5c] = 6;
              iVar9 = piVar11[0x3b];
              piVar12 = (int *)FUN_005aaf90();
              (**(code **)(*piVar12 + 0x28))(0x2d,iVar9,0,1);
            }
          }
        }
      }
      piVar10 = (int *)(**(code **)(*piVar10 + 8))();
    } while (piVar10 != (int *)0x0);
    return;
  }
  if (*(int *)(param_1 + 100) == 6) {
    iVar9 = *(int *)(param_1 + 0x50);
    piVar10 = *(int **)(param_1 + 0x60);
    puVar8 = auStack_10;
    pvVar7 = (void *)FUN_005aaf90();
    pfVar13 = (float *)FUN_005a7490(pvVar7,puVar8,iVar9);
    fStack_38 = *pfVar13 - 0.0;
    fStack_40 = 0.0 - pfVar13[2];
    fStack_34 = -1.0;
    fStack_3c = _DAT_0066cb18;
    FUN_005a33b0((void *)(*(int *)(param_1 + 0x54) + 0x1c),&fStack_40);
    if (piVar10[0xb] != 2) {
      return;
    }
    FUN_005a33b0(&fStack_50,(undefined4 *)(*(int *)(param_1 + 0x54) + 0x1c));
    iVar9 = FUN_005aaf90();
    fStack_38 = *(float *)(iVar9 + 0x8cc) * _DAT_0066cb14;
    fStack_34 = _DAT_0066cb10;
    fStack_40 = fStack_38 * fStack_50;
    fStack_3c = fStack_38 * fStack_4c;
    fStack_38 = fStack_38 * fStack_48;
    iVar9 = (**(code **)(*(int *)(*(int *)(param_1 + 0x54) + 0xc) + 0xc))
                      (&fStack_40,param_2,0x3e4ccccd);
    if (iVar9 != 0) {
      return;
    }
    iVar9 = (**(code **)(*piVar10 + 0x10))();
    if (iVar9 != 0) {
      return;
    }
    (**(code **)(*piVar10 + 0xc))(1);
    FUN_005cfe80(piVar10,5);
    return;
  }
  piVar10 = (int *)(param_1 + 0x74);
  iVar9 = (**(code **)(*(int *)(param_1 + 0x74) + 0xc))();
  if (iVar9 != 0) {
    piVar12 = (int *)(**(code **)(*piVar10 + 0xc))();
    iVar9 = (**(code **)(*piVar12 + 8))();
    FUN_005a33b0(&uStack_20,(undefined4 *)(iVar9 + 0x44));
    iVar14 = (**(code **)(*(int *)(*(int *)(param_1 + 0x54) + 0xc) + 0xc))
                       (&uStack_20,param_2,0x3f800000);
    if (iVar14 == 0) {
      FUN_005a33b0(&fStack_40,(undefined4 *)(iVar9 + 0x44));
      iVar9 = *(int *)(param_1 + 0x54);
      *(float *)(iVar9 + 0x3c) = fStack_40;
      *(float *)(iVar9 + 0x40) = fStack_3c;
      *(float *)(iVar9 + 0x44) = fStack_38;
      *(float *)(iVar9 + 0x48) = fStack_34;
      if (*(int *)(param_1 + 0x34) == 0) {
        FUN_005aaf90();
        FUN_00403ef0();
      }
      iVar9 = (**(code **)(*piVar12 + 0xc))();
      iVar14 = FUN_005c61d0((void *)(param_1 + 0x80),piVar12[6]);
      if (iVar14 != 0) {
        FUN_005c6200((void *)(param_1 + 0x80),piVar12[6]);
      }
      FUN_005c6200(piVar10,piVar12[6]);
      if ((iVar9 == 0) &&
         (iVar9 = *(int *)(param_1 + 0x50), iVar14 = FUN_005aaf90(),
         iVar9 == *(int *)(iVar14 + 0x9ec))) {
        iVar9 = FUN_005aaf90();
        iVar9 = *(int *)(iVar9 + 0x9ec);
        iVar14 = FUN_005aaf90();
        if (*(int *)(*(int *)(iVar14 + 8 + iVar9 * 4) + 8) != 1) {
          uVar15 = *(uint *)(param_1 + 0x18);
          pvVar7 = (void *)FUN_0059ed70();
          uVar15 = FUN_005a02c0(pvVar7,uVar15);
          if (((*(int *)(param_1 + 0x60) != 0) ||
              (iVar9 = (**(code **)(*(int *)(param_1 + 0x98) + 0x10))(), iVar9 != 0)) &&
             (iVar9 = (**(code **)(*(int *)(param_1 + 0x98) + 0x10))(), iVar9 != 0)) {
            iVar9 = 1;
            if (*(int *)(param_1 + 0x50) == 0) {
              uVar15 = uVar15 + 0x37;
            }
            else {
              uVar15 = uVar15 + 0xb1;
            }
            pvVar7 = (void *)FUN_005ba8b0();
            FUN_005ba980(pvVar7,uVar15,iVar9);
          }
        }
      }
    }
  }
  iVar9 = (**(code **)(*piVar10 + 0xc))();
  if (iVar9 != 0) {
    return;
  }
  iVar9 = (**(code **)(*(int *)(param_1 + 0x8c) + 0x10))();
  if (iVar9 == 0) {
    return;
  }
  if (*(int *)(param_1 + 0xb0) != 0) {
    return;
  }
  piVar10 = (int *)(**(code **)(*(int *)(param_1 + 0x8c) + 0x10))();
  iVar9 = (**(code **)(*piVar10 + 0x10))();
  uVar16 = (**(code **)(*piVar10 + 0x10))();
  FUN_005aaf90();
  iVar14 = FUN_005ad870(uVar16);
  if (iVar14 != 0) goto LAB_005c6c6f;
  iVar14 = *(int *)(param_1 + 0x50);
  *(undefined4 *)(param_1 + 0xb0) = 1;
  iVar17 = FUN_005aaf90();
  if (iVar14 == *(int *)(iVar17 + 0x9ec)) {
    uVar15 = *(uint *)(param_1 + 0x18);
    uVar18 = (**(code **)(*piVar10 + 4))();
    pvVar7 = (void *)FUN_005aaf90();
    FUN_005aaf50(pvVar7,uVar15,uVar18);
  }
  iVar14 = (**(code **)(*(int *)(param_1 + 0x98) + 0x10))();
  if ((iVar14 != 0) ||
     (iVar14 = *(int *)(param_1 + 0x50), iVar17 = FUN_005aaf90(), iVar14 != *(int *)(iVar17 + 0x9ec)
     )) goto LAB_005c6c6f;
  iVar14 = FUN_005aaf90();
  iVar14 = *(int *)(iVar14 + 0x9ec);
  iVar17 = FUN_005aaf90();
  if ((*(int *)(*(int *)(iVar17 + 8 + iVar14 * 4) + 8) == 1) || (*(int *)(param_1 + 0x60) != 0))
  goto LAB_005c6c6f;
  bVar5 = false;
  iVar14 = FUN_005aaf90();
  iVar14 = *(int *)(iVar14 + 0x9ec);
  iVar17 = FUN_005aaf90();
  piVar10 = (int *)(**(code **)(*(int *)(*(int *)(iVar17 + 8 + (uint)(iVar14 == 0) * 4) + 0x58) +
                               0x10))();
  if (piVar10 == (int *)0x0) {
LAB_005c6bc9:
    iVar14 = FUN_005aaf90();
    iVar14 = *(int *)(iVar14 + 0x9ec);
    iVar17 = FUN_005aaf90();
    piVar10 = (int *)(**(code **)(*(int *)(*(int *)(iVar17 + 8 + (uint)(iVar14 == 0) * 4) + 100) +
                                 0x10))();
    if (piVar10 != (int *)0x0) {
      uVar16 = (**(code **)(*piVar10 + 0x10))();
      FUN_005aaf90();
      iVar14 = FUN_005ad870(uVar16);
      if (iVar14 != 0) goto LAB_005c6c0b;
    }
  }
  else {
    uVar16 = (**(code **)(*piVar10 + 0x10))();
    FUN_005aaf90();
    iVar14 = FUN_005ad870(uVar16);
    if (iVar14 == 0) goto LAB_005c6bc9;
LAB_005c6c0b:
    bVar5 = true;
  }
  if (bVar5) {
    uVar15 = *(uint *)(param_1 + 0x18);
    pvVar7 = (void *)FUN_0059ed70();
    uVar15 = FUN_005a02c0(pvVar7,uVar15);
    iVar14 = FUN_005aaf90();
    iVar14 = *(int *)(iVar14 + 0x9ec);
    iVar17 = FUN_005aaf90();
    if (*(int *)(*(int *)(iVar17 + 8 + (uint)(iVar14 == 0) * 4) + 0x84) == 0) {
      iVar14 = 1;
      if (*(int *)(param_1 + 0x50) == 0) {
        uVar15 = uVar15 + 0x51;
      }
      else {
        uVar15 = uVar15 + 0xcb;
      }
      pvVar7 = (void *)FUN_005ba8b0();
      FUN_005ba980(pvVar7,uVar15,iVar14);
    }
  }
LAB_005c6c6f:
  fVar6 = _DAT_0066cb10;
  if (*(int *)(param_1 + 100) != 0xb) {
    iVar14 = *(int *)(param_1 + 0x54);
    fStack_50 = *(float *)(iVar9 + 0x30) - *(float *)(iVar14 + 0x3c);
    fStack_48 = *(float *)(iVar9 + 0x38) - *(float *)(iVar14 + 0x44);
    fStack_4c = *(float *)(iVar9 + 0x34) - *(float *)(iVar14 + 0x40);
    fStack_44 = _DAT_0066cb10;
    if (((fStack_50 == DAT_006bc940) && (fStack_4c == DAT_006bc944)) && (fStack_48 == DAT_006bc948))
    {
      iVar14 = *(int *)(param_1 + 0x50);
      puVar8 = auStack_10;
      pvVar7 = (void *)FUN_005aaf90();
      puVar8 = (undefined4 *)FUN_005a7490(pvVar7,puVar8,iVar14);
      iVar14 = *(int *)(param_1 + 0x54);
      *(undefined4 *)(iVar14 + 0x2c) = *puVar8;
      *(undefined4 *)(iVar14 + 0x30) = puVar8[1];
      *(undefined4 *)(iVar14 + 0x34) = puVar8[2];
      *(undefined4 *)(iVar14 + 0x38) = puVar8[3];
      iVar14 = *(int *)(param_1 + 0x54);
      fStack_50 = *(float *)(iVar14 + 0x2c);
      fStack_4c = *(float *)(iVar14 + 0x30);
      fStack_48 = *(float *)(iVar14 + 0x34);
      fStack_44 = *(float *)(iVar14 + 0x38);
    }
    else {
      *(float *)(iVar14 + 0x2c) = fStack_50;
      *(float *)(iVar14 + 0x30) = fStack_4c;
      *(float *)(iVar14 + 0x34) = fStack_48;
      *(float *)(iVar14 + 0x38) = fVar6;
    }
    iVar14 = *(int *)(param_1 + 0x54);
    pfVar13 = (float *)(iVar14 + 0x2c);
    if (*(int *)(iVar14 + 0x38) != 0x3f800000) {
      if (*(int *)(iVar14 + 0x38) == -0x40800000) {
        *(float *)(iVar14 + 0x38) =
             SQRT(*pfVar13 * *pfVar13 +
                  *(float *)(iVar14 + 0x34) * *(float *)(iVar14 + 0x34) +
                  *(float *)(iVar14 + 0x30) * *(float *)(iVar14 + 0x30));
      }
      if (_DAT_0066cb18 < *(float *)(iVar14 + 0x38)) {
        fVar6 = _DAT_0066cb1c / *(float *)(iVar14 + 0x38);
        *(undefined4 *)(iVar14 + 0x38) = 0xbf800000;
        *pfVar13 = *pfVar13 * fVar6;
        *(float *)(iVar14 + 0x30) = fVar6 * *(float *)(iVar14 + 0x30);
        *(float *)(iVar14 + 0x34) = fVar6 * *(float *)(iVar14 + 0x34);
      }
      *(undefined4 *)(iVar14 + 0x38) = 0x3f800000;
    }
    if (fStack_44 != _DAT_0066cb1c) {
      if (fStack_44 == _DAT_0066cb10) {
        fStack_44 = SQRT(fStack_50 * fStack_50 + fStack_48 * fStack_48 + fStack_4c * fStack_4c);
      }
      if (_DAT_0066cb18 < fStack_44) {
        fStack_44 = _DAT_0066cb1c / fStack_44;
        fStack_50 = fStack_44 * fStack_50;
        fStack_4c = fStack_44 * fStack_4c;
        fStack_48 = fStack_44 * fStack_48;
      }
      fStack_44 = 1.0;
    }
    fVar19 = (float10)(**(code **)(**(int **)(param_1 + 0x40) + 0xc))();
    iVar14 = *(int *)(param_1 + 0x54);
    fStack_18 = *(float *)(iVar14 + 0x44);
    fStack_4c = *(float *)(iVar14 + 0x40) + (float)(fVar19 * (float10)fStack_4c);
    fStack_48 = fStack_18 + (float)(fVar19 * (float10)fStack_48);
    fStack_50 = *(float *)(iVar14 + 0x3c) + (float)(fVar19 * (float10)fStack_50);
    fStack_44 = -1.0;
    iVar14 = (**(code **)(*(int *)(iVar14 + 0xc) + 0xc))(&fStack_50,param_2,0x40000000);
    if (iVar14 == 0) {
      uVar16 = *(undefined4 *)(iVar9 + 0x34);
      uVar1 = *(undefined4 *)(iVar9 + 0x38);
      uVar2 = *(undefined4 *)(iVar9 + 0x3c);
      iVar14 = *(int *)(param_1 + 0x54);
      *(undefined4 *)(iVar14 + 0x3c) = *(undefined4 *)(iVar9 + 0x30);
      *(undefined4 *)(iVar14 + 0x40) = uVar16;
      *(undefined4 *)(iVar14 + 0x44) = uVar1;
      *(undefined4 *)(iVar14 + 0x48) = uVar2;
      fVar6 = fStack_8 * _DAT_0066cafc * _DAT_0066cb20 + *(float *)(param_1 + 0xc0);
      *(float *)(param_1 + 0xc0) = fVar6;
      if (_DAT_0066cafc + _DAT_0066cafc <= fVar6) {
        *(float *)(param_1 + 0xc0) = fVar6 - (_DAT_0066cafc + _DAT_0066cafc);
      }
      fStack_8 = *(float *)(iVar9 + 0xc) * _DAT_0066cb28;
      if (fStack_8 < DAT_0066cb08) {
        fStack_8 = DAT_0066cb08;
      }
      fVar6 = fStack_8 * _DAT_0066cb14;
      if (*(int *)(param_1 + 0x50) == 1) {
        *(float *)(param_1 + 0xc0) = *(float *)(param_1 + 0xc0) * _DAT_0066cb10;
      }
      fVar19 = (float10)fcos((float10)*(float *)(param_1 + 0xc0));
      iVar14 = *(int *)(param_1 + 0x54);
      fVar20 = (float10)fsin((float10)*(float *)(param_1 + 0xc0));
      fVar3 = *(float *)(iVar9 + 0x38);
      fVar4 = *(float *)(iVar9 + 0x30);
      *(float *)(iVar14 + 0x40) = *(float *)(iVar9 + 0x34) + fVar6;
      *(float *)(iVar14 + 0x3c) =
           fVar4 + (float)(fVar19 * (float10)fStack_8 - (float10)(float)(fVar20 * (float10)fStack_8)
                          );
      *(float *)(iVar14 + 0x44) =
           fVar3 + (float)(fVar19 * (float10)fStack_8) + (float)(fVar20 * (float10)fStack_8);
      *(undefined4 *)(iVar14 + 0x48) = 0xbf800000;
    }
  }
  return;
}

