
UINT FUN_00600010(HDC param_1,HPALETTE param_2)

{
  int iVar1;
  uint uVar2;
  int iVar3;
  BYTE *pBVar4;
  HPALETTE hpal;
  BOOL BVar5;
  UINT UVar6;
  int iVar7;
  LOGPALETTE LStack_808;
  tagPALETTEENTRY atStack_800 [256];
  tagPALETTEENTRY atStack_400 [256];
  
  UVar6 = 0xffffffff;
  iVar1 = GetDeviceCaps(param_1,0x18);
  iVar1 = iVar1 / 2;
  iVar7 = 0x100 - iVar1;
  if ((DAT_006be59c != (HPALETTE)0x0) && (param_2 != DAT_006be5a0)) {
    FUN_00600230(param_1);
  }
  uVar2 = GetDeviceCaps(param_1,0x26);
  if ((uVar2 & 0x100) == 0) {
    DAT_006be5a0 = param_2;
    return 0;
  }
  if (param_2 != (HPALETTE)0x0) {
    SetSystemPaletteUse(param_1,2);
    SetSystemPaletteUse(param_1,1);
    GetPaletteEntries(param_2,0,0x100,atStack_800);
    GetSystemPaletteEntries(param_1,0,0x100,atStack_400);
    iVar3 = 0;
    if (0 < iVar1) {
      pBVar4 = &atStack_800[0].peFlags;
      do {
        atStack_800[iVar3] = atStack_400[iVar3];
        *pBVar4 = '\0';
        iVar3 = iVar3 + 1;
        pBVar4 = pBVar4 + 4;
      } while (iVar3 < iVar1);
    }
    if (iVar7 < 0x100) {
      iVar3 = iVar7 * 4;
      do {
        *(undefined4 *)(&atStack_800[0].peRed + iVar3) =
             *(undefined4 *)(&atStack_400[0].peRed + iVar3);
        (&atStack_800[0].peFlags)[iVar3] = '\0';
        iVar3 = iVar3 + 4;
      } while (iVar3 < 0x400);
    }
    if (iVar1 < iVar7) {
      pBVar4 = &atStack_800[iVar1].peFlags;
      iVar7 = iVar7 - iVar1;
      do {
        *pBVar4 = '\x04';
        pBVar4 = pBVar4 + 4;
        iVar7 = iVar7 + -1;
      } while (iVar7 != 0);
    }
    LStack_808.palVersion = 0x300;
    LStack_808.palNumEntries = 1;
    hpal = CreatePalette(&LStack_808);
    if (hpal != (HPALETTE)0x0) {
      BVar5 = ResizePalette(hpal,0x100);
      if (BVar5 != 0) {
        SetPaletteEntries(hpal,0,0x100,atStack_800);
        DeleteObject(param_2);
        param_2 = hpal;
      }
    }
    DAT_006be5a0 = param_2;
    if (DAT_006be59c != (HPALETTE)0x0) {
      DeleteObject(DAT_006be59c);
    }
    DAT_006be59c = (HPALETTE)0x0;
    DAT_006be59c = SelectPalette(param_1,DAT_006be5a0,0);
    UVar6 = RealizePalette(param_1);
    DAT_006be4c0 = param_2;
  }
  return UVar6;
}

