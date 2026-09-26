
undefined4 __cdecl FUN_006289f0(byte *param_1,int param_2)

{
  byte bVar1;
  int iVar2;
  int *piVar3;
  byte *pbVar4;
  LPCWSTR pWVar5;
  LPWSTR pWVar6;
  int iVar7;
  int *piVar8;
  uint uVar9;
  uint uVar10;
  byte *pbVar11;
  byte *pbVar12;
  bool bVar13;
  
  if (param_1 == (byte *)0x0) {
    return 0xffffffff;
  }
  pbVar4 = FUN_00628cf0(param_1,0x3d);
  if (pbVar4 == (byte *)0x0) {
    return 0xffffffff;
  }
  if (param_1 == pbVar4) {
    return 0xffffffff;
  }
  bVar13 = pbVar4[1] == 0;
  if (DAT_006be7a4 == DAT_006be7a8) {
    DAT_006be7a4 = FUN_00628c80(DAT_006be7a4);
  }
  if (DAT_006be7a4 == (int *)0x0) {
    if ((param_2 == 0) || (DAT_006be7ac == (undefined4 *)0x0)) {
      if (bVar13) {
        return 0;
      }
      DAT_006be7a4 = (int *)FUN_00619170(4);
      if (DAT_006be7a4 == (int *)0x0) {
        return 0xffffffff;
      }
      *DAT_006be7a4 = 0;
      if (DAT_006be7ac == (undefined4 *)0x0) {
        DAT_006be7ac = (undefined4 *)FUN_00619170(4);
        if (DAT_006be7ac == (undefined4 *)0x0) {
          return 0xffffffff;
        }
        *DAT_006be7ac = 0;
      }
    }
    else {
      pWVar5 = FUN_00628970();
      if (pWVar5 != (LPCWSTR)0x0) {
        return 0xffffffff;
      }
    }
  }
  piVar8 = DAT_006be7a4;
  pWVar6 = (LPWSTR)(pbVar4 + -(int)param_1);
  iVar7 = FUN_00628c00(param_1,pWVar6);
  if ((iVar7 < 0) || (*piVar8 == 0)) {
    if (bVar13) {
      return 0;
    }
    if (iVar7 < 0) {
      iVar7 = -iVar7;
    }
    piVar8 = FUN_0061d740(piVar8,iVar7 * 4 + 8);
    if (piVar8 == (int *)0x0) {
      return 0xffffffff;
    }
    piVar8[iVar7] = (int)param_1;
    piVar8[iVar7 + 1] = 0;
    DAT_006be7a4 = piVar8;
  }
  else if (bVar13) {
    FUN_00618e60((undefined *)piVar8[iVar7]);
    iVar2 = piVar8[iVar7];
    piVar3 = piVar8 + iVar7;
    while (iVar2 != 0) {
      *piVar3 = piVar3[1];
      iVar7 = iVar7 + 1;
      iVar2 = piVar3[1];
      piVar3 = piVar3 + 1;
    }
    piVar8 = FUN_0061d740(piVar8,iVar7 * 4);
    if (piVar8 != (int *)0x0) {
      DAT_006be7a4 = piVar8;
    }
  }
  else {
    piVar8[iVar7] = (int)param_1;
  }
  if (param_2 != 0) {
    uVar9 = 0xffffffff;
    pbVar4 = param_1;
    do {
      if (uVar9 == 0) break;
      uVar9 = uVar9 - 1;
      bVar1 = *pbVar4;
      pbVar4 = pbVar4 + 1;
    } while (bVar1 != 0);
    pbVar4 = (byte *)FUN_00619170(~uVar9 + 1);
    if (pbVar4 != (byte *)0x0) {
      uVar9 = 0xffffffff;
      do {
        pbVar11 = param_1;
        if (uVar9 == 0) break;
        uVar9 = uVar9 - 1;
        pbVar11 = param_1 + 1;
        bVar1 = *param_1;
        param_1 = pbVar11;
      } while (bVar1 != 0);
      uVar9 = ~uVar9;
      pbVar11 = pbVar11 + -uVar9;
      pbVar12 = pbVar4;
      for (uVar10 = uVar9 >> 2; uVar10 != 0; uVar10 = uVar10 - 1) {
        *(undefined4 *)pbVar12 = *(undefined4 *)pbVar11;
        pbVar11 = pbVar11 + 4;
        pbVar12 = pbVar12 + 4;
      }
      for (uVar9 = uVar9 & 3; uVar9 != 0; uVar9 = uVar9 - 1) {
        *pbVar12 = *pbVar11;
        pbVar11 = pbVar11 + 1;
        pbVar12 = pbVar12 + 1;
      }
      pbVar4[(int)pWVar6] = 0;
      SetEnvironmentVariableA
                ((LPCSTR)pbVar4,(LPCSTR)(~-(uint)bVar13 & (uint)(pbVar4 + 1 + (int)pWVar6)));
      FUN_00618e60(pbVar4);
      return 0;
    }
  }
  return 0;
}

