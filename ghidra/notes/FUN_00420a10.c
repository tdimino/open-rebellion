
undefined4 __fastcall FUN_00420a10(int param_1)

{
  int iVar1;
  
  iVar1 = *(int *)(param_1 + -0x58) / *(int *)(param_1 + 0x34);
  if (iVar1 != 0) {
    do {
      FUN_004201d0((void *)(param_1 + -0x94),0);
      iVar1 = iVar1 + -1;
    } while (iVar1 != 0);
  }
  InvalidateRect(*(HWND *)(param_1 + -0x7c),(RECT *)0x0,0);
  return 1;
}

