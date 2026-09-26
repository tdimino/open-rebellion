
void FUN_00626be0(void)

{
  byte bVar1;
  byte bVar2;
  byte *pbVar3;
  DWORD DVar4;
  int iVar5;
  uint uVar6;
  uint uVar7;
  byte *pbVar8;
  byte *pbVar9;
  bool bVar10;
  
  FUN_0061d9d0(0xc);
  DAT_006bec40 = 0;
  DAT_006b0258 = 0xffffffff;
  DAT_006b0248 = 0xffffffff;
  pbVar3 = (byte *)FUN_00628550(&DAT_0066f450);
  if (pbVar3 == (byte *)0x0) {
    FUN_0061da50(0xc);
    DVar4 = GetTimeZoneInformation((LPTIME_ZONE_INFORMATION)&lpTimeZoneInformation_006bec48);
    if (DVar4 == 0xffffffff) {
      return;
    }
    DAT_006bec40 = 1;
    DAT_006b01b0 = (int)lpTimeZoneInformation_006bec48 * 0x3c;
    if (DAT_006bec8e != 0) {
      DAT_006b01b0 = DAT_006b01b0 + DAT_006bec9c * 0x3c;
    }
    if ((DAT_006bece2 == 0) || (DAT_006becf0 == 0)) {
      DAT_006b01b4 = 0;
      DAT_006b01b8 = 0;
    }
    else {
      DAT_006b01b4 = 1;
      DAT_006b01b8 = (DAT_006becf0 - DAT_006bec9c) * 0x3c;
    }
    FUN_006282a0(PTR_DAT_006b0240,(LPCWSTR)&DAT_006bec4c,0x40);
    FUN_006282a0(PTR_DAT_006b0244,(LPCWSTR)&DAT_006beca0,0x40);
    PTR_DAT_006b0244[0x3f] = 0;
    PTR_DAT_006b0240[0x3f] = 0;
    return;
  }
  if (*pbVar3 != 0) {
    pbVar8 = pbVar3;
    pbVar9 = DAT_006becf4;
    if (DAT_006becf4 != (byte *)0x0) {
      do {
        bVar1 = *pbVar8;
        bVar10 = bVar1 < *pbVar9;
        if (bVar1 != *pbVar9) {
LAB_00626d37:
          iVar5 = (1 - (uint)bVar10) - (uint)(bVar10 != 0);
          goto LAB_00626d3c;
        }
        if (bVar1 == 0) break;
        bVar1 = pbVar8[1];
        bVar10 = bVar1 < pbVar9[1];
        if (bVar1 != pbVar9[1]) goto LAB_00626d37;
        pbVar8 = pbVar8 + 2;
        pbVar9 = pbVar9 + 2;
      } while (bVar1 != 0);
      iVar5 = 0;
LAB_00626d3c:
      if (iVar5 == 0) goto LAB_00626ea9;
    }
    FUN_00618e60(DAT_006becf4);
    uVar6 = 0xffffffff;
    pbVar8 = pbVar3;
    do {
      if (uVar6 == 0) break;
      uVar6 = uVar6 - 1;
      bVar1 = *pbVar8;
      pbVar8 = pbVar8 + 1;
    } while (bVar1 != 0);
    DAT_006becf4 = (byte *)FUN_00619170(~uVar6);
    if (DAT_006becf4 != (byte *)0x0) {
      uVar6 = 0xffffffff;
      pbVar8 = pbVar3;
      do {
        pbVar9 = pbVar8;
        if (uVar6 == 0) break;
        uVar6 = uVar6 - 1;
        pbVar9 = pbVar8 + 1;
        bVar1 = *pbVar8;
        pbVar8 = pbVar9;
      } while (bVar1 != 0);
      uVar6 = ~uVar6;
      pbVar8 = pbVar9 + -uVar6;
      pbVar9 = DAT_006becf4;
      for (uVar7 = uVar6 >> 2; uVar7 != 0; uVar7 = uVar7 - 1) {
        *(undefined4 *)pbVar9 = *(undefined4 *)pbVar8;
        pbVar8 = pbVar8 + 4;
        pbVar9 = pbVar9 + 4;
      }
      for (uVar6 = uVar6 & 3; uVar6 != 0; uVar6 = uVar6 - 1) {
        *pbVar9 = *pbVar8;
        pbVar8 = pbVar8 + 1;
        pbVar9 = pbVar9 + 1;
      }
      FUN_0061da50(0xc);
      _strncpy(PTR_DAT_006b0240,(char *)pbVar3,3);
      pbVar8 = pbVar3 + 3;
      PTR_DAT_006b0240[3] = 0;
      bVar1 = *pbVar8;
      if (bVar1 == 0x2d) {
        pbVar8 = pbVar3 + 4;
      }
      iVar5 = FUN_00619560(pbVar8);
      DAT_006b01b0 = iVar5 * 0xe10;
      for (; (bVar2 = *pbVar8, bVar2 == 0x2b || (('/' < (char)bVar2 && ((char)bVar2 < ':'))));
          pbVar8 = pbVar8 + 1) {
      }
      if (*pbVar8 == 0x3a) {
        pbVar8 = pbVar8 + 1;
        iVar5 = FUN_00619560(pbVar8);
        DAT_006b01b0 = DAT_006b01b0 + iVar5 * 0x3c;
        bVar2 = *pbVar8;
        while (('/' < (char)bVar2 && ((char)bVar2 < ':'))) {
          pbVar3 = pbVar8 + 1;
          pbVar8 = pbVar8 + 1;
          bVar2 = *pbVar3;
        }
        if (*pbVar8 == 0x3a) {
          pbVar8 = pbVar8 + 1;
          iVar5 = FUN_00619560(pbVar8);
          DAT_006b01b0 = DAT_006b01b0 + iVar5;
          bVar2 = *pbVar8;
          while (('/' < (char)bVar2 && ((char)bVar2 < ':'))) {
            pbVar3 = pbVar8 + 1;
            pbVar8 = pbVar8 + 1;
            bVar2 = *pbVar3;
          }
        }
      }
      if (bVar1 == 0x2d) {
        DAT_006b01b0 = -DAT_006b01b0;
      }
      DAT_006b01b4 = (int)(char)*pbVar8;
      if (DAT_006b01b4 == 0) {
        *PTR_DAT_006b0244 = 0;
        return;
      }
      _strncpy(PTR_DAT_006b0244,(char *)pbVar8,3);
      PTR_DAT_006b0244[3] = 0;
      return;
    }
  }
LAB_00626ea9:
  FUN_0061da50(0xc);
  return;
}

