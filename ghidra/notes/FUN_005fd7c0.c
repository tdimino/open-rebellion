
undefined4 __thiscall
FUN_005fd7c0(void *this,uint *param_1,int param_2,int param_3,byte param_4,int param_5,int param_6,
            uint param_7,int param_8)

{
  short sVar1;
  int iVar2;
  byte bVar3;
  int iVar4;
  int iVar5;
  byte *pbVar6;
  int iVar7;
  int iVar8;
  uint uVar9;
  int iVar10;
  int iVar11;
  char cVar12;
  uint uVar13;
  uint uVar14;
  byte bVar15;
  int iVar16;
  byte *pbVar17;
  byte *pbVar18;
  int local_18;
  
  iVar4 = FUN_005fc0e0(this);
  iVar5 = FUN_00583c40((int)param_1);
  pbVar6 = (byte *)FUN_00583c40((int)this);
  iVar7 = FUN_005fc0f0((int *)param_1);
  iVar8 = FUN_005fc0e0((int *)param_1);
  uVar9 = FUN_005fc9c0(param_1);
  FUN_005fc9c0(this);
  if (iVar7 < 0) {
    iVar7 = -iVar7;
  }
  iVar10 = FUN_005fc0f0(this);
  if (iVar10 < 0) {
    iVar10 = -iVar10;
    local_18 = 1;
  }
  else {
    local_18 = -1;
  }
  if (param_7 == 0) {
    param_7 = iVar8 - param_2;
  }
  if (param_8 == 0) {
    param_8 = iVar7 - param_3;
  }
  if (iVar10 - param_6 < param_8) {
    param_8 = iVar10 - param_6;
  }
  if (iVar4 - param_5 < (int)param_7) {
    param_7 = iVar4 - param_5;
  }
  if (0 < param_8) {
    iVar4 = param_2 - param_5;
    iVar7 = param_7 + param_2;
    cVar12 = (char)param_2;
    if (local_18 == -1) {
      iVar8 = param_3 + -1;
      iVar10 = ((iVar10 + param_3) - param_6) + -1;
      while (param_8 + -1 + param_3 < iVar10) {
        sVar1 = *(short *)pbVar6;
        if (sVar1 == 0) {
          iVar10 = iVar10 + -1;
        }
        else {
          if (sVar1 == 0x100) {
            return 0;
          }
          if (sVar1 == 0x200) {
            iVar10 = iVar10 - (uint)pbVar6[3];
            pbVar6 = pbVar6 + 2;
          }
        }
        pbVar6 = pbVar6 + 2;
      }
      iVar11 = FUN_005fc9e0(param_1,iVar10);
      pbVar17 = (byte *)(iVar11 * uVar9 + iVar4 + iVar5);
      iVar11 = iVar4;
      iVar2 = iVar10;
      while (iVar16 = iVar2, param_8 = iVar10, iVar8 < iVar16) {
        sVar1 = *(short *)pbVar6;
        if (sVar1 == 0) {
          iVar10 = iVar16 + -1;
          iVar11 = FUN_005fc9e0(param_1,iVar10);
          pbVar17 = (byte *)(iVar11 * uVar9 + iVar4 + iVar5);
          pbVar6 = pbVar6 + 2;
          iVar11 = iVar4;
          iVar2 = iVar10;
        }
        else {
          iVar10 = iVar8;
          iVar2 = iVar8;
          if (sVar1 != 0x100) {
            if (sVar1 == 0x200) {
              iVar16 = iVar16 - (uint)pbVar6[3];
              iVar11 = iVar11 + (uint)pbVar6[2];
              iVar10 = FUN_005fc9e0(param_1,iVar16);
              pbVar17 = (byte *)(iVar10 * uVar9 + iVar11 + iVar5);
              pbVar6 = pbVar6 + 4;
              param_8 = iVar16;
            }
            bVar15 = *pbVar6;
            iVar10 = param_8;
            if (bVar15 == 0) {
              bVar15 = pbVar6[1];
              pbVar6 = pbVar6 + 2;
              param_7 = (uint)bVar15;
              if (iVar11 < iVar7) {
                bVar3 = bVar15;
                if (iVar11 < param_2) {
                  uVar14 = (uint)bVar15;
                  if (param_2 - iVar11 < (int)uVar14) {
                    pbVar17 = pbVar17 + (param_2 - iVar11);
                    bVar3 = (bVar15 + (char)iVar11) - cVar12;
                    param_7 = (uint)bVar3;
                    pbVar6 = pbVar6 + (param_2 - iVar11);
                    iVar11 = param_2;
                  }
                  else {
                    iVar11 = iVar11 + uVar14;
                    pbVar17 = pbVar17 + uVar14;
                    pbVar6 = pbVar6 + uVar14;
                    bVar3 = 0;
                    param_7 = 0;
                  }
                }
                if (bVar3 != 0) {
                  uVar14 = iVar7 - iVar11;
                  if ((int)param_7 <= iVar7 - iVar11) {
                    uVar14 = param_7;
                  }
                  iVar11 = iVar11 + param_7;
                  for (; uVar14 != 0; uVar14 = uVar14 - 1) {
                    if (*pbVar6 != param_4) {
                      *pbVar17 = *pbVar6;
                    }
                    pbVar17 = pbVar17 + 1;
                    pbVar6 = pbVar6 + 1;
                    bVar3 = bVar3 - 1;
                    param_7 = (uint)bVar3;
                  }
                  pbVar17 = pbVar17 + param_7;
                  pbVar6 = pbVar6 + param_7;
                }
              }
              iVar2 = iVar16;
              if ((bVar15 & 1) != 0) {
                pbVar6 = pbVar6 + 1;
              }
            }
            else {
              param_7 = (uint)bVar15;
              if (iVar11 < iVar7) {
                if (iVar11 < param_2) {
                  uVar14 = (uint)bVar15;
                  if (param_2 - iVar11 < (int)uVar14) {
                    bVar15 = bVar15 + ((char)iVar11 - cVar12);
                    param_7 = (uint)bVar15;
                    pbVar17 = pbVar17 + (param_2 - iVar11);
                    iVar11 = param_2;
                  }
                  else {
                    iVar11 = iVar11 + uVar14;
                    pbVar17 = pbVar17 + uVar14;
                    bVar15 = 0;
                    param_7 = 0;
                  }
                }
                if (bVar15 != 0) {
                  bVar15 = pbVar6[1];
                  if (bVar15 != param_4) {
                    uVar14 = iVar7 - iVar11;
                    if ((int)param_7 <= iVar7 - iVar11) {
                      uVar14 = param_7;
                    }
                    pbVar18 = pbVar17;
                    for (uVar13 = uVar14 >> 2; uVar13 != 0; uVar13 = uVar13 - 1) {
                      *(uint *)pbVar18 = CONCAT22(CONCAT11(bVar15,bVar15),CONCAT11(bVar15,bVar15));
                      pbVar18 = pbVar18 + 4;
                    }
                    for (uVar14 = uVar14 & 3; iVar16 = param_8, uVar14 != 0; uVar14 = uVar14 - 1) {
                      *pbVar18 = bVar15;
                      pbVar18 = pbVar18 + 1;
                    }
                  }
                  pbVar17 = pbVar17 + param_7;
                  iVar11 = iVar11 + param_7;
                }
              }
              pbVar6 = pbVar6 + 2;
              iVar2 = iVar16;
            }
          }
        }
      }
    }
    else {
      iVar8 = param_3 - param_6;
      iVar10 = param_8 + param_3;
      while (iVar8 < param_3) {
        sVar1 = *(short *)pbVar6;
        if (sVar1 == 0) {
          iVar8 = iVar8 + 1;
        }
        else {
          if (sVar1 == 0x100) {
            return 0;
          }
          if (sVar1 == 0x200) {
            iVar8 = iVar8 + (uint)pbVar6[3];
            pbVar6 = pbVar6 + 2;
          }
        }
        pbVar6 = pbVar6 + 2;
      }
      iVar11 = FUN_005fc9e0(param_1,iVar8);
      pbVar17 = (byte *)(iVar11 * uVar9 + iVar4 + iVar5);
      iVar11 = iVar4;
      iVar2 = iVar8;
      while (iVar16 = iVar2, param_8 = iVar8, iVar10 < iVar16) {
        sVar1 = *(short *)pbVar6;
        if (sVar1 == 0) {
          iVar8 = iVar16 + -1;
          iVar11 = FUN_005fc9e0(param_1,iVar8);
          pbVar17 = (byte *)(iVar11 * uVar9 + iVar4 + iVar5);
          pbVar6 = pbVar6 + 2;
          iVar11 = iVar4;
          iVar2 = iVar8;
        }
        else {
          iVar8 = iVar10;
          iVar2 = iVar10;
          if (sVar1 != 0x100) {
            if (sVar1 == 0x200) {
              iVar16 = iVar16 - (uint)pbVar6[3];
              iVar11 = iVar11 + (uint)pbVar6[2];
              iVar8 = FUN_005fc9e0(param_1,iVar16);
              pbVar17 = (byte *)(iVar8 * uVar9 + iVar11 + iVar5);
              pbVar6 = pbVar6 + 4;
              param_8 = iVar16;
            }
            bVar15 = *pbVar6;
            iVar8 = param_8;
            if (bVar15 == 0) {
              bVar15 = pbVar6[1];
              pbVar6 = pbVar6 + 2;
              param_7 = (uint)bVar15;
              if (iVar11 < iVar7) {
                bVar3 = bVar15;
                if (iVar11 < param_2) {
                  uVar14 = (uint)bVar15;
                  if (param_2 - iVar11 < (int)uVar14) {
                    pbVar17 = pbVar17 + (param_2 - iVar11);
                    bVar3 = (bVar15 + (char)iVar11) - cVar12;
                    param_7 = (uint)bVar3;
                    pbVar6 = pbVar6 + (param_2 - iVar11);
                    iVar11 = param_2;
                  }
                  else {
                    iVar11 = iVar11 + uVar14;
                    pbVar17 = pbVar17 + uVar14;
                    pbVar6 = pbVar6 + uVar14;
                    bVar3 = 0;
                    param_7 = 0;
                  }
                }
                if (bVar3 != 0) {
                  uVar14 = iVar7 - iVar11;
                  if ((int)param_7 <= iVar7 - iVar11) {
                    uVar14 = param_7;
                  }
                  iVar11 = iVar11 + param_7;
                  for (; uVar14 != 0; uVar14 = uVar14 - 1) {
                    if (*pbVar6 != param_4) {
                      *pbVar17 = *pbVar6;
                    }
                    pbVar17 = pbVar17 + 1;
                    pbVar6 = pbVar6 + 1;
                    bVar3 = bVar3 - 1;
                    param_7 = (uint)bVar3;
                  }
                  pbVar17 = pbVar17 + param_7;
                  pbVar6 = pbVar6 + param_7;
                }
              }
              iVar2 = param_8;
              if ((bVar15 & 1) != 0) {
                pbVar6 = pbVar6 + 1;
              }
            }
            else {
              pbVar6 = pbVar6 + 1;
              param_7 = (uint)bVar15;
              if (iVar11 < iVar7) {
                if (iVar11 < param_2) {
                  uVar14 = (uint)bVar15;
                  if (param_2 - iVar11 < (int)uVar14) {
                    bVar15 = bVar15 + ((char)iVar11 - cVar12);
                    uVar14 = param_2 - iVar11;
                    param_7 = (uint)bVar15;
                    iVar11 = param_2;
                  }
                  else {
                    iVar11 = iVar11 + uVar14;
                    bVar15 = 0;
                    param_7 = 0;
                  }
                  pbVar17 = pbVar17 + uVar14;
                }
                if (bVar15 != 0) {
                  bVar15 = *pbVar6;
                  if (bVar15 != param_4) {
                    uVar14 = iVar7 - iVar11;
                    if ((int)param_7 <= iVar7 - iVar11) {
                      uVar14 = param_7;
                    }
                    pbVar18 = pbVar17;
                    for (uVar13 = uVar14 >> 2; uVar13 != 0; uVar13 = uVar13 - 1) {
                      *(uint *)pbVar18 = CONCAT22(CONCAT11(bVar15,bVar15),CONCAT11(bVar15,bVar15));
                      pbVar18 = pbVar18 + 4;
                    }
                    for (uVar14 = uVar14 & 3; iVar16 = param_8, uVar14 != 0; uVar14 = uVar14 - 1) {
                      *pbVar18 = bVar15;
                      pbVar18 = pbVar18 + 1;
                    }
                  }
                  pbVar17 = pbVar17 + param_7;
                  pbVar6 = pbVar6 + param_7;
                  iVar11 = iVar11 + param_7;
                }
              }
              pbVar6 = pbVar6 + 1;
              iVar2 = iVar16;
            }
          }
        }
      }
    }
    FUN_005fd170(param_1);
    return 1;
  }
  return 0;
}

