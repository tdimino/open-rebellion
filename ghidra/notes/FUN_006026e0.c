
void __fastcall FUN_006026e0(void *param_1)

{
  LPRECT ptVar1;
  HWND hWnd;
  int iVar2;
  int iVar3;
  HDC hdc;
  LPCSTR lpchText;
  int iVar4;
  int iVar5;
  int iVar6;
  LPRECT lprc;
  UINT format;
  
  hWnd = *(HWND *)((int)param_1 + 0x18);
  iVar2 = *(int *)((int)param_1 + 0x38);
  iVar4 = *(int *)((int)param_1 + 0x3c);
  if (*(int *)((int)param_1 + 200) != 0) {
    iVar3 = FUN_00583c40(*(int *)((int)param_1 + 200));
    if (iVar3 != 0) {
      hdc = GetDC(hWnd);
      FUN_006002c0(param_1,hdc);
      SetBkMode(hdc,1);
      SetTextAlign(hdc,0);
      iVar3 = *(int *)((int)param_1 + 200);
      ptVar1 = (LPRECT)((int)param_1 + 0xcc);
      format = 0x401;
      iVar6 = -1;
      ptVar1->left = 0;
      *(int *)((int)param_1 + 0xd4) = iVar2;
      *(undefined4 *)((int)param_1 + 0xd0) = 0;
      *(undefined4 *)((int)param_1 + 0xd8) = 0;
      lprc = ptVar1;
      lpchText = (LPCSTR)FUN_00583c40(iVar3);
      iVar3 = DrawTextA(hdc,lpchText,iVar6,lprc,format);
      *(int *)((int)param_1 + 0xd8) = iVar3;
      FUN_006002f0(param_1,hdc);
      iVar3 = *(int *)((int)param_1 + 0xd4);
      iVar6 = *(int *)((int)param_1 + 0xd8);
      iVar5 = (iVar2 - iVar3) / 2;
      iVar2 = *(int *)((int)param_1 + 0xd8);
      ptVar1->left = iVar5;
      iVar4 = (iVar4 - iVar6) / 2;
      *(int *)((int)param_1 + 0xd4) = iVar5 + iVar3;
      *(int *)((int)param_1 + 0xd0) = iVar4;
      *(int *)((int)param_1 + 0xd8) = iVar4 + iVar2;
      ReleaseDC(hWnd,hdc);
    }
  }
  return;
}

