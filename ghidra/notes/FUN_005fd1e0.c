
undefined4 __thiscall
FUN_005fd1e0(void *this,uint *param_1,int param_2,int param_3,int param_4,int param_5,int param_6,
            uint param_7)

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
  byte *pbVar16;
  byte *pbVar17;
  int iVar18;
  byte *pbVar19;
  int local_c;
  
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
    local_c = 1;
  }
  else {
    local_c = -1;
  }
  if (param_6 == 0) {
    param_6 = iVar8 - param_2;
  }
  if (param_7 == 0) {
    param_7 = iVar7 - param_3;
  }
  if (iVar10 - param_5 < (int)param_7) {
    param_7 = iVar10 - param_5;
  }
  if (iVar4 - param_4 < param_6) {
    param_6 = iVar4 - param_4;
  }
  if ((int)param_7 < 1) {
    return 0;
  }
  iVar4 = param_2 - param_4;
  iVar7 = param_6 + param_2;
  cVar12 = (char)param_2;
  if (local_c == -1) {
    iVar10 = ((iVar10 + param_3) - param_5) + -1;
    iVar8 = param_3 + -1;
    while ((int)((param_7 - 1) + param_3) < iVar10) {
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
    pbVar16 = (byte *)(iVar11 * uVar9 + iVar4 + iVar5);
    iVar11 = iVar4;
    iVar2 = iVar10;
    while (iVar18 = iVar2, param_3 = iVar10, iVar8 < iVar18) {
      sVar1 = *(short *)pbVar6;
      if (sVar1 == 0) {
        iVar10 = iVar18 + -1;
        iVar11 = FUN_005fc9e0(param_1,iVar10);
        pbVar16 = (byte *)(iVar11 * uVar9 + iVar4 + iVar5);
        pbVar6 = pbVar6 + 2;
        iVar11 = iVar4;
        iVar2 = iVar10;
      }
      else {
        iVar10 = iVar8;
        iVar2 = iVar8;
        if (sVar1 != 0x100) {
          if (sVar1 == 0x200) {
            iVar18 = iVar18 - (uint)pbVar6[3];
            iVar11 = iVar11 + (uint)pbVar6[2];
            iVar10 = FUN_005fc9e0(param_1,iVar18);
            pbVar16 = (byte *)(iVar10 * uVar9 + iVar11 + iVar5);
            pbVar6 = pbVar6 + 4;
            param_3 = iVar18;
          }
          bVar15 = *pbVar6;
          iVar10 = param_3;
          if (bVar15 == 0) {
            bVar15 = pbVar6[1];
            pbVar6 = pbVar6 + 2;
            param_7 = (uint)bVar15;
            if (iVar11 < iVar7) {
              bVar3 = bVar15;
              if (iVar11 < param_2) {
                uVar14 = (uint)bVar15;
                if (param_2 - iVar11 < (int)uVar14) {
                  bVar3 = (bVar15 + (char)iVar11) - cVar12;
                  pbVar16 = pbVar16 + (param_2 - iVar11);
                  param_7 = (uint)bVar3;
                  pbVar6 = pbVar6 + (param_2 - iVar11);
                  iVar11 = param_2;
                }
                else {
                  iVar11 = iVar11 + uVar14;
                  pbVar16 = pbVar16 + uVar14;
                  pbVar6 = pbVar6 + uVar14;
                  bVar3 = 0;
                  param_7 = 0;
                }
              }
              if (bVar3 != 0) {
                uVar14 = param_7;
                if (param_6 - iVar11 < (int)param_7) {
                  uVar14 = param_6 - iVar11;
                }
                pbVar17 = pbVar6;
                pbVar19 = pbVar16;
                for (uVar13 = uVar14 >> 2; uVar13 != 0; uVar13 = uVar13 - 1) {
                  *(undefined4 *)pbVar19 = *(undefined4 *)pbVar17;
                  pbVar17 = pbVar17 + 4;
                  pbVar19 = pbVar19 + 4;
                }
                pbVar16 = pbVar16 + param_7;
                for (uVar14 = uVar14 & 3; uVar14 != 0; uVar14 = uVar14 - 1) {
                  *pbVar19 = *pbVar17;
                  pbVar17 = pbVar17 + 1;
                  pbVar19 = pbVar19 + 1;
                }
                pbVar6 = pbVar6 + param_7;
                iVar11 = iVar11 + param_7;
                iVar18 = param_3;
              }
            }
            iVar2 = iVar18;
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
                  pbVar16 = pbVar16 + (param_2 - iVar11);
                  iVar11 = param_2;
                }
                else {
                  iVar11 = iVar11 + uVar14;
                  pbVar16 = pbVar16 + uVar14;
                  bVar15 = 0;
                  param_7 = 0;
                }
              }
              if (bVar15 != 0) {
                uVar14 = param_7;
                if (param_6 - iVar11 < (int)param_7) {
                  uVar14 = param_6 - iVar11;
                }
                bVar15 = pbVar6[1];
                pbVar17 = pbVar16;
                for (uVar13 = uVar14 >> 2; uVar13 != 0; uVar13 = uVar13 - 1) {
                  *(uint *)pbVar17 = CONCAT22(CONCAT11(bVar15,bVar15),CONCAT11(bVar15,bVar15));
                  pbVar17 = pbVar17 + 4;
                }
                for (uVar14 = uVar14 & 3; uVar14 != 0; uVar14 = uVar14 - 1) {
                  *pbVar17 = bVar15;
                  pbVar17 = pbVar17 + 1;
                }
                pbVar16 = pbVar16 + param_7;
                iVar11 = iVar11 + param_7;
                iVar18 = param_3;
              }
            }
            pbVar6 = pbVar6 + 2;
            iVar2 = iVar18;
          }
        }
      }
    }
  }
  else {
    iVar10 = param_7 + param_3;
    iVar8 = param_3 - param_5;
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
    pbVar16 = (byte *)(iVar11 * uVar9 + iVar4 + iVar5);
    iVar11 = iVar4;
    while (param_3 = iVar8, iVar10 < param_3) {
      sVar1 = *(short *)pbVar6;
      if (sVar1 == 0) {
        iVar8 = FUN_005fc9e0(param_1,param_3 + -1);
        pbVar16 = (byte *)(iVar8 * uVar9 + iVar4 + iVar5);
        pbVar6 = pbVar6 + 2;
        iVar11 = iVar4;
        iVar8 = param_3 + -1;
      }
      else {
        iVar8 = iVar10;
        if (sVar1 != 0x100) {
          if (sVar1 == 0x200) {
            param_3 = param_3 - (uint)pbVar6[3];
            iVar11 = iVar11 + (uint)pbVar6[2];
            iVar8 = FUN_005fc9e0(param_1,param_3);
            pbVar16 = (byte *)(iVar8 * uVar9 + iVar11 + iVar5);
            pbVar6 = pbVar6 + 4;
          }
          bVar15 = *pbVar6;
          iVar8 = param_3;
          if (bVar15 == 0) {
            bVar15 = pbVar6[1];
            pbVar6 = pbVar6 + 2;
            param_7 = (uint)bVar15;
            if (iVar11 < iVar7) {
              bVar3 = bVar15;
              if (iVar11 < param_2) {
                uVar14 = (uint)bVar15;
                if (param_2 - iVar11 < (int)uVar14) {
                  bVar3 = (bVar15 + (char)iVar11) - cVar12;
                  pbVar16 = pbVar16 + (param_2 - iVar11);
                  param_7 = (uint)bVar3;
                  pbVar6 = pbVar6 + (param_2 - iVar11);
                  iVar11 = param_2;
                }
                else {
                  iVar11 = iVar11 + uVar14;
                  pbVar16 = pbVar16 + uVar14;
                  pbVar6 = pbVar6 + uVar14;
                  bVar3 = 0;
                  param_7 = 0;
                }
              }
              if (bVar3 != 0) {
                uVar14 = param_7;
                if (param_6 - iVar11 < (int)param_7) {
                  uVar14 = param_6 - iVar11;
                }
                pbVar17 = pbVar6;
                pbVar19 = pbVar16;
                for (uVar13 = uVar14 >> 2; uVar13 != 0; uVar13 = uVar13 - 1) {
                  *(undefined4 *)pbVar19 = *(undefined4 *)pbVar17;
                  pbVar17 = pbVar17 + 4;
                  pbVar19 = pbVar19 + 4;
                }
                pbVar16 = pbVar16 + param_7;
                pbVar6 = pbVar6 + param_7;
                iVar11 = iVar11 + param_7;
                for (uVar14 = uVar14 & 3; uVar14 != 0; uVar14 = uVar14 - 1) {
                  *pbVar19 = *pbVar17;
                  pbVar17 = pbVar17 + 1;
                  pbVar19 = pbVar19 + 1;
                }
              }
            }
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
                  pbVar16 = pbVar16 + (param_2 - iVar11);
                  iVar11 = param_2;
                }
                else {
                  iVar11 = iVar11 + uVar14;
                  pbVar16 = pbVar16 + uVar14;
                  bVar15 = 0;
                  param_7 = 0;
                }
              }
              if (bVar15 != 0) {
                uVar14 = param_7;
                if (param_6 - iVar11 < (int)param_7) {
                  uVar14 = param_6 - iVar11;
                }
                bVar15 = pbVar6[1];
                pbVar17 = pbVar16;
                for (uVar13 = uVar14 >> 2; uVar13 != 0; uVar13 = uVar13 - 1) {
                  *(uint *)pbVar17 = CONCAT22(CONCAT11(bVar15,bVar15),CONCAT11(bVar15,bVar15));
                  pbVar17 = pbVar17 + 4;
                }
                for (uVar14 = uVar14 & 3; uVar14 != 0; uVar14 = uVar14 - 1) {
                  *pbVar17 = bVar15;
                  pbVar17 = pbVar17 + 1;
                }
                pbVar16 = pbVar16 + param_7;
                iVar11 = iVar11 + param_7;
              }
            }
            pbVar6 = pbVar6 + 2;
          }
        }
      }
    }
  }
  FUN_005fd170(param_1);
  return 1;
}

