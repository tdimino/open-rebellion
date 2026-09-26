
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined4 __cdecl FUN_006218b0(int param_1)

{
  BYTE *pBVar1;
  byte bVar2;
  byte bVar3;
  UINT CodePage;
  UINT *pUVar4;
  BOOL BVar5;
  uint uVar6;
  uint uVar7;
  int iVar8;
  int iVar9;
  BYTE *pBVar10;
  byte *pbVar11;
  byte *pbVar12;
  undefined4 *puVar13;
  _cpinfo local_14;
  
  FUN_0061d9d0(0x19);
  CodePage = FUN_00621ae0(param_1);
  if (CodePage == DAT_006beb78) {
    FUN_0061da50(0x19);
    return 0;
  }
  if (CodePage == 0) {
    FUN_00621b90();
    FUN_00621bd0();
    FUN_0061da50(0x19);
    return 0;
  }
  iVar9 = 0;
  pUVar4 = &DAT_006af620;
  do {
    if (*pUVar4 == CodePage) {
      puVar13 = (undefined4 *)&DAT_006be970;
      for (iVar8 = 0x40; iVar8 != 0; iVar8 = iVar8 + -1) {
        *puVar13 = 0;
        puVar13 = puVar13 + 1;
      }
      *(undefined1 *)puVar13 = 0;
      uVar6 = 0;
      pbVar11 = &DAT_006af630 + iVar9 * 0x30;
      do {
        bVar2 = *pbVar11;
        for (pbVar12 = pbVar11; (bVar2 != 0 && (bVar2 = pbVar12[1], bVar2 != 0));
            pbVar12 = pbVar12 + 2) {
          uVar7 = (uint)*pbVar12;
          if (uVar7 <= bVar2) {
            bVar3 = (&DAT_006af618)[uVar6];
            do {
              (&DAT_006be971)[uVar7] = (&DAT_006be971)[uVar7] | bVar3;
              uVar7 = uVar7 + 1;
            } while (uVar7 <= bVar2);
          }
          bVar2 = pbVar12[2];
        }
        uVar6 = uVar6 + 1;
        pbVar11 = pbVar11 + 8;
      } while (uVar6 < 4);
      _DAT_006bed10 = 1;
      DAT_006beb78 = CodePage;
      DAT_006beb7c = FUN_00621b30(CodePage);
      _DAT_006beb80 = (&DAT_006af624)[iVar9 * 0xc];
      _DAT_006beb84 = (&DAT_006af628)[iVar9 * 0xc];
      _DAT_006beb88 = (&DAT_006af62c)[iVar9 * 0xc];
      goto LAB_00621a02;
    }
    pUVar4 = pUVar4 + 0xc;
    iVar9 = iVar9 + 1;
  } while (pUVar4 < &DAT_006af710);
  BVar5 = GetCPInfo(CodePage,&local_14);
  if (BVar5 == 1) {
    puVar13 = (undefined4 *)&DAT_006be970;
    for (iVar9 = 0x40; iVar9 != 0; iVar9 = iVar9 + -1) {
      *puVar13 = 0;
      puVar13 = puVar13 + 1;
    }
    *(undefined1 *)puVar13 = 0;
    DAT_006beb7c = 0;
    if (local_14.MaxCharSize < 2) {
      _DAT_006bed10 = 0;
      DAT_006beb78 = CodePage;
    }
    else {
      DAT_006beb78 = CodePage;
      if (local_14.LeadByte[0] != '\0') {
        pBVar10 = local_14.LeadByte + 1;
        do {
          bVar2 = *pBVar10;
          if (bVar2 == 0) break;
          for (uVar6 = (uint)pBVar10[-1]; uVar6 <= bVar2; uVar6 = uVar6 + 1) {
            (&DAT_006be971)[uVar6] = (&DAT_006be971)[uVar6] | 4;
          }
          pBVar1 = pBVar10 + 1;
          pBVar10 = pBVar10 + 2;
        } while (*pBVar1 != 0);
      }
      uVar6 = 1;
      do {
        (&DAT_006be971)[uVar6] = (&DAT_006be971)[uVar6] | 8;
        uVar6 = uVar6 + 1;
      } while (uVar6 < 0xff);
      DAT_006beb7c = FUN_00621b30(CodePage);
      _DAT_006bed10 = 1;
    }
    _DAT_006beb80 = 0;
    _DAT_006beb84 = 0;
    _DAT_006beb88 = 0;
  }
  else {
    if (DAT_006beb8c == 0) {
      FUN_0061da50(0x19);
      return 0xffffffff;
    }
    FUN_00621b90();
  }
LAB_00621a02:
  FUN_00621bd0();
  FUN_0061da50(0x19);
  return 0;
}

