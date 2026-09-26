
void __fastcall FUN_004163d0(int param_1)

{
  HDC hdc;
  uint uVar1;
  HPALETTE hpal;
  int iVar2;
  int iVar3;
  BOOL BVar4;
  HPALETTE hPal;
  BYTE *pBVar5;
  LOGPALETTE LStack_408;
  tagPALETTEENTRY atStack_400 [256];
  
  hdc = GetDC(*(HWND *)(param_1 + 0x18));
  if (hdc != (HDC)0x0) {
    uVar1 = GetDeviceCaps(hdc,0x26);
    if ((uVar1 & 0x100) != 0) {
      LStack_408.palVersion = 0x300;
      LStack_408.palNumEntries = 1;
      hpal = CreatePalette(&LStack_408);
      iVar2 = GetDeviceCaps(hdc,0x18);
      iVar3 = GetDeviceCaps(hdc,0x68);
      if (iVar3 == 0x100) {
        SetSystemPaletteUse(hdc,2);
        SetSystemPaletteUse(hdc,1);
        GetSystemPaletteEntries(hdc,0,0x100,atStack_400);
        iVar2 = iVar2 / 2;
        if (0 < iVar2) {
          pBVar5 = &atStack_400[0].peFlags;
          iVar3 = iVar2;
          do {
            *pBVar5 = '\0';
            pBVar5 = pBVar5 + 4;
            iVar3 = iVar3 + -1;
          } while (iVar3 != 0);
        }
        if (iVar2 < 0x100 - iVar2) {
          iVar3 = (0x100 - iVar2) - iVar2;
          pBVar5 = &atStack_400[iVar2].peBlue;
          iVar2 = iVar2 + iVar3;
          do {
            pBVar5[1] = '\0';
            *pBVar5 = '\0';
            pBVar5[-1] = '\0';
            ((tagPALETTEENTRY *)(pBVar5 + -2))->peRed = '\0';
            pBVar5 = pBVar5 + 4;
            iVar3 = iVar3 + -1;
          } while (iVar3 != 0);
        }
        if (iVar2 < 0x100) {
          pBVar5 = &atStack_400[iVar2].peFlags;
          iVar2 = 0x100 - iVar2;
          do {
            *pBVar5 = '\0';
            pBVar5 = pBVar5 + 4;
            iVar2 = iVar2 + -1;
          } while (iVar2 != 0);
        }
        if (hpal != (HPALETTE)0x0) {
          BVar4 = ResizePalette(hpal,0x100);
          if (BVar4 != 0) {
            SetPaletteEntries(hpal,0,0x100,atStack_400);
            hPal = SelectPalette(hdc,hpal,0);
            RealizePalette(hdc);
            SelectPalette(hdc,hPal,0);
          }
          DeleteObject(hpal);
        }
      }
    }
    ReleaseDC(*(HWND *)(param_1 + 0x18),hdc);
  }
  return;
}

