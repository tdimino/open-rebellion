
void __fastcall FUN_0059afb0(int param_1)

{
  HWND hWnd;
  HDC hdc;
  int iVar1;
  int *piVar2;
  HPALETTE hpal;
  undefined1 *puVar3;
  undefined1 *puVar4;
  int iVar5;
  tagPALETTEENTRY atStack_400 [10];
  undefined1 auStack_3d8 [984];
  
  hWnd = *(HWND *)(param_1 + 0x18);
  hdc = GetDC(hWnd);
  iVar1 = FUN_005c14b0();
  piVar2 = (int *)FUN_005ff440(param_1);
  hpal = FUN_005fc3f0(piVar2);
  GetSystemPaletteEntries(hdc,0,0x100,atStack_400);
  if (iVar1 != 0) {
    puVar4 = auStack_3d8 + 1;
    puVar3 = (undefined1 *)(iVar1 + 0x2a);
    iVar5 = 0xec;
    do {
      puVar3[(int)atStack_400 + (1 - iVar1)] = 4;
      puVar4[-1] = puVar3[-2];
      *puVar4 = puVar3[-1];
      puVar3[(int)atStack_400 - iVar1] = *puVar3;
      puVar4 = puVar4 + 4;
      puVar3 = puVar3 + 4;
      iVar5 = iVar5 + -1;
    } while (iVar5 != 0);
  }
  if (hpal != (HPALETTE)0x0) {
    SetPaletteEntries(hpal,0,0x100,atStack_400);
  }
  FUN_00600010(hdc,hpal);
  ReleaseDC(hWnd,hdc);
  return;
}

