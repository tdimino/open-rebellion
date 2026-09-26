
bool __thiscall FUN_005ff6b0(void *this,UINT param_1)

{
  char cVar1;
  HWND hWnd;
  HDC hdc;
  HDC hdc_00;
  HGDIOBJ pvVar2;
  HGDIOBJ pvVar3;
  HPALETTE pHVar4;
  int iVar5;
  CHAR *pCVar6;
  char *pcVar7;
  bool bVar8;
  uint in_stack_00000018;
  COLORREF in_stack_0000001c;
  UINT in_stack_00000020;
  CHAR local_100 [256];
  
  pCVar6 = local_100;
  for (iVar5 = 0x3f; iVar5 != 0; iVar5 = iVar5 + -1) {
    pCVar6[0] = '\0';
    pCVar6[1] = '\0';
    pCVar6[2] = '\0';
    pCVar6[3] = '\0';
    pCVar6 = pCVar6 + 4;
  }
  pCVar6[0] = '\0';
  pCVar6[1] = '\0';
  pCVar6[2] = '\0';
  bVar8 = false;
  FUN_00601060(param_1,local_100,0xff);
  iVar5 = -1;
  pcVar7 = local_100;
  do {
    if (iVar5 == 0) break;
    iVar5 = iVar5 + -1;
    cVar1 = *pcVar7;
    pcVar7 = pcVar7 + 1;
  } while (cVar1 != '\0');
  if ((iVar5 != -2) &&
     ((iVar5 = *(int *)((int)this + 0x98), iVar5 != 0 ||
      (iVar5 = *(int *)((int)this + 0x94), iVar5 != 0)))) {
    hWnd = *(HWND *)((int)this + 0x18);
    hdc = GetDC(hWnd);
    hdc_00 = CreateCompatibleDC(hdc);
    pvVar2 = (HGDIOBJ)FUN_006002b0(in_stack_00000018);
    pHVar4 = DAT_006be5a0;
    bVar8 = hdc_00 != (HDC)0x0;
    if (bVar8) {
      pvVar3 = (HGDIOBJ)FUN_005fc130(iVar5);
      pvVar3 = SelectObject(hdc_00,pvVar3);
      pHVar4 = SelectPalette(hdc_00,pHVar4,0);
      pvVar2 = SelectObject(hdc_00,pvVar2);
      SetTextColor(hdc_00,in_stack_0000001c);
      SetBkMode(hdc_00,1);
      DrawTextA(hdc_00,local_100,-1,(LPRECT)&stack0x00000008,in_stack_00000020);
      SelectPalette(hdc_00,pHVar4,0);
      SelectObject(hdc_00,pvVar2);
      SelectObject(hdc_00,pvVar3);
      DeleteDC(hdc_00);
    }
    ReleaseDC(hWnd,hdc);
  }
  return bVar8;
}

