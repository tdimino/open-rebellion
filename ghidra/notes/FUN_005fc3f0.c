
HPALETTE __fastcall FUN_005fc3f0(int *param_1)

{
  int iVar1;
  undefined1 *puVar2;
  undefined1 *puVar3;
  HPALETTE hpal;
  BOOL BVar4;
  PALETTEENTRY *pPVar5;
  int iVar6;
  int iVar7;
  LOGPALETTE local_408;
  PALETTEENTRY local_400 [256];
  
  iVar1 = *param_1;
  if (iVar1 == 0) {
    return (HPALETTE)0x0;
  }
  iVar6 = iVar1 + 0x28;
  pPVar5 = local_400;
  iVar7 = 0x100;
  puVar2 = (undefined1 *)(iVar1 + 0x29);
  do {
    puVar3 = puVar2 + 4;
    pPVar5->peRed = puVar2[1];
    puVar3[(int)local_400 + (-4 - iVar6)] = *puVar2;
    puVar3[(int)local_400 + (-3 - iVar6)] = puVar2[-1];
    puVar3[(int)local_400 + (-2 - iVar6)] = 0;
    pPVar5 = pPVar5 + 1;
    iVar7 = iVar7 + -1;
    puVar2 = puVar3;
  } while (iVar7 != 0);
  local_408.palVersion = 0x300;
  local_408.palNumEntries = 1;
  hpal = CreatePalette(&local_408);
  if (hpal != (HPALETTE)0x0) {
    BVar4 = ResizePalette(hpal,0x100);
    if (BVar4 != 0) {
      SetPaletteEntries(hpal,0,0x100,local_400);
    }
  }
  return hpal;
}

