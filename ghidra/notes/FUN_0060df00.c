
undefined4 __thiscall FUN_0060df00(void *param_1,int *param_2)

{
  uint uVar1;
  HDC hdc;
  HPALETTE hPal;
  int aiStack_8 [2];
  
  uVar1 = FUN_0060dba0((int)param_1);
  if (uVar1 == 0) {
    return 0;
  }
  hdc = GetDC(*(HWND *)(*(int *)((int)param_1 + 4) + 0x18));
  hPal = SelectPalette(hdc,DAT_006be5a0,0);
  FUN_0060e050(param_1,hdc);
  FUN_0060e120(param_1,aiStack_8,param_2);
  FUN_0060e0a0(param_1,hdc,aiStack_8);
  SelectPalette(hdc,hPal,0);
  ReleaseDC(*(HWND *)(*(int *)((int)param_1 + 4) + 0x18),hdc);
  return 1;
}

