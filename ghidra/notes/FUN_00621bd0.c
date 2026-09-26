
void FUN_00621bd0(void)

{
  BOOL BVar1;
  uint uVar2;
  uint uVar3;
  uint uVar4;
  BYTE *pBVar5;
  ushort *puVar6;
  CHAR *pCVar7;
  _cpinfo local_514;
  CHAR aCStack_500 [256];
  WCHAR aWStack_400 [128];
  WCHAR aWStack_300 [128];
  WORD aWStack_200 [256];
  
  BVar1 = GetCPInfo(DAT_006beb78,&local_514);
  if (BVar1 == 1) {
    uVar2 = 0;
    do {
      aCStack_500[uVar2] = (CHAR)uVar2;
      uVar2 = uVar2 + 1;
    } while (uVar2 < 0x100);
    aCStack_500[0] = ' ';
    if (local_514.LeadByte[0] != 0) {
      pBVar5 = local_514.LeadByte + 1;
      do {
        uVar2 = (uint)local_514.LeadByte[0];
        if (uVar2 <= *pBVar5) {
          uVar3 = (*pBVar5 - uVar2) + 1;
          uVar4 = uVar3 >> 2;
          pCVar7 = aCStack_500 + uVar2;
          while (uVar4 != 0) {
            uVar4 = uVar4 - 1;
            builtin_memcpy(pCVar7,"    ",4);
            pCVar7 = pCVar7 + 4;
          }
          for (uVar3 = uVar3 & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
            *pCVar7 = ' ';
            pCVar7 = pCVar7 + 1;
          }
        }
        local_514.LeadByte[0] = pBVar5[1];
        pBVar5 = pBVar5 + 2;
      } while (local_514.LeadByte[0] != 0);
    }
    FUN_00624e80(1,aCStack_500,0x100,aWStack_200,DAT_006beb78,DAT_006beb7c,0);
    FUN_00620ea0(DAT_006beb7c,0x100,aCStack_500,(LPCWSTR)&DAT_00000100,aWStack_400,0x100,
                 DAT_006beb78,0);
    FUN_00620ea0(DAT_006beb7c,0x200,aCStack_500,(LPCWSTR)&DAT_00000100,aWStack_300,0x100,
                 DAT_006beb78,0);
    uVar2 = 0;
    puVar6 = aWStack_200;
    do {
      if ((*puVar6 & 1) == 0) {
        if ((*puVar6 & 2) == 0) {
          (&DAT_006bea78)[uVar2] = 0;
        }
        else {
          (&DAT_006be971)[uVar2] = (&DAT_006be971)[uVar2] | 0x20;
          (&DAT_006bea78)[uVar2] = *(undefined1 *)((int)aWStack_300 + uVar2);
        }
      }
      else {
        (&DAT_006be971)[uVar2] = (&DAT_006be971)[uVar2] | 0x10;
        (&DAT_006bea78)[uVar2] = *(undefined1 *)((int)aWStack_400 + uVar2);
      }
      uVar2 = uVar2 + 1;
      puVar6 = puVar6 + 1;
    } while (uVar2 < 0x100);
    return;
  }
  uVar2 = 0;
  do {
    if ((uVar2 < 0x41) || (0x5a < uVar2)) {
      if ((uVar2 < 0x61) || (0x7a < uVar2)) {
        (&DAT_006bea78)[uVar2] = 0;
      }
      else {
        (&DAT_006be971)[uVar2] = (&DAT_006be971)[uVar2] | 0x20;
        (&DAT_006bea78)[uVar2] = (char)uVar2 + -0x20;
      }
    }
    else {
      (&DAT_006be971)[uVar2] = (&DAT_006be971)[uVar2] | 0x10;
      (&DAT_006bea78)[uVar2] = (char)uVar2 + ' ';
    }
    uVar2 = uVar2 + 1;
  } while (uVar2 < 0x100);
  return;
}

