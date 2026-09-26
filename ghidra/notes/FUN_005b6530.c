
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

char __thiscall FUN_005b6530(int *param_1,int *param_2)

{
  float fVar1;
  float fVar2;
  float fVar3;
  float fVar4;
  float fVar5;
  float fVar6;
  float fVar7;
  float fVar8;
  float fVar9;
  bool bVar10;
  float fVar11;
  float fVar12;
  char cVar13;
  int iVar14;
  int *piVar15;
  int iVar16;
  float10 fVar17;
  float10 fVar18;
  float10 fVar19;
  float10 fVar20;
  float fStack_4;
  
  cVar13 = '\x04';
  iVar14 = (**(code **)(*param_2 + 0x20))();
  iVar16 = 1;
  if ((iVar14 == 1) && (param_1[0x78] == 0)) {
    return '\x01';
  }
  fVar1 = (float)param_2[0xd];
  fVar2 = (float)param_1[0xd];
  fVar11 = (float)param_2[0xe] - (float)param_1[0xe];
  fVar12 = (float)param_2[0xc] - (float)param_1[0xc];
  fStack_4 = -1.0;
  fVar17 = FUN_005b18d0(param_1);
  fVar1 = fVar12 * fVar12 + fVar11 * fVar11 + (fVar1 - fVar2) * (fVar1 - fVar2);
  if ((float)fVar17 * (float)fVar17 < fVar1) {
    return '\x02';
  }
  fVar17 = (float10)fcos((float10)_DAT_0066c4bc * (float10)_DAT_0066c4f4);
  piVar15 = param_1 + 0x75;
  iVar14 = 4;
  fVar18 = (float10)fsin((float10)_DAT_0066c4bc * (float10)_DAT_0066c4f4);
  fVar19 = (float10)fcos((float10)_DAT_0066c4bc * (float10)_DAT_0066c50c);
  fVar20 = (float10)fsin((float10)(float)((float10)_DAT_0066c4bc * (float10)_DAT_0066c50c));
  fVar2 = (float)param_1[6];
  fVar3 = (float)param_1[6];
  fVar4 = (float)param_1[4];
  fVar5 = (float)param_1[4];
  fVar6 = (float)param_1[6];
  fVar7 = (float)param_1[6];
  fVar8 = (float)param_1[4];
  fVar9 = (float)param_1[4];
  do {
    piVar15[-1] = 0;
    *piVar15 = 0;
    piVar15[1] = 0;
    piVar15[6] = 0;
    piVar15[-0x11] = 0;
    piVar15[-0x13] = 0;
    piVar15[-0x12] = 0;
    piVar15 = piVar15 + 0x1d;
    iVar14 = iVar14 + -1;
  } while (iVar14 != 0);
  if (param_1[0x50] != 1) {
    bVar10 = fVar12 * (fVar8 * (float)fVar19 - fVar6 * (float)fVar20) +
             fVar11 * (fVar7 * (float)fVar19 + fVar9 * (float)fVar20) < _DAT_0066c4f0;
    if (fVar12 * (fVar4 * (float)fVar17 - fVar2 * (float)fVar18) +
        fVar11 * (fVar3 * (float)fVar17 + fVar5 * (float)fVar18) < _DAT_0066c4f0) {
      if (bVar10) {
        iVar16 = 2;
      }
      goto LAB_005b66fd;
    }
    if (bVar10) {
      iVar16 = 3;
      goto LAB_005b66fd;
    }
  }
  iVar16 = 0;
LAB_005b66fd:
  if (param_1[iVar16 * 0x1d + 0x78] == 1) {
    if (param_1[iVar16 * 0x1d + 0x60] != 0) {
      fStack_4 = SQRT(fVar1);
      fVar17 = (float10)(**(code **)(*param_1 + 0x50))();
      if (((float10)fStack_4 <= fVar17) && (iVar14 = (**(code **)(*param_2 + 0x20))(), iVar14 == 0))
      {
        iVar14 = (**(code **)(*param_2 + 0x20))();
        if (iVar14 == 0) {
          fVar2 = (float)param_1[iVar16 * 0x1d + 0x68];
        }
        else {
          fVar2 = (float)param_1[iVar16 * 0x1d + 0x67];
        }
        param_1[iVar16 * 0x1d + 100] =
             (int)((float)param_1[iVar16 * 0x1d + 0x60] *
                  fVar2 * (float)param_1[iVar16 * 0x1d + 0x66]);
        param_1[(iVar16 + 4) * 0x1d] = (int)param_2;
      }
    }
    if (param_1[iVar16 * 0x1d + 0x5f] != 0) {
      if (fStack_4 == _DAT_0066c500) {
        fStack_4 = SQRT(fVar1);
      }
      fVar17 = (float10)(**(code **)(*param_1 + 0x54))();
      if ((float10)fStack_4 <= fVar17) {
        iVar14 = (**(code **)(*param_2 + 0x20))();
        if (iVar14 == 0) {
          fVar2 = (float)param_1[iVar16 * 0x1d + 0x6d];
        }
        else {
          fVar2 = (float)param_1[iVar16 * 0x1d + 0x6c];
        }
        param_1[iVar16 * 0x1d + 0x62] =
             (int)((float)param_1[iVar16 * 0x1d + 0x5f] *
                  (float)param_1[iVar16 * 0x1d + 0x6b] * fVar2);
        param_1[iVar16 * 0x1d + 0x75] = (int)param_2;
      }
    }
    if (param_1[iVar16 * 0x1d + 0x61] != 0) {
      if (fStack_4 == _DAT_0066c500) {
        fStack_4 = SQRT(fVar1);
      }
      fVar17 = (float10)(**(code **)(*param_1 + 0x58))();
      if ((float10)fStack_4 <= fVar17) {
        iVar14 = (**(code **)(*param_2 + 0x20))();
        if (iVar14 == 0) {
          fVar1 = (float)param_1[iVar16 * 0x1d + 0x72];
        }
        else {
          fVar1 = (float)param_1[iVar16 * 0x1d + 0x71];
        }
        param_1[iVar16 * 0x1d + 99] =
             (int)((float)param_1[iVar16 * 0x1d + 0x61] *
                  fVar1 * (float)param_1[iVar16 * 0x1d + 0x70]);
        param_1[iVar16 * 0x1d + 0x76] = (int)param_2;
      }
    }
  }
  else {
    cVar13 = '\x01';
  }
  if (cVar13 == '\x04') {
    cVar13 = FUN_005b3f10(param_1);
  }
  return cVar13;
}

