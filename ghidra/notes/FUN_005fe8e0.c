
undefined4 __thiscall
FUN_005fe8e0(void *this,uint *param_1,int param_2,int param_3,byte param_4,int param_5,int param_6,
            uint param_7,int param_8)

{
  undefined1 uVar1;
  short sVar2;
  int iVar3;
  int iVar4;
  int iVar5;
  byte *pbVar6;
  int iVar7;
  int iVar8;
  uint uVar9;
  int iVar10;
  int iVar11;
  byte bVar12;
  byte bVar13;
  uint uVar14;
  uint uVar15;
  char cVar16;
  int iVar17;
  undefined4 *puVar18;
  undefined4 *puVar19;
  int local_10;
  
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
    local_10 = 1;
  }
  else {
    local_10 = -1;
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
  if (param_8 < 1) {
    return 0;
  }
  iVar4 = param_2 - param_5;
  iVar7 = param_7 + param_2;
  cVar16 = (char)param_2;
  if (local_10 == -1) {
    iVar8 = param_3 + -1;
    iVar10 = ((iVar10 + param_3) - param_6) + -1;
    while (param_8 + -1 + param_3 < iVar10) {
      sVar2 = *(short *)pbVar6;
      if (sVar2 == 0) {
        iVar10 = iVar10 + -1;
      }
      else {
        if (sVar2 == 0x100) {
          return 0;
        }
        if (sVar2 == 0x200) {
          iVar10 = iVar10 - (uint)pbVar6[3];
          pbVar6 = pbVar6 + 2;
        }
      }
      pbVar6 = pbVar6 + 2;
    }
    iVar11 = FUN_005fc9e0(param_1,iVar10);
    puVar18 = (undefined4 *)(iVar11 * uVar9 + iVar4 + iVar5);
    iVar11 = iVar4;
    iVar3 = iVar10;
    while (iVar17 = iVar3, param_8 = iVar10, iVar8 < iVar17) {
      sVar2 = *(short *)pbVar6;
      if (sVar2 == 0) {
        iVar10 = iVar17 + -1;
        iVar11 = FUN_005fc9e0(param_1,iVar10);
        puVar18 = (undefined4 *)(iVar11 * uVar9 + iVar4 + iVar5);
        pbVar6 = pbVar6 + 2;
        iVar11 = iVar4;
        iVar3 = iVar10;
      }
      else {
        iVar10 = iVar8;
        iVar3 = iVar8;
        if (sVar2 != 0x100) {
          if (sVar2 == 0x200) {
            iVar17 = iVar17 - (uint)pbVar6[3];
            iVar11 = iVar11 + (uint)pbVar6[2];
            iVar10 = FUN_005fc9e0(param_1,iVar17);
            puVar18 = (undefined4 *)(iVar10 * uVar9 + iVar11 + iVar5);
            pbVar6 = pbVar6 + 4;
            param_8 = iVar17;
          }
          bVar12 = *pbVar6;
          iVar10 = param_8;
          if (bVar12 == 0) {
            bVar12 = pbVar6[1];
            pbVar6 = pbVar6 + 2;
            param_7 = (uint)bVar12;
            if (iVar11 < iVar7) {
              bVar13 = bVar12;
              if (iVar11 < param_2) {
                uVar15 = (uint)bVar12;
                if (param_2 - iVar11 < (int)uVar15) {
                  puVar18 = (undefined4 *)((int)puVar18 + (param_2 - iVar11));
                  bVar13 = (bVar12 + (char)iVar11) - cVar16;
                  param_7 = (uint)bVar13;
                  pbVar6 = pbVar6 + (param_2 - iVar11);
                  iVar11 = param_2;
                }
                else {
                  iVar11 = iVar11 + uVar15;
                  puVar18 = (undefined4 *)((int)puVar18 + uVar15);
                  pbVar6 = pbVar6 + uVar15;
                  bVar13 = 0;
                  param_7 = 0;
                }
              }
              if (bVar13 != 0) {
                uVar15 = iVar7 - iVar11;
                if ((int)param_7 <= iVar7 - iVar11) {
                  uVar15 = param_7;
                }
                iVar11 = iVar11 + param_7;
                for (; uVar15 != 0; uVar15 = uVar15 - 1) {
                  if (*pbVar6 != param_4) {
                    *(undefined1 *)puVar18 =
                         *(undefined1 *)(*(int *)((int)this + 0x24) + (uint)*pbVar6);
                  }
                  puVar18 = (undefined4 *)((int)puVar18 + 1);
                  pbVar6 = pbVar6 + 1;
                  bVar13 = bVar13 - 1;
                  param_7 = (uint)bVar13;
                }
                puVar18 = (undefined4 *)((int)puVar18 + param_7);
                pbVar6 = pbVar6 + param_7;
              }
            }
            iVar3 = iVar17;
            if ((bVar12 & 1) != 0) {
              pbVar6 = pbVar6 + 1;
            }
          }
          else {
            param_7 = (uint)bVar12;
            if (iVar11 < iVar7) {
              if (iVar11 < param_2) {
                uVar15 = (uint)bVar12;
                if (param_2 - iVar11 < (int)uVar15) {
                  bVar12 = bVar12 + ((char)iVar11 - cVar16);
                  param_7 = (uint)bVar12;
                  puVar18 = (undefined4 *)((int)puVar18 + (param_2 - iVar11));
                  iVar11 = param_2;
                }
                else {
                  iVar11 = iVar11 + uVar15;
                  puVar18 = (undefined4 *)((int)puVar18 + uVar15);
                  bVar12 = 0;
                  param_7 = 0;
                }
              }
              if (bVar12 != 0) {
                if (pbVar6[1] != param_4) {
                  uVar15 = iVar7 - iVar11;
                  if ((int)param_7 <= iVar7 - iVar11) {
                    uVar15 = param_7;
                  }
                  uVar1 = *(undefined1 *)(*(int *)((int)this + 0x24) + (uint)pbVar6[1]);
                  puVar19 = puVar18;
                  for (uVar14 = uVar15 >> 2; uVar14 != 0; uVar14 = uVar14 - 1) {
                    *puVar19 = CONCAT22(CONCAT11(uVar1,uVar1),CONCAT11(uVar1,uVar1));
                    puVar19 = puVar19 + 1;
                  }
                  for (uVar15 = uVar15 & 3; iVar17 = param_8, uVar15 != 0; uVar15 = uVar15 - 1) {
                    *(undefined1 *)puVar19 = uVar1;
                    puVar19 = (undefined4 *)((int)puVar19 + 1);
                  }
                }
                puVar18 = (undefined4 *)((int)puVar18 + param_7);
                iVar11 = iVar11 + param_7;
              }
            }
            pbVar6 = pbVar6 + 2;
            iVar3 = iVar17;
          }
        }
      }
    }
  }
  else {
    iVar8 = param_3 - param_6;
    iVar10 = param_8 + param_3;
    while (iVar8 < param_3) {
      sVar2 = *(short *)pbVar6;
      if (sVar2 == 0) {
        iVar8 = iVar8 + 1;
      }
      else {
        if (sVar2 == 0x100) {
          return 0;
        }
        if (sVar2 == 0x200) {
          iVar8 = iVar8 + (uint)pbVar6[3];
          pbVar6 = pbVar6 + 2;
        }
      }
      pbVar6 = pbVar6 + 2;
    }
    iVar11 = FUN_005fc9e0(param_1,iVar8);
    puVar18 = (undefined4 *)(iVar11 * uVar9 + iVar4 + iVar5);
    iVar11 = iVar4;
    iVar3 = iVar8;
joined_r0x005fecd4:
    iVar17 = iVar3;
    param_8 = iVar8;
    if (iVar10 < iVar17) {
      sVar2 = *(short *)pbVar6;
      if (sVar2 == 0) {
        iVar8 = iVar17 + -1;
        iVar11 = FUN_005fc9e0(param_1,iVar8);
        puVar18 = (undefined4 *)(iVar11 * uVar9 + iVar4 + iVar5);
        pbVar6 = pbVar6 + 2;
        iVar11 = iVar4;
        iVar3 = iVar8;
      }
      else {
        iVar8 = iVar10;
        iVar3 = iVar10;
        if (sVar2 != 0x100) {
          if (sVar2 == 0x200) {
            iVar17 = iVar17 - (uint)pbVar6[3];
            iVar11 = iVar11 + (uint)pbVar6[2];
            iVar8 = FUN_005fc9e0(param_1,iVar17);
            puVar18 = (undefined4 *)(iVar8 * uVar9 + iVar11 + iVar5);
            pbVar6 = pbVar6 + 4;
            param_8 = iVar17;
          }
          bVar12 = *pbVar6;
          iVar8 = param_8;
          if (bVar12 == 0) {
            bVar12 = pbVar6[1];
            pbVar6 = pbVar6 + 2;
            param_7 = (uint)bVar12;
            if (iVar11 < iVar7) {
              bVar13 = bVar12;
              if (iVar11 < param_2) {
                uVar15 = (uint)bVar12;
                if (param_2 - iVar11 < (int)uVar15) {
                  puVar18 = (undefined4 *)((int)puVar18 + (param_2 - iVar11));
                  bVar13 = (bVar12 + (char)iVar11) - cVar16;
                  param_7 = (uint)bVar13;
                  pbVar6 = pbVar6 + (param_2 - iVar11);
                  iVar11 = param_2;
                }
                else {
                  iVar11 = iVar11 + uVar15;
                  puVar18 = (undefined4 *)((int)puVar18 + uVar15);
                  pbVar6 = pbVar6 + uVar15;
                  bVar13 = 0;
                  param_7 = 0;
                }
              }
              if (bVar13 != 0) {
                uVar15 = iVar7 - iVar11;
                if ((int)param_7 <= iVar7 - iVar11) {
                  uVar15 = param_7;
                }
                iVar11 = iVar11 + param_7;
                for (; uVar15 != 0; uVar15 = uVar15 - 1) {
                  if (*pbVar6 != param_4) {
                    *(undefined1 *)puVar18 =
                         *(undefined1 *)(*(int *)((int)this + 0x24) + (uint)*pbVar6);
                  }
                  puVar18 = (undefined4 *)((int)puVar18 + 1);
                  pbVar6 = pbVar6 + 1;
                  bVar13 = bVar13 - 1;
                  param_7 = (uint)bVar13;
                }
                puVar18 = (undefined4 *)((int)puVar18 + param_7);
                pbVar6 = pbVar6 + param_7;
              }
            }
            iVar3 = iVar17;
            if ((bVar12 & 1) == 0) goto joined_r0x005fecd4;
          }
          else {
            pbVar6 = pbVar6 + 1;
            param_7 = (uint)bVar12;
            if (iVar11 < iVar7) {
              if (iVar11 < param_2) {
                uVar15 = (uint)bVar12;
                if (param_2 - iVar11 < (int)uVar15) {
                  bVar12 = bVar12 + ((char)iVar11 - cVar16);
                  uVar15 = param_2 - iVar11;
                  param_7 = (uint)bVar12;
                  iVar11 = param_2;
                }
                else {
                  iVar11 = iVar11 + uVar15;
                  bVar12 = 0;
                  param_7 = 0;
                }
                puVar18 = (undefined4 *)((int)puVar18 + uVar15);
                iVar17 = param_8;
              }
              if (bVar12 != 0) {
                if (*pbVar6 != param_4) {
                  uVar15 = iVar7 - iVar11;
                  if ((int)param_7 <= iVar7 - iVar11) {
                    uVar15 = param_7;
                  }
                  uVar1 = *(undefined1 *)(*(int *)((int)this + 0x24) + (uint)*pbVar6);
                  puVar19 = puVar18;
                  for (uVar14 = uVar15 >> 2; uVar14 != 0; uVar14 = uVar14 - 1) {
                    *puVar19 = CONCAT22(CONCAT11(uVar1,uVar1),CONCAT11(uVar1,uVar1));
                    puVar19 = puVar19 + 1;
                  }
                  for (uVar15 = uVar15 & 3; iVar17 = param_8, uVar15 != 0; uVar15 = uVar15 - 1) {
                    *(undefined1 *)puVar19 = uVar1;
                    puVar19 = (undefined4 *)((int)puVar19 + 1);
                  }
                }
                puVar18 = (undefined4 *)((int)puVar18 + param_7);
                pbVar6 = pbVar6 + param_7 + 1;
                iVar11 = iVar11 + param_7;
                iVar3 = iVar17;
                goto joined_r0x005fecd4;
              }
            }
          }
          pbVar6 = pbVar6 + 1;
          iVar3 = iVar17;
        }
      }
      goto joined_r0x005fecd4;
    }
  }
  FUN_005fd170(param_1);
  return 1;
}

