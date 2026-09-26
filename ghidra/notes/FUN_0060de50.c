
undefined4 __fastcall FUN_0060de50(void *param_1)

{
  undefined4 *puVar1;
  uint uVar2;
  HDC hdc;
  HPALETTE hPal;
  
  uVar2 = FUN_0060dba0((int)param_1);
  if (uVar2 == 0) {
    return 0;
  }
  hdc = GetDC(*(HWND *)(*(int *)((int)param_1 + 4) + 0x18));
  hPal = SelectPalette(hdc,DAT_006be5a0,0);
  FUN_0060e050(param_1,hdc);
  SelectPalette(hdc,hPal,0);
  ReleaseDC(*(HWND *)(*(int *)((int)param_1 + 4) + 0x18),hdc);
  SetWindowLongA(*(HWND *)(*(int *)((int)param_1 + 4) + 0x18),-0x10,*(LONG *)((int)param_1 + 0x20));
  puVar1 = *(undefined4 **)((int)param_1 + 8);
  if (puVar1 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar1);
    FUN_00618b60((undefined *)puVar1);
  }
  puVar1 = *(undefined4 **)((int)param_1 + 0xc);
  *(undefined4 *)((int)param_1 + 8) = 0;
  if (puVar1 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar1);
    FUN_00618b60((undefined *)puVar1);
  }
  *(undefined4 *)((int)param_1 + 0xc) = 0;
  *(uint *)((int)param_1 + 0x24) = *(uint *)((int)param_1 + 0x24) & 0xfffffffe;
  return 1;
}

