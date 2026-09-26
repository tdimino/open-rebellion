
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __fastcall FUN_005b3a40(int *param_1)

{
  float fVar1;
  float fVar2;
  float fVar3;
  float fVar4;
  float fVar5;
  float fVar6;
  float fVar7;
  float fVar8;
  bool bVar9;
  float fVar10;
  float fVar11;
  float fVar12;
  int iVar13;
  uint uVar14;
  void *this;
  undefined4 uVar15;
  int *piVar16;
  int iVar17;
  int *piVar18;
  float10 fVar19;
  float10 fVar20;
  float10 fVar21;
  float10 fVar22;
  float10 fVar23;
  int iVar24;
  float fStack_14;
  
  piVar18 = param_1;
  FUN_005aaf90();
  iVar13 = FUN_005ad870(piVar18);
  if ((iVar13 != 0) && (param_1[0x5c] != 6)) {
    bVar9 = false;
    uVar14 = 0;
    piVar18 = param_1 + 0x78;
    do {
      if (*piVar18 == 1) {
        bVar9 = true;
        break;
      }
      uVar14 = uVar14 + 1;
      piVar18 = piVar18 + 0x1d;
    } while (uVar14 < 4);
    if (bVar9) {
      iVar24 = 2;
      iVar13 = 0;
      FUN_005aaf90();
      iVar13 = FUN_005a8a70(iVar13,iVar24);
      if (iVar13 == 0) {
        uVar14 = param_1[0x5d];
        piVar18 = (int *)0x0;
        iVar13 = 2;
        this = (void *)FUN_00596bd0();
        iVar24 = FUN_005a6480(this,uVar14);
        if (iVar24 != 0) {
          piVar18 = (int *)FUN_005d2190((int *)(iVar24 + 0x8c));
          if (*(int *)(iVar24 + 100) == 1) {
            return;
          }
          if (piVar18 != (int *)0x0) {
            iVar13 = *param_1;
            uVar15 = (**(code **)(*piVar18 + 0x20))();
            iVar13 = (**(code **)(iVar13 + 0x84))(uVar15);
            if (iVar13 == 4) {
              return;
            }
            iVar13 = (**(code **)(*param_1 + 0xa8))(piVar18);
          }
          piVar16 = (int *)(**(code **)(*(int *)(iVar24 + 0xa4) + 0x10))();
          while ((((piVar16 != (int *)0x0 && (iVar13 != 3)) && (iVar13 != 4)) && (iVar13 != 1))) {
            iVar13 = *param_1;
            uVar15 = (**(code **)(*piVar16 + 0x10))();
            iVar13 = (**(code **)(iVar13 + 0xa8))(uVar15);
            piVar16 = (int *)(**(code **)(*piVar16 + 8))();
          }
        }
        if (piVar18 == (int *)0x0) {
          uVar14 = 0;
        }
        else {
          uVar14 = piVar18[0x3b];
        }
        FUN_005b2140(param_1,uVar14);
        if (((iVar24 != 0) && ((*(int *)(iVar24 + 100) == 1 || (*(int *)(iVar24 + 100) == 4)))) ||
           ((iVar24 = FUN_005b7ce0((int)param_1), iVar24 != 0 && (iVar13 == 2)))) {
          fVar19 = FUN_005b18d0(param_1);
          iVar13 = FUN_005aaf90();
          piVar18 = FUN_005cdf50((void *)(iVar13 + 0x900),(int)param_1,(float)fVar19);
          iVar13 = (**(code **)(*piVar18 + 0xc))();
          if (iVar13 != 0) {
            fVar19 = (float10)fcos((float10)_DAT_0066c4bc * (float10)_DAT_0066c4f4);
            piVar16 = param_1 + 0x75;
            iVar13 = 4;
            fVar20 = (float10)fsin((float10)_DAT_0066c4bc * (float10)_DAT_0066c4f4);
            fVar21 = (float10)fcos((float10)_DAT_0066c4bc * (float10)_DAT_0066c50c);
            fVar22 = (float10)fsin((float10)(float)((float10)_DAT_0066c4bc * (float10)_DAT_0066c50c)
                                  );
            fVar1 = (float)param_1[6];
            fVar2 = (float)param_1[6];
            fVar3 = (float)param_1[4];
            fVar4 = (float)param_1[4];
            fVar5 = (float)param_1[6];
            fVar6 = (float)param_1[6];
            fVar7 = (float)param_1[4];
            fVar8 = (float)param_1[4];
            do {
              piVar16[-1] = 0;
              *piVar16 = 0;
              piVar16[1] = 0;
              piVar16[6] = 0;
              piVar16[-0x11] = 0;
              piVar16[-0x13] = 0;
              piVar16[-0x12] = 0;
              piVar16 = piVar16 + 0x1d;
              iVar13 = iVar13 + -1;
            } while (iVar13 != 0);
            for (iVar13 = (**(code **)(*piVar18 + 0xc))(); iVar13 != 0;
                iVar13 = FUN_0041c210(iVar13)) {
              iVar24 = *(int *)(iVar13 + 0x10);
              fVar10 = *(float *)(iVar24 + 0x38) - (float)param_1[0xe];
              fVar11 = *(float *)(iVar24 + 0x34) - (float)param_1[0xd];
              fStack_14 = -1.0;
              fVar12 = *(float *)(iVar24 + 0x30) - (float)param_1[0xc];
              if (param_1[0x50] == 1) {
LAB_005b3d80:
                iVar17 = 0;
              }
              else {
                bVar9 = _DAT_0066c4f0 <=
                        fVar12 * (fVar7 * (float)fVar21 - fVar5 * (float)fVar22) +
                        fVar10 * (fVar6 * (float)fVar21 + fVar8 * (float)fVar22);
                if (_DAT_0066c4f0 <=
                    fVar10 * (fVar2 * (float)fVar19 + fVar4 * (float)fVar20) +
                    fVar12 * (fVar3 * (float)fVar19 - fVar1 * (float)fVar20)) {
                  if (bVar9) goto LAB_005b3d80;
                  iVar17 = 3;
                }
                else if (bVar9) {
                  iVar17 = 1;
                }
                else {
                  iVar17 = 2;
                }
              }
              if (param_1[iVar17 * 0x1d + 0x78] == 1) {
                if (param_1[iVar17 * 0x1d + 0x5f] != 0) {
                  fStack_14 = SQRT(fVar12 * fVar12 + fVar10 * fVar10 + fVar11 * fVar11);
                  fVar23 = (float10)(**(code **)(*param_1 + 0x54))();
                  if (((float10)fStack_14 <= fVar23) &&
                     ((float)param_1[iVar17 * 0x1d + 0x62] <
                      (float)param_1[iVar17 * 0x1d + 0x6b] * (float)param_1[iVar17 * 0x1d + 0x6c] *
                      (float)param_1[iVar17 * 0x1d + 0x5f])) {
                    fVar23 = FUN_005b1770(param_1,(float)param_1[iVar17 * 0x1d + 0x6b] *
                                                  (float)param_1[iVar17 * 0x1d + 0x6c] *
                                                  (float)param_1[iVar17 * 0x1d + 0x5f]);
                    param_1[iVar17 * 0x1d + 0x62] = (int)(float)fVar23;
                    param_1[iVar17 * 0x1d + 0x75] = iVar24;
                  }
                }
                if (param_1[iVar17 * 0x1d + 0x61] != 0) {
                  if (fStack_14 == _DAT_0066c500) {
                    fStack_14 = SQRT(fVar12 * fVar12 + fVar10 * fVar10 + fVar11 * fVar11);
                  }
                  fVar23 = (float10)(**(code **)(*param_1 + 0x58))();
                  if (((float10)fStack_14 <= fVar23) &&
                     ((float)param_1[iVar17 * 0x1d + 99] <
                      (float)param_1[iVar17 * 0x1d + 0x71] * (float)param_1[iVar17 * 0x1d + 0x70] *
                      (float)param_1[iVar17 * 0x1d + 0x61])) {
                    fVar23 = FUN_005b1770(param_1,(float)param_1[iVar17 * 0x1d + 0x71] *
                                                  (float)param_1[iVar17 * 0x1d + 0x70] *
                                                  (float)param_1[iVar17 * 0x1d + 0x61]);
                    param_1[iVar17 * 0x1d + 99] = (int)(float)fVar23;
                    param_1[iVar17 * 0x1d + 0x76] = iVar24;
                  }
                }
              }
            }
            FUN_005b3f10(param_1);
          }
          if (piVar18 != (int *)0x0) {
            (**(code **)*piVar18)(1);
          }
        }
      }
    }
  }
  return;
}

