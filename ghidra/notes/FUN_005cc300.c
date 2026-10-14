
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __thiscall FUN_005cc300(void *param_1,undefined4 param_2)

{
  float fVar1;
  bool bVar2;
  void *pvVar3;
  undefined4 *puVar4;
  int iVar5;
  int *piVar6;
  undefined3 extraout_var;
  int iVar7;
  uint uVar8;
  undefined4 uVar9;
  int iVar10;
  uint uVar11;
  float unaff_EBX;
  float *pfVar12;
  float10 fVar13;
  int *piVar14;
  float fStack_6c;
  float fStack_68;
  float fStack_64;
  float fStack_60;
  float fStack_5c;
  float fStack_58;
  undefined4 uStack_54;
  float fStack_50;
  float fStack_4c;
  float fStack_48;
  float fStack_44;
  undefined4 uStack_40;
  undefined **ppuStack_3c;
  float fStack_38;
  float fStack_24;
  float fStack_20;
  void *pvStack_18;
  int iStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00654373;
  pvStack_c = ExceptionList;
  if (*(int *)((int)param_1 + 100) == 3) {
    iVar5 = *(int *)((int)param_1 + 0x50);
    puVar4 = &uStack_40;
    ExceptionList = &pvStack_c;
    pvVar3 = (void *)FUN_005aaf90();
    puVar4 = (undefined4 *)FUN_005a7490(pvVar3,puVar4,iVar5);
    iVar5 = *(int *)((int)param_1 + 0x54);
    *(undefined4 *)(iVar5 + 0x1c) = *puVar4;
    *(undefined4 *)(iVar5 + 0x20) = puVar4[1];
    *(undefined4 *)(iVar5 + 0x24) = puVar4[2];
    *(undefined4 *)(iVar5 + 0x28) = puVar4[3];
    iVar5 = *(int *)((int)param_1 + 0x54);
    *(float *)(iVar5 + 0x1c) = *(float *)(iVar5 + 0x1c) * _DAT_0066cff8;
    *(float *)(iVar5 + 0x20) = *(float *)(iVar5 + 0x20) * _DAT_0066cff8;
    *(float *)(iVar5 + 0x24) = *(float *)(iVar5 + 0x24) * _DAT_0066cff8;
    iVar5 = *(int *)((int)param_1 + 0x54);
    *(undefined4 *)(iVar5 + 0x2c) = *(undefined4 *)(iVar5 + 0x1c);
    *(undefined4 *)(iVar5 + 0x30) = *(undefined4 *)(iVar5 + 0x20);
    *(undefined4 *)(iVar5 + 0x34) = *(undefined4 *)(iVar5 + 0x24);
    *(undefined4 *)(iVar5 + 0x38) = *(undefined4 *)(iVar5 + 0x28);
    FUN_005a33b0(&fStack_60,(undefined4 *)(*(int *)((int)param_1 + 0x54) + 0x3c));
    iVar5 = FUN_005aaf90();
    uStack_54 = 0xbf800000;
    fStack_64 = *(float *)(iVar5 + 0x8cc) * *(float *)(*(int *)((int)param_1 + 0x54) + 0x24) *
                _DAT_0066cffc;
    fStack_58 = fStack_64;
    iVar5 = (**(code **)(*(int *)(*(int *)((int)param_1 + 0x54) + 0xc) + 0xc))
                      (&fStack_60,param_2,0x3f800000);
    if (iVar5 != 0) {
      ExceptionList = pvStack_18;
      return;
    }
    iVar5 = *(int *)((int)param_1 + 0x50);
    pvVar3 = (void *)FUN_005aaf90();
    fVar13 = FUN_005a8ad0(pvVar3,iVar5);
    if ((float10)_DAT_0066d000 < fVar13) {
      ExceptionList = pvStack_18;
      return;
    }
    piVar6 = (int *)FUN_005c6070(*(int **)((int)param_1 + 0x54));
    if (piVar6 == (int *)0x0) {
      ExceptionList = pvStack_18;
      return;
    }
    piVar14 = piVar6;
    FUN_005aaf90();
    iVar5 = FUN_005ad870(piVar14);
    if (iVar5 == 0) {
      ExceptionList = pvStack_18;
      return;
    }
    iVar5 = FUN_005b1b70(piVar6);
    if (iVar5 != 1) {
      ExceptionList = pvStack_18;
      return;
    }
    FUN_005a33b0(&fStack_4c,piVar6 + 0xc);
    iVar5 = *(int *)((int)param_1 + 0x50);
    pfVar12 = &fStack_5c;
    pvVar3 = (void *)FUN_005aaf90();
    iVar5 = FUN_005a7490(pvVar3,pfVar12,iVar5);
    if (_DAT_0066cff0 < -((unaff_EBX - fStack_44) * *(float *)(iVar5 + 8))) {
      ExceptionList = pvStack_18;
      return;
    }
    iVar5 = piVar6[0x5c];
    if (iVar5 == 8) {
      ExceptionList = pvStack_18;
      return;
    }
    if (iVar5 == 7) {
      ExceptionList = pvStack_18;
      return;
    }
    if (iVar5 == 6) {
      ExceptionList = pvStack_18;
      return;
    }
    FUN_005a33b0(&fStack_4c,(undefined4 *)(*(int *)((int)param_1 + 0x54) + 0x1c));
    FUN_005a33b0(piVar6 + 8,&fStack_4c);
    FUN_005a33b0(&fStack_4c,(undefined4 *)(*(int *)((int)param_1 + 0x54) + 0x1c));
    FUN_005a33b0(piVar6 + 4,&fStack_4c);
    bVar2 = FUN_005b23c0((int)piVar6);
    if ((CONCAT31(extraout_var,bVar2) != 0) && (piVar6[0x54] == 0)) {
      FUN_005c1360(piVar6[0x3b],0);
    }
    if (piVar6[0x49] != 0) {
      ExceptionList = pvStack_18;
      return;
    }
    piVar6[0x5c] = 6;
    iVar5 = piVar6[0x3b];
    piVar6 = (int *)FUN_005aaf90();
    (**(code **)(*piVar6 + 0x28))(0x2d,iVar5,0,1);
    ExceptionList = pvStack_18;
    return;
  }
  ExceptionList = &pvStack_c;
  iVar5 = (**(code **)(*(int *)((int)param_1 + 0x74) + 0xc))();
  if (iVar5 != 0) {
    piVar6 = (int *)(**(code **)(*(int *)((int)param_1 + 0x74) + 0xc))();
    iVar7 = (**(code **)(*piVar6 + 8))();
    FUN_005a33b0(&fStack_50,(undefined4 *)(*(int *)((int)param_1 + 0x54) + 0x3c));
    FUN_005a33b0(&fStack_60,(undefined4 *)(iVar7 + 0x44));
    ppuStack_3c = (undefined **)(fStack_5c - fStack_4c);
    iVar5 = *(int *)((int)param_1 + 0x54);
    fStack_38 = fStack_58 - fStack_48;
    *(undefined4 *)(iVar5 + 0x38) = 0xbf800000;
    *(undefined ***)(iVar5 + 0x30) = ppuStack_3c;
    *(float *)(iVar5 + 0x2c) = fStack_60 - fStack_50;
    *(float *)(iVar5 + 0x34) = fStack_38;
    iVar5 = *(int *)((int)param_1 + 0x54);
    pfVar12 = (float *)(iVar5 + 0x2c);
    if (*(int *)(iVar5 + 0x38) != 0x3f800000) {
      fVar13 = FUN_005b50b0(pfVar12);
      if ((float10)_DAT_0066d000 < fVar13) {
        fVar13 = (float10)_DAT_0066d004 / fVar13;
        *(undefined4 *)(iVar5 + 0x38) = 0xbf800000;
        *pfVar12 = (float)(fVar13 * (float10)*pfVar12);
        *(float *)(iVar5 + 0x30) = (float)(fVar13 * (float10)*(float *)(iVar5 + 0x30));
        *(float *)(iVar5 + 0x34) = (float)(fVar13 * (float10)*(float *)(iVar5 + 0x34));
      }
      *(undefined4 *)(iVar5 + 0x38) = 0x3f800000;
    }
    FUN_005a33b0(&uStack_40,(undefined4 *)(iVar7 + 0x44));
    iVar5 = (**(code **)(*(int *)(*(int *)((int)param_1 + 0x54) + 0xc) + 0xc))
                      (&uStack_40,param_2,0x3f800000);
    if (iVar5 != 0) {
      ExceptionList = pvStack_18;
      return;
    }
    if (*(int *)((int)param_1 + 0x34) == 0) {
      FUN_005aaf90();
      FUN_00403ef0();
    }
    iVar5 = (**(code **)(*piVar6 + 0xc))();
    iVar7 = FUN_005c61d0((void *)((int)param_1 + 0x80),piVar6[6]);
    if (iVar7 != 0) {
      FUN_005c6200((void *)((int)param_1 + 0x80),piVar6[6]);
    }
    FUN_005c6200((void *)((int)param_1 + 0x74),piVar6[6]);
    if (iVar5 != 0) {
      ExceptionList = pvStack_18;
      return;
    }
    iVar5 = *(int *)((int)param_1 + 0x50);
    iVar7 = FUN_005aaf90();
    if (iVar5 != *(int *)(iVar7 + 0x9ec)) {
      ExceptionList = pvStack_18;
      return;
    }
    iVar5 = FUN_005aaf90();
    iVar5 = *(int *)(iVar5 + 0x9ec);
    iVar7 = FUN_005aaf90();
    if (*(int *)(*(int *)(iVar7 + 8 + iVar5 * 4) + 8) == 1) {
      ExceptionList = pvStack_18;
      return;
    }
    iVar5 = *(int *)((int)param_1 + 0x18);
    pvVar3 = (void *)FUN_0059ed70();
    uVar8 = FUN_005a0300(pvVar3,iVar5);
    if ((*(int *)((int)param_1 + 0x60) == 0) &&
       (iVar5 = (**(code **)(*(int *)((int)param_1 + 0x98) + 0x10))(), iVar5 == 0)) {
      ExceptionList = pvStack_18;
      return;
    }
    iVar5 = (**(code **)(*(int *)((int)param_1 + 0x98) + 0x10))();
    if (iVar5 == 0) {
      ExceptionList = pvStack_18;
      return;
    }
    if (uVar8 != 0xffffffff) {
      iVar5 = 1;
      if (*(int *)((int)param_1 + 0x50) == 0) {
        uVar8 = uVar8 + 0x2f;
        pvVar3 = (void *)FUN_005ba8b0();
        FUN_005ba980(pvVar3,uVar8,iVar5);
        ExceptionList = pvStack_18;
        return;
      }
      uVar8 = uVar8 + 0xa9;
      pvVar3 = (void *)FUN_005ba8b0();
      FUN_005ba980(pvVar3,uVar8,iVar5);
      ExceptionList = pvStack_18;
      return;
    }
    iVar5 = 1;
    if (*(int *)((int)param_1 + 0x50) == 0) {
      uVar8 = 0x2e;
      pvVar3 = (void *)FUN_005ba8b0();
      FUN_005ba980(pvVar3,uVar8,iVar5);
      ExceptionList = pvStack_18;
      return;
    }
    uVar8 = 0xa8;
    pvVar3 = (void *)FUN_005ba8b0();
    FUN_005ba980(pvVar3,uVar8,iVar5);
    ExceptionList = pvStack_18;
    return;
  }
  iVar5 = (**(code **)(*(int *)((int)param_1 + 0x74) + 0xc))();
  if (iVar5 != 0) {
    ExceptionList = pvStack_18;
    return;
  }
  iVar5 = (**(code **)(*(int *)((int)param_1 + 0x8c) + 0x10))();
  if (iVar5 == 0) {
    ExceptionList = pvStack_18;
    return;
  }
  piVar6 = (int *)(**(code **)(*(int *)((int)param_1 + 0x8c) + 0x10))();
  iVar5 = (**(code **)(*piVar6 + 0x10))();
  uVar9 = (**(code **)(*piVar6 + 0x10))();
  FUN_005aaf90();
  iVar7 = FUN_005ad870(uVar9);
  if ((iVar7 == 0) && (*(int *)((int)param_1 + 0xb0) == 0)) {
    iVar7 = *(int *)((int)param_1 + 0x50);
    *(undefined4 *)((int)param_1 + 0xb0) = 1;
    iVar10 = FUN_005aaf90();
    if (iVar7 == *(int *)(iVar10 + 0x9ec)) {
      uVar8 = *(uint *)((int)param_1 + 0x18);
      uVar11 = (**(code **)(*piVar6 + 4))();
      pvVar3 = (void *)FUN_005aaf90();
      FUN_005aaf50(pvVar3,uVar8,uVar11);
    }
    iVar7 = (**(code **)(*(int *)((int)param_1 + 0x98) + 0x10))();
    if ((iVar7 == 0) &&
       (iVar7 = *(int *)((int)param_1 + 0x50), iVar10 = FUN_005aaf90(),
       iVar7 == *(int *)(iVar10 + 0x9ec))) {
      iVar7 = FUN_005aaf90();
      iVar7 = *(int *)(iVar7 + 0x9ec);
      iVar10 = FUN_005aaf90();
      if ((*(int *)(*(int *)(iVar10 + 8 + iVar7 * 4) + 8) != 1) &&
         (*(int *)((int)param_1 + 0x60) == 0)) {
        bVar2 = false;
        iVar7 = FUN_005aaf90();
        iVar7 = *(int *)(iVar7 + 0x9ec);
        iVar10 = FUN_005aaf90();
        piVar6 = (int *)(**(code **)(*(int *)(*(int *)(iVar10 + 8 + (uint)(iVar7 == 0) * 4) + 0x58)
                                    + 0x10))();
        if (piVar6 == (int *)0x0) {
LAB_005cc8b9:
          iVar7 = FUN_005aaf90();
          iVar7 = *(int *)(iVar7 + 0x9ec);
          iVar10 = FUN_005aaf90();
          piVar6 = (int *)(**(code **)(*(int *)(*(int *)(iVar10 + 8 + (uint)(iVar7 == 0) * 4) + 100)
                                      + 0x10))();
          if (piVar6 != (int *)0x0) {
            uVar9 = (**(code **)(*piVar6 + 0x10))();
            FUN_005aaf90();
            iVar7 = FUN_005ad870(uVar9);
            if (iVar7 != 0) goto LAB_005cc8fb;
          }
        }
        else {
          uVar9 = (**(code **)(*piVar6 + 0x10))();
          FUN_005aaf90();
          iVar7 = FUN_005ad870(uVar9);
          if (iVar7 == 0) goto LAB_005cc8b9;
LAB_005cc8fb:
          bVar2 = true;
        }
        if (bVar2) {
          iVar7 = *(int *)((int)param_1 + 0x18);
          pvVar3 = (void *)FUN_0059ed70();
          uVar8 = FUN_005a0300(pvVar3,iVar7);
          iVar7 = FUN_005aaf90();
          iVar7 = *(int *)(iVar7 + 0x9ec);
          iVar10 = FUN_005aaf90();
          if (*(int *)(*(int *)(iVar10 + 8 + (uint)(iVar7 == 0) * 4) + 0x84) == 0) {
            if (uVar8 == 0xffffffff) {
              if (*(int *)((int)param_1 + 0x50) == 0) {
                uVar8 = 0x48;
              }
              else {
                uVar8 = 0xc2;
              }
            }
            else if (*(int *)((int)param_1 + 0x50) == 0) {
              uVar8 = uVar8 + 0x49;
            }
            else {
              uVar8 = uVar8 + 0xc3;
            }
            iVar7 = 1;
            pvVar3 = (void *)FUN_005ba8b0();
            FUN_005ba980(pvVar3,uVar8,iVar7);
          }
        }
      }
    }
  }
  fVar1 = _DAT_0066cff8;
  if (*(int *)((int)param_1 + 100) == 0xb) {
    ExceptionList = pvStack_18;
    return;
  }
  iVar7 = *(int *)((int)param_1 + 0x54);
  fStack_6c = *(float *)(iVar5 + 0x30) - *(float *)(iVar7 + 0x3c);
  fStack_64 = *(float *)(iVar5 + 0x38) - *(float *)(iVar7 + 0x44);
  fStack_68 = *(float *)(iVar5 + 0x34) - *(float *)(iVar7 + 0x40);
  fStack_60 = _DAT_0066cff8;
  if (((fStack_6c == DAT_006bc940) && (fStack_68 == DAT_006bc944)) && (fStack_64 == DAT_006bc948)) {
    iVar7 = *(int *)((int)param_1 + 0x50);
    pfVar12 = &fStack_4c;
    pvVar3 = (void *)FUN_005aaf90();
    puVar4 = (undefined4 *)FUN_005a7490(pvVar3,pfVar12,iVar7);
    iVar7 = *(int *)((int)param_1 + 0x54);
    *(undefined4 *)(iVar7 + 0x2c) = *puVar4;
    *(undefined4 *)(iVar7 + 0x30) = puVar4[1];
    *(undefined4 *)(iVar7 + 0x34) = puVar4[2];
    *(undefined4 *)(iVar7 + 0x38) = puVar4[3];
    iVar7 = *(int *)((int)param_1 + 0x54);
    fStack_64 = -_DAT_0066cfec;
    fStack_6c = fStack_64 * *(float *)(iVar7 + 0x2c);
    fStack_68 = *(float *)(iVar7 + 0x30) * fStack_64;
    fStack_64 = *(float *)(iVar7 + 0x34) * fStack_64;
    fStack_60 = _DAT_0066cff8;
    fStack_48 = fStack_68;
    fStack_44 = fStack_64;
  }
  else {
    *(float *)(iVar7 + 0x2c) = fStack_6c;
    *(float *)(iVar7 + 0x30) = fStack_68;
    *(float *)(iVar7 + 0x34) = fStack_64;
    *(float *)(iVar7 + 0x38) = fVar1;
    iVar7 = *(int *)((int)param_1 + 0x54);
    if (*(int *)(iVar7 + 0x38) != 0x3f800000) {
      if (*(int *)(iVar7 + 0x38) == -0x40800000) {
        fVar1 = *(float *)(iVar7 + 0x2c);
        *(float *)(iVar7 + 0x38) =
             SQRT(fVar1 * fVar1 +
                  *(float *)(iVar7 + 0x34) * *(float *)(iVar7 + 0x34) +
                  *(float *)(iVar7 + 0x30) * *(float *)(iVar7 + 0x30));
      }
      if (_DAT_0066d000 < *(float *)(iVar7 + 0x38)) {
        FUN_005ba080((float *)(iVar7 + 0x2c),_DAT_0066d004 / *(float *)(iVar7 + 0x38));
      }
      *(undefined4 *)(iVar7 + 0x38) = 0x3f800000;
    }
  }
  FUN_005f5c10(&ppuStack_3c);
  ppuStack_3c = &PTR_FUN_0066c678;
  iStack_10 = 0;
  (**(code **)(**(int **)((int)param_1 + 0x40) + 0x10))(&ppuStack_3c);
  if (fStack_60 == _DAT_0066cff8) {
    fStack_60 = SQRT(fStack_6c * fStack_6c + fStack_64 * fStack_64 + fStack_68 * fStack_68);
  }
  if ((fStack_60 - _DAT_0066d008 < fStack_24) || (fStack_20 < fStack_60 - _DAT_0066d008)) {
    if (*(int *)((int)param_1 + 0x3c) == 2) {
      iVar7 = *(int *)((int)param_1 + 0x54);
      if (*(int *)(iVar7 + 0x38) != 0x3f800000) {
        if (*(int *)(iVar7 + 0x38) == -0x40800000) {
          fVar1 = *(float *)(iVar7 + 0x2c);
          *(float *)(iVar7 + 0x38) =
               SQRT(fVar1 * fVar1 +
                    *(float *)(iVar7 + 0x34) * *(float *)(iVar7 + 0x34) +
                    *(float *)(iVar7 + 0x30) * *(float *)(iVar7 + 0x30));
        }
        if (_DAT_0066d000 < *(float *)(iVar7 + 0x38)) {
          FUN_005ba080((float *)(iVar7 + 0x2c),_DAT_0066d004 / *(float *)(iVar7 + 0x38));
        }
        *(undefined4 *)(iVar7 + 0x38) = 0x3f800000;
      }
      fVar1 = fStack_24;
      if (fStack_60 == _DAT_0066cff8) {
        fStack_60 = SQRT(fStack_6c * fStack_6c + fStack_64 * fStack_64 + fStack_68 * fStack_68);
      }
      if (fStack_60 < fStack_24) {
        iVar7 = 0;
        uVar8 = *(uint *)(iVar5 + 0x174);
        if (uVar8 != 0) {
          if (DAT_006bc470 == (undefined4 *)0x0) {
            puVar4 = (undefined4 *)FUN_00618b70(0x48);
            iStack_10._0_1_ = 1;
            if (puVar4 == (undefined4 *)0x0) {
              DAT_006bc470 = (undefined4 *)0x0;
            }
            else {
              DAT_006bc470 = FUN_005a6290(puVar4);
            }
            iStack_10 = (uint)iStack_10._1_3_ << 8;
          }
          iVar7 = FUN_005a6480(DAT_006bc470,uVar8);
        }
        if ((iVar7 != 0) &&
           (piVar6 = (int *)(**(code **)(*(int *)(iVar7 + 0x8c) + 0x10))(), piVar6 != (int *)0x0)) {
          uVar8 = (**(code **)(*piVar6 + 4))();
          iVar5 = FUN_005ca4b0((void *)((int)param_1 + 0x1c),uVar8);
          if (iVar5 != 0) {
            iVar5 = *(int *)((int)param_1 + 0x54);
            *(float *)(iVar5 + 0x78) = DAT_006bc940;
            *(float *)(iVar5 + 0x7c) = DAT_006bc944;
            *(float *)(iVar5 + 0x80) = DAT_006bc948;
            *(undefined4 *)(iVar5 + 0x84) = DAT_006bc94c;
            goto LAB_005ccfb1;
          }
        }
        fVar1 = fVar1 * _DAT_0066d00c;
        iVar5 = *(int *)((int)param_1 + 0x50);
        pfVar12 = &fStack_4c;
        pvVar3 = (void *)FUN_005aaf90();
        pfVar12 = (float *)FUN_005a7490(pvVar3,pfVar12,iVar5);
        fStack_6c = *pfVar12 * fVar1;
        fStack_68 = pfVar12[1] * fVar1;
        fStack_64 = pfVar12[2] * fVar1;
        fStack_60 = _DAT_0066cff8;
        fStack_48 = fStack_68;
        fStack_44 = fStack_64;
      }
      if ((((*(int *)((int)param_1 + 0x38) == 2) && (*(int *)((int)param_1 + 0x34) == 0)) &&
          (iVar5 = *(int *)((int)param_1 + 0x50), iVar7 = FUN_005aaf90(),
          *(int *)(*(int *)(iVar7 + 8 + iVar5 * 4) + 8) == 1)) &&
         (iVar5 = FUN_005c5eb0((int *)((int)param_1 + 0x1c)), 7 < iVar5)) {
        FUN_005c9be0(param_1,1);
      }
    }
    if (((fStack_6c != DAT_006bc940) || (fStack_68 != DAT_006bc944)) || (fStack_64 != DAT_006bc948))
    {
      if (fStack_60 != _DAT_0066d004) {
        if (fStack_60 == _DAT_0066cff8) {
          fStack_60 = SQRT(fStack_6c * fStack_6c + fStack_64 * fStack_64 + fStack_68 * fStack_68);
        }
        if (_DAT_0066d000 < fStack_60) {
          fStack_60 = _DAT_0066d004 / fStack_60;
          fStack_6c = fStack_60 * fStack_6c;
          fStack_68 = fStack_60 * fStack_68;
          fStack_64 = fStack_60 * fStack_64;
        }
        fStack_60 = 1.0;
      }
      if (*(int *)((int)param_1 + 0x3c) == 1) {
        fVar13 = (float10)(**(code **)(**(int **)((int)param_1 + 0x40) + 0xc))();
      }
      else {
        fVar1 = fStack_20 - fStack_24;
        iVar5 = FUN_005c5eb0((int *)((int)param_1 + 0x1c));
        if (iVar5 < 3) {
          fVar13 = (float10)0.5;
        }
        else {
          iVar5 = FUN_005c5eb0((int *)((int)param_1 + 0x1c));
          if (iVar5 < 6) {
            fVar13 = (float10)_DAT_0066d014;
          }
          else {
            fVar13 = (float10)_DAT_0066d010;
          }
        }
        fVar13 = fVar13 * (float10)fVar1 + (float10)fStack_24;
        if (fVar13 < (float10)_DAT_0066d018) {
          fVar13 = (float10)_DAT_0066d018;
        }
      }
      fStack_6c = (float)(fVar13 * (float10)fStack_6c);
      fStack_68 = (float)(fVar13 * (float10)fStack_68);
      fStack_64 = (float)(fVar13 * (float10)fStack_64);
    }
    iVar5 = *(int *)((int)param_1 + 0x54);
    fStack_6c = *(float *)(iVar5 + 0x3c) + fStack_6c;
    fStack_44 = *(float *)(iVar5 + 0x44);
    fStack_60 = -1.0;
    fStack_68 = *(float *)(iVar5 + 0x40) + fStack_68;
    fStack_64 = fStack_44 + fStack_64;
    (**(code **)(*(int *)(iVar5 + 0xc) + 0xc))(&fStack_6c,puStack_8,0x40000000);
  }
  else {
    iVar5 = *(int *)((int)param_1 + 0x54);
    *(float *)(iVar5 + 0x78) = DAT_006bc940;
    *(float *)(iVar5 + 0x7c) = DAT_006bc944;
    *(float *)(iVar5 + 0x80) = DAT_006bc948;
    *(undefined4 *)(iVar5 + 0x84) = DAT_006bc94c;
  }
LAB_005ccfb1:
  iStack_10 = 0xffffffff;
  ppuStack_3c = &PTR_FUN_0066c678;
  FUN_005f5c50(&ppuStack_3c);
  ExceptionList = pvStack_18;
  return;
}

