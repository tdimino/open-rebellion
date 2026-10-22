
void __fastcall FUN_00603780(void *param_1)

{
  if (*(int *)((int)param_1 + 0xf8) != 0) {
    FUN_006030f0(param_1,1);
    if (*(HWND *)((int)param_1 + 0x18) != (HWND)0x0) {
      InvalidateRect(*(HWND *)((int)param_1 + 0x18),(RECT *)0x0,0);
    }
    *(undefined4 *)((int)param_1 + 0xf8) = 0;
    *(undefined4 *)((int)param_1 + 0x108) = 0;
  }
  return;
}

