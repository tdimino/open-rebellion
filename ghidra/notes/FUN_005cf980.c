
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __fastcall FUN_005cf980(int param_1)

{
  float fVar1;
  float fVar2;
  float fVar3;
  bool bVar4;
  int iVar5;
  int *piVar6;
  undefined4 uVar7;
  int iVar8;
  void *pvVar9;
  int *piVar10;
  int iVar11;
  int iVar12;
  uint uVar13;
  int *piVar14;
  float *pfVar15;
  uint uVar16;
  int iVar17;
  int iStack_34;
  undefined1 auStack_10 [16];
  
  iVar17 = *(int *)(param_1 + 8);
  iVar5 = FUN_005c7150(iVar17);
  if (*(int *)(*(int *)(param_1 + 8) + 0x34) == 0) {
    for (piVar6 = (int *)(**(code **)(*(int *)(*(int *)(param_1 + 8) + 0x1c) + 0x10))();
        piVar6 != (int *)0x0; piVar6 = (int *)(**(code **)(*piVar6 + 8))()) {
      uVar7 = (**(code **)(*piVar6 + 0x10))();
      FUN_005aaf90();
      iVar8 = FUN_005ad870(uVar7);
      if (iVar8 == 1) {
        iVar8 = (**(code **)(*piVar6 + 0x10))();
        iStack_34 = 0;
        uVar16 = *(uint *)(iVar8 + 0x164);
        if (*(int *)(iVar8 + 0x35c) == 0) {
          if (uVar16 != 0) {
            pvVar9 = (void *)FUN_00596bd0();
            piVar10 = FUN_005a6360(pvVar9,uVar16);
            if (piVar10 != (int *)0x0) {
              FUN_005aaf90();
              iVar11 = FUN_005a9330();
              if (iVar11 == 0) {
                piVar14 = (int *)0x0;
              }
              else {
                piVar14 = (int *)(iVar11 + 0x1c);
              }
              if (piVar14 == piVar10) {
                iVar11 = FUN_005ba420(iVar11);
                if (iVar11 == 1) {
                  iVar11 = FUN_005b84e0(piVar10,iVar8);
joined_r0x005cfa76:
                  if (iVar11 == 1) goto LAB_005cfa78;
                }
              }
              else {
                piVar14 = piVar10 + -0x106;
                FUN_005aaf90();
                iVar11 = FUN_005ad870(piVar14);
                if (iVar11 == 1) {
                  iVar11 = FUN_005b84e0(piVar10,iVar8);
                  goto joined_r0x005cfa76;
                }
              }
            }
          }
        }
        else {
LAB_005cfa78:
          iStack_34 = 1;
        }
        uVar16 = 1;
        if (*(int *)(iVar8 + 0x35c) == 0) {
          iVar11 = *(int *)(*(int *)(param_1 + 8) + 0x50);
          iVar12 = FUN_005aaf90();
          for (piVar10 = (int *)(**(code **)(*(int *)(*(int *)(iVar12 + 8 + iVar11 * 4) + 0x58) +
                                            0x10))(); piVar10 != (int *)0x0;
              piVar10 = (int *)(**(code **)(*piVar10 + 8))()) {
            iVar12 = (**(code **)(*piVar10 + 0x10))();
            iVar11 = iVar12;
            FUN_005aaf90();
            iVar11 = FUN_005ad870(iVar11);
            if (iVar11 == 1) {
              iVar11 = FUN_005b84e0((void *)(iVar12 + 0x418),iVar8);
              if (iVar11 == 1) {
                iStack_34 = 1;
                break;
              }
              uVar13 = FUN_005b8480(iVar12 + 0x418);
              uVar16 = uVar16 & uVar13;
            }
          }
          if ((iStack_34 == 0) && (*(int *)(iVar8 + 0x168) == 1)) {
            FUN_005aaf90();
            iVar11 = FUN_005a9330();
            if ((iVar11 != 0) && (iVar12 = FUN_005ba420(iVar11), iVar12 != 0)) {
              iStack_34 = FUN_005b84e0((void *)(iVar11 + 0x1c),iVar8);
              uVar13 = FUN_005b8480(iVar11 + 0x1c);
              uVar16 = uVar16 & uVar13;
            }
          }
          if (iStack_34 == 0) {
            if (*(int *)(param_1 + 0x10) == 0) {
              *(undefined4 *)(param_1 + 0x10) = 1;
              iVar8 = iVar5;
              if (*(int *)(iVar17 + 0x50) == 0) {
                if (iVar5 == 0) {
                  uVar13 = 0x8b;
                  iVar8 = 1;
                }
                else if (iVar5 == 1) {
                  uVar13 = 0x8c;
                }
                else if (iVar5 == 2) {
                  uVar13 = 0x8d;
                  iVar8 = 1;
                }
                else {
                  if (iVar5 != 3) goto LAB_005cfbec;
                  uVar13 = 0x8e;
                  iVar8 = 1;
                }
              }
              else if (iVar5 == 0) {
                uVar13 = 0x109;
                iVar8 = 1;
              }
              else if (iVar5 == 1) {
                uVar13 = 0x10a;
              }
              else if (iVar5 == 2) {
                iVar8 = 1;
                uVar13 = 0x10b;
              }
              else {
                if (iVar5 != 3) goto LAB_005cfbec;
                iVar8 = 1;
                uVar13 = 0x10c;
              }
              pvVar9 = (void *)FUN_005ba8b0();
              FUN_005ba980(pvVar9,uVar13,iVar8);
              goto LAB_005cfbec;
            }
          }
          else {
LAB_005cfbec:
            if (*(int *)(param_1 + 0x10) == 0) goto LAB_005cfcbb;
          }
          if (((iStack_34 == 0) && (uVar16 != 0)) && (*(int *)(param_1 + 4) == 0)) {
            *(undefined4 *)(param_1 + 4) = 1;
            iVar8 = *(int *)(*(int *)(param_1 + 8) + 0x50);
            iVar11 = FUN_005aaf90();
            if (*(int *)(*(int *)(iVar11 + 8 + iVar8 * 4) + 0x84) == 0) {
              iVar8 = *(int *)(*(int *)(param_1 + 8) + 0x50);
              iVar11 = FUN_005aaf90();
              if (iVar8 == *(int *)(iVar11 + 0x9ec)) {
                iVar8 = *(int *)(param_1 + 8);
                pvVar9 = (void *)FUN_0059ed70();
                FUN_0059f680(pvVar9,iVar8);
              }
              (**(code **)(**(int **)(param_1 + 8) + 100))(0);
            }
            else {
              for (piVar10 = (int *)(**(code **)(*(int *)(*(int *)(param_1 + 8) + 0x1c) + 0x10))();
                  piVar10 != (int *)0x0; piVar10 = (int *)(**(code **)(*piVar10 + 8))()) {
                iVar8 = (**(code **)(*piVar10 + 0x10))();
                if (*(int *)(iVar8 + 0x35c) == 0) {
                  piVar14 = (int *)FUN_005aaf90();
                  iVar8 = *piVar14;
                  uVar7 = (**(code **)(*piVar10 + 4))(1);
                  uVar7 = (**(code **)(*piVar10 + 4))(uVar7);
                  (**(code **)(iVar8 + 0x28))(0x2f,uVar7);
                }
              }
            }
          }
        }
      }
LAB_005cfcbb:
    }
    bVar4 = true;
    for (piVar6 = (int *)(**(code **)(*(int *)(*(int *)(param_1 + 8) + 0x1c) + 0x10))();
        piVar6 != (int *)0x0; piVar6 = (int *)(**(code **)(*piVar6 + 8))()) {
      iVar8 = (**(code **)(*piVar6 + 0x10))();
      if (*(int *)(iVar8 + 0x35c) == 2) {
        fVar1 = *(float *)(iVar8 + 0x30);
        fVar2 = *(float *)(iVar8 + 0x34);
        fVar3 = *(float *)(iVar8 + 0x38);
        uVar16 = *(uint *)(iVar8 + 0x164);
        if (uVar16 != 0) {
          pvVar9 = (void *)FUN_00596bd0();
          piVar10 = FUN_005a6360(pvVar9,uVar16);
          if ((piVar10 != (int *)0x0) &&
             (pfVar15 = (float *)(**(code **)(*piVar10 + 4))(auStack_10), fVar2 = fVar2 - pfVar15[1]
             , fVar3 = fVar3 - pfVar15[2], fVar1 = fVar1 - *pfVar15,
             SQRT(fVar1 * fVar1 + fVar3 * fVar3 + fVar2 * fVar2) < _DAT_0066d088)) {
            *(undefined4 *)(iVar8 + 0x35c) = 3;
            uVar7 = *(undefined4 *)(iVar8 + 0xec);
            iVar11 = piVar10[1];
            piVar10 = (int *)FUN_005aaf90();
            (**(code **)(*piVar10 + 0x28))(0x1a,iVar11,uVar7,1);
          }
        }
      }
      if (*(int *)(iVar8 + 0x35c) != 4) {
        bVar4 = false;
      }
    }
    if ((bVar4) && (*(int *)(param_1 + 0x10) == 0)) {
      *(undefined4 *)(param_1 + 0x10) = 1;
      if (*(int *)(iVar17 + 0x50) == 0) {
        if (iVar5 == 0) {
          uVar16 = 0x87;
        }
        else if (iVar5 == 1) {
          uVar16 = 0x88;
        }
        else if (iVar5 == 2) {
          uVar16 = 0x89;
        }
        else {
          if (iVar5 != 3) {
            return;
          }
          uVar16 = 0x8a;
        }
      }
      else if (iVar5 == 0) {
        uVar16 = 0x105;
      }
      else if (iVar5 == 1) {
        uVar16 = 0x106;
      }
      else if (iVar5 == 2) {
        uVar16 = 0x107;
      }
      else {
        if (iVar5 != 3) {
          return;
        }
        uVar16 = 0x108;
      }
      iVar17 = 1;
      pvVar9 = (void *)FUN_005ba8b0();
      FUN_005ba980(pvVar9,uVar16,iVar17);
    }
  }
  return;
}

