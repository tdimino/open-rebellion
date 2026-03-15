
void __fastcall FUN_006036a0(int param_1)

{
  int iVar1;
  HWND hWnd;
  SHORT SVar2;
  WPARAM wParam;
  LPARAM lParam;
  
  iVar1 = *(int *)(param_1 + 0x108);
  *(int *)(param_1 + 0x108) = iVar1 + 1;
  if (*(int *)(param_1 + 0x104) < iVar1) {
    SVar2 = GetAsyncKeyState(1);
    GetCapture();
    if (((int)SVar2 & 0x10000000U) != 0) {
      lParam = 0;
      hWnd = *(HWND *)(*(int *)(param_1 + 0x20) + 0x18);
      wParam = FUN_00602510(param_1);
      PostMessageA(hWnd,0x111,wParam,lParam);
      return;
    }
    PostMessageA(*(HWND *)(param_1 + 0x18),0x111,*(WPARAM *)(param_1 + 0x24),0);
  }
  return;
}

