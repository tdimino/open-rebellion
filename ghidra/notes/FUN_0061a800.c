
char * __cdecl FUN_0061a800(int param_1,byte *param_2)

{
  char cVar1;
  byte bVar2;
  bool bVar3;
  char *pcVar4;
  byte *pbVar5;
  int iVar6;
  uint uVar7;
  int iVar8;
  undefined **ppuVar9;
  byte *pbVar10;
  size_t sVar11;
  char *pcVar12;
  undefined4 *puVar13;
  bool bVar14;
  int local_90;
  int local_88;
  byte local_84 [132];
  
  iVar8 = 0;
  if ((param_1 < 0) || (5 < param_1)) {
    return (char *)0x0;
  }
  FUN_0061d9d0(0x13);
  DAT_006bfe44 = DAT_006bfe44 + 1;
  while (lpAddend_006bfe48 != (LONG *)0x0) {
    Sleep(1);
  }
  if (param_1 != 0) {
    if (param_2 == (byte *)0x0) {
      pcVar4 = (&DAT_006acb1c)[param_1 * 3];
    }
    else {
      pcVar4 = (char *)FUN_0061aad0(param_1,param_2);
    }
    goto LAB_0061aa99;
  }
  bVar3 = true;
  local_90 = 0;
  if (param_2 != (byte *)0x0) {
    if (((*param_2 == 0x4c) && (param_2[1] == 0x43)) && (param_2[2] == 0x5f)) {
      pcVar4 = _strpbrk((char *)param_2,"=;");
      while (((pcVar4 != (char *)0x0 && (sVar11 = (int)pcVar4 - (int)param_2, sVar11 != 0)) &&
             (*pcVar4 != ';'))) {
        local_88 = 1;
        ppuVar9 = &PTR_s_LC_COLLATE_006acb24;
        do {
          iVar8 = _strncmp(*ppuVar9,(char *)param_2,sVar11);
          if (iVar8 == 0) {
            uVar7 = 0xffffffff;
            pcVar12 = *ppuVar9;
            do {
              if (uVar7 == 0) break;
              uVar7 = uVar7 - 1;
              cVar1 = *pcVar12;
              pcVar12 = pcVar12 + 1;
            } while (cVar1 != '\0');
            if (sVar11 == ~uVar7 - 1) break;
          }
          ppuVar9 = ppuVar9 + 3;
          local_88 = local_88 + 1;
        } while ((int)ppuVar9 < 0x6acb55);
        pcVar4 = pcVar4 + 1;
        sVar11 = _strcspn(pcVar4,&DAT_006a8f48);
        if ((sVar11 == 0) && (*pcVar4 != ';')) break;
        if (local_88 < 6) {
          _strncpy((char *)local_84,pcVar4,sVar11);
          local_84[sVar11] = 0;
          iVar8 = FUN_0061aad0(local_88,local_84);
          if (iVar8 != 0) {
            local_90 = local_90 + 1;
          }
        }
        if ((pcVar4[sVar11] == '\0') ||
           (param_2 = (byte *)(pcVar4 + sVar11 + 1), pcVar4[sVar11 + 1] == '\0')) goto LAB_0061aa86;
        pcVar4 = _strpbrk((char *)param_2,"=;");
      }
      FUN_0061da50(0x13);
      DAT_006bfe44 = DAT_006bfe44 + -1;
      return (char *)0x0;
    }
    pcVar4 = FUN_0061ad50(param_2,(char *)local_84,(undefined4 *)0x0,(uint *)0x0);
    if (pcVar4 == (char *)0x0) goto LAB_0061aa99;
    puVar13 = &DAT_006acb1c;
    do {
      if ((undefined **)puVar13 != &DAT_006acb1c) {
        pbVar10 = (byte *)*puVar13;
        pbVar5 = local_84;
        do {
          bVar2 = *pbVar5;
          bVar14 = bVar2 < *pbVar10;
          if (bVar2 != *pbVar10) {
LAB_0061aa2b:
            iVar6 = (1 - (uint)bVar14) - (uint)(bVar14 != 0);
            goto LAB_0061aa30;
          }
          if (bVar2 == 0) break;
          bVar2 = pbVar5[1];
          bVar14 = bVar2 < pbVar10[1];
          if (bVar2 != pbVar10[1]) goto LAB_0061aa2b;
          pbVar5 = pbVar5 + 2;
          pbVar10 = pbVar10 + 2;
        } while (bVar2 != 0);
        iVar6 = 0;
LAB_0061aa30:
        if ((iVar6 == 0) || (iVar6 = FUN_0061aad0(iVar8,local_84), iVar6 != 0)) {
          local_90 = local_90 + 1;
        }
        else {
          bVar3 = false;
        }
      }
      puVar13 = puVar13 + 3;
      iVar8 = iVar8 + 1;
    } while ((int)puVar13 < 0x6acb59);
    if (bVar3) {
      pcVar4 = FUN_0061ac30();
      FUN_00618e60(DAT_006acb1c);
      DAT_006acb1c = (undefined *)0x0;
      goto LAB_0061aa99;
    }
LAB_0061aa86:
    if (local_90 == 0) {
      pcVar4 = (char *)0x0;
      goto LAB_0061aa99;
    }
  }
  pcVar4 = FUN_0061ac30();
LAB_0061aa99:
  FUN_0061da50(0x13);
  DAT_006bfe44 = DAT_006bfe44 + -1;
  return pcVar4;
}

